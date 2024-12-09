use std::io::{Cursor, Read};
use std::fmt;
use crate::script::script::Script;
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
    testnet: bool,
    segwit: bool
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxInput>, tx_outs: Vec<TxOutput>, locktime: u32, testnet: bool, segwit: bool) -> Self { // is this necessary?
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
            segwit
        }
    }

    pub fn id(&self) -> String {
        // Convert transaction hash to hex string
        hex::encode(self.hash())
    }
    
    fn hash(&self) -> Vec<u8> {
        let serialized = self.serialize_legacy();
        let hash = hash256(&serialized);
        // Reverse to get little endian
        hash.into_iter().rev().collect()
    }

    // Serialize method
    pub fn serialize(&self) -> Vec<u8> {
        if self.segwit {
            self.serialize_segwit()
        } else {
            self.serialize_legacy()
        }
    }

    fn serialize_legacy(&self) -> Vec<u8> {
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

    fn serialize_segwit(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Serialize version
        result.extend_from_slice(&self.version.to_le_bytes());

        // Serialize the marker byte and flag
        let marker_bytes: [u8; 2] = [0x00, 0x01];
        result.extend_from_slice(&marker_bytes);

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

        // iterate over the tx_ins
        self.tx_ins.clone()
            .into_iter()
            .for_each(|input| {
                let length = input.witness_length();
                result.extend_from_slice(&[length]);

                let witness = input.get_witness().unwrap();
                for item in witness {
                    if item.len() == 1 {
                        result.extend_from_slice(&item);
                    } else {
                        result.extend_from_slice(&encode_varint(item.len() as u64));
                        result.extend_from_slice(&item);
                    }
                }
            });

        // Serialize locktime (4 bytes, little endian)
        result.extend_from_slice(&self.locktime.to_le_bytes());

        result
    }

    pub fn parse(stream: &mut Cursor<Vec<u8>>, testnet: bool) -> Self {
        stream.set_position(4);
        let mut marker_byte = [0u8; 1];
        stream.read_exact(&mut marker_byte).unwrap();
        let marker = marker_byte[0];
        stream.set_position(0);
        // consider using traits to use different methods here
        if marker == 0x00 { // must be a segwit tx
            Self::parse_segwit(stream, testnet)
        } else {
            Self::parse_legacy(stream, testnet)
        }
    }

    /// Parses legacy (pre-segwit) transactions
    fn parse_legacy(stream: &mut Cursor<Vec<u8>>, testnet: bool) -> Self {
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
        let segwit = false;

        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
            segwit
        }
    }

    fn parse_segwit(stream: &mut Cursor<Vec<u8>>, testnet: bool) -> Self {
        // first, read the version
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).unwrap();
        let version = u32::from_le_bytes(buffer);

        // read the next two bytes
        let mut marker_bytes = [0u8; 2];
        stream.read_exact(&mut marker_bytes).unwrap();
        if marker_bytes != [0x00, 0x01] {
            panic!("Not a segwit transaction - marker bytes: {:?}", marker_bytes);
        }

        // Parse inputs
        // Add proper error handling
        let input_count = read_varint(stream).unwrap();
        
        let mut tx_ins: Vec<TxInput> = (0..input_count)
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

        // Parse the witness data for each input
        // change this to use map
        tx_ins = tx_ins
            .into_iter()
            .map(|input| {
                let witness_count = read_varint(stream).unwrap();
                let mut items = vec![];
                for _ in 0..witness_count {
                    let length = read_varint(stream).unwrap() as usize;
                    if length == 0 {
                        items.push(vec![0]);
                    } else {
                        let mut witness_item = vec![0u8; length];
                        stream.read_exact(&mut witness_item).unwrap();
                        items.push(witness_item);
                    }
                    input.set_witness(Some(items.clone()));
                }
                input
            })
            .collect();

        // Parse the locktime
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).unwrap();
        let locktime = u32::from_le_bytes(buffer);

        let segwit = true;

        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
            segwit
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
            testnet: self.testnet,
            segwit: self.segwit
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
        // z will be calculated differently for a segwit tx
        let input: &TxInput = &self.tx_ins[index];
        let script_pubkey = input.script_pubkey(self.testnet);
        // later we need to deal with p2sh-p2wphk (the wrapped version)
        // for now, deal with p2wpkh
        let z: Vec<u8>;
        let witness;
        if script_pubkey.is_p2wpkh() {
            z = self.sig_hash_bip143(index, None, None);
            witness = input.clone().get_witness();
        } else { // legacy tx
            z = self.sig_hash(sig_hash_type, index);
            witness = None;
        }

        let script_sig = input.get_script_sig();
        let combined_script = script_sig.concat(script_pubkey);
        combined_script.evaluate(z, witness) // update this method after
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

    /// Returns a byte vector of the signature hash to be signed for the input at this index
    fn sig_hash_bip143(&self, input_index: usize, redeem_script: Option<Script>, witness_script: Option<Script>) -> Vec<u8> {
        // per BIP143 spec
        let tx_in = self.tx_ins[input_index].clone();

        let mut result = vec![];
        result.extend_from_slice(&self.version.to_le_bytes());

        result.extend_from_slice(&self.hash_prevouts());
        result.extend_from_slice(&self.hash_sequence());
        result.extend_from_slice(&tx_in.get_prev_tx_id_le()); // Previous tx id in little-endian
        result.extend_from_slice(&tx_in.get_prev_index()); // 4-byte little-endian index

        // let script_code: Vec<u8>;
        // if witness_script.is_some() {
            // TODO implement
            // script_code = witness_script.serialize()
        // } else if redeem_script.is_some() {
            // TODO implement
            // script_code = p2pkh_script(redeem_script.cmds[1]).serialize()
        // } else {
        let script_code = Script::p2pkh_script(tx_in.script_pubkey(self.testnet).get_commands()[1].clone()).serialize();
        // }
        result.extend_from_slice(&script_code);

        // Add tx_in value in little endian (8 bytes)
        result.extend_from_slice(&tx_in.value().to_le_bytes());

        // Add tx_in sequence in little endian (4 bytes)
        result.extend_from_slice(&tx_in.get_sequence());

        // Add hash of all outputs
        result.extend_from_slice(&self.hash_outputs());

        // Add locktime in little endian (4 bytes)
        result.extend_from_slice(&self.locktime.to_le_bytes());

        // Add SIGHASH_ALL in little endian (4 bytes)
        result.extend_from_slice(&1u32.to_le_bytes());

        // Hash the result and return
        hash256(&result)
    }
    
    fn hash_prevouts(&self) -> Vec<u8> {
        let mut all_prevouts = Vec::new();
        let mut all_sequence = Vec::new();
        
        for tx_in in &self.tx_ins {
            // Add prev_tx in little endian and prev_index
            all_prevouts.extend(tx_in.get_prev_tx_id_le());
            all_prevouts.extend(tx_in.get_prev_index());
            // Add sequence in little endian
            all_sequence.extend_from_slice(&tx_in.get_sequence());
        }
        
        hash256(&all_prevouts)
    }
    
    fn hash_sequence(&self) -> Vec<u8> {
        let mut all_sequence = Vec::new();
        
        for tx_in in &self.tx_ins {
            all_sequence.extend_from_slice(&tx_in.get_sequence());
        }
        
        hash256(&all_sequence)
    }
    
    fn hash_outputs(&self) -> Vec<u8> {
        let mut all_outputs = Vec::new();
        
        for tx_out in &self.tx_outs {
            all_outputs.extend(tx_out.serialize());
        }
        
        hash256(&all_outputs)
    }

}

impl fmt::Display for Tx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "tx_id: {}", self.id())?;
        writeln!(f, "version: {}", self.version)?;
        writeln!(f, "tx_ins:")?;
        for (i, input) in self.tx_ins.iter().enumerate() {
            writeln!(f, "\t{}: {}", i, input)?;
        }
        writeln!(f, "tx_outs:")?;
        for (i, output) in self.tx_outs.iter().enumerate() {
            writeln!(f, "\t{}: {}", i, output)?;
        }
        writeln!(f, "locktime: {}", self.locktime)
    }
}
