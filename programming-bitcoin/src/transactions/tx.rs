use std::io::{Cursor, Read};
use std::fmt;
use crate::utils::hash256::hash256;
use crate::utils::varint::{ read_varint, encode_varint };
use crate::transactions::tx_input::TxInput;
use crate::transactions::tx_output::TxOutput;
use crate::utils::sig_hash_type::SigHashType;

#[derive(Clone, Debug)]
pub struct Tx {
    version: u32,
    tx_ins: Vec<TxInput>,
    tx_outs: Vec<TxOutput>,
    locktime: u32,
    testnet: bool
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxInput>, tx_outs: Vec<TxOutput>, locktime: u32, testnet: bool) -> Self { // is this necessary?
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet
        }
    }

    pub fn id(&self) -> String {
        // Convert transaction hash to hex string
        hex::encode(self.hash())
    }
    
    fn hash(&self) -> Vec<u8> {
        let serialized = self.serialize();
        let hash = hash256(&serialized);
        // Reverse to get little endian
        hash.into_iter().rev().collect()
    }

    // Serialize method
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Serialize version
        result.extend_from_slice(&self.version.to_le_bytes());
        
        // Serialize tx_ins
        let inputs = &self.tx_ins;
        result.extend_from_slice(&encode_varint(inputs.len() as u64));

        for input in inputs {
            result.extend(input.serialize());
        }
        
        // Serialize tx_outs
        let outputs = self.tx_outs.clone();
        result.extend_from_slice(&encode_varint(outputs.len() as u64));
        for output in outputs {
            result.extend(output.serialize());
        }
        
        // Serialize locktime (4 bytes, little endian)
        result.extend_from_slice(&self.locktime.to_le_bytes());

        // We omit the testnet field - this is not part of the serialized tx
        
        result
    }

    pub fn parse(stream: &mut Cursor<Vec<u8>>, testnet: bool) -> Self {
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).unwrap();
        let version = u32::from_le_bytes(buffer);

        // Parse inputs
        // Add proper error handling
        let input_count = read_varint(stream).unwrap();
        
        let tx_ins: Vec<TxInput> = (0..input_count)
            .map(|_| {
                TxInput::parse(stream)
            })
            .collect();

        // Parse outputs
        // Add proper error handling
        let output_count = read_varint(stream).unwrap();
        
        let tx_outs: Vec<TxOutput> = (0..output_count)
            .map(|_| {
                TxOutput::parse(stream)
            })
            .collect();

        // Parse the locktime
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).unwrap();
        let locktime = u32::from_le_bytes(buffer);

        // Parse testnet flag (1 byte) - can we parse this if it's not actually included?
        // let mut testnet_buffer = [0u8; 1];
        // stream.read_exact(&mut testnet_buffer).unwrap();
        // let testnet = testnet_buffer[0] != 0;
        // let testnet = true;

        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet 
        }
    }

    pub fn fee(&self) -> u64 {
        let input_total: u64 = self.tx_ins
            .iter()
            .map(|input| input.value())
            .sum();

        let output_total: u64 = self.tx_outs
            .iter()
            .map(|output| output.get_amount())
            .sum();

        input_total - output_total
    }

    pub fn get_tx_outs(&self) -> Vec<TxOutput> {
        self.tx_outs.clone()
    }

    /// Returns the signature hash
    pub fn sig_hash(&self, sig_hash_type: SigHashType, tx_index: usize) -> Vec<u8> {
        let inputs = &self.tx_ins;
        let mut modified_inputs: Vec<TxInput> = inputs
            .iter()
            .map(|input| {
                input.empty_script_sig()
            })
            .collect();
        // can we remove this clone()?
        let mut current_input = modified_inputs[tx_index].clone();
        // Can we deduce the sig_hash_type based on the last byte of the script_sig?
        current_input.get_script_sig();
        current_input = current_input.replace_script_sig(self.testnet);
        modified_inputs[tx_index] = current_input;
        let modified_tx = Self {
            version: self.version,
            tx_ins: modified_inputs,
            tx_outs: self.get_tx_outs(),
            locktime: self.locktime,
            testnet: true
        };
        let mut serialized_tx = modified_tx.serialize();
        let sighash = match sig_hash_type {
            SigHashType::SigHashAll => 1u32.to_le_bytes(),
            SigHashType::SigHashNone => 2u32.to_le_bytes(),
            SigHashType::SigHashSingle => 3u32.to_le_bytes(),
        };
        dbg!(&sighash);
        serialized_tx.extend_from_slice(&sighash);
        hash256(&serialized_tx)
    }

    pub fn verify_input(&self, sig_hash_type: SigHashType, index: usize) -> bool {
        let z = self.sig_hash(sig_hash_type, index);
        let input: &TxInput = &self.tx_ins[index];
        let script_sig = input.get_script_sig();
        let script_pubkey = input.script_pubkey(self.testnet);
        let combined_script = script_sig.concat(script_pubkey);
        combined_script.evaluate(z)
    }

    /// Verify the transaction
    pub fn verify(&self) -> bool {
        // fee() will always be positive as it returns u64

        for (index, _) in self.tx_ins.iter().enumerate() {
            if !self.verify_input(SigHashType::SigHashAll, index) {
                return false
            }
        }
        true
    }
}

impl fmt::Display for Tx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "tx: {}", self.id())?;
        writeln!(f, "version: {}", self.version)?;
        writeln!(f, "tx_ins:")?;
        for (i, input) in self.tx_ins.iter().enumerate() {
            writeln!(f, "\t{}: {}", i, input)?;
        }
        writeln!(f, "tx_outs:")?;
        for (i, output) in self.tx_outs.iter().enumerate() {
            writeln!(f, "\t{}: {}", i, hex::encode(output.serialize()))?;
        }
        writeln!(f, "locktime: {}", self.locktime)
    }
}
