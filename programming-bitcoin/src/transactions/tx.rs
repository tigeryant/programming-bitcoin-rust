use std::io::{Cursor, Read};
use std::fmt;
use primitive_types::U256;

use crate::ecc::private_key::PrivateKey;
use crate::script::script::Script;
use crate::utils::hash256::hash256;
use crate::utils::varint::{ read_varint, encode_varint };
use crate::transactions::tx_input::TxInput;
use crate::transactions::tx_output::TxOutput;
use crate::utils::sig_hash_type::SigHashType;

use super::input_signing_data::InputSigningData;

#[derive(Clone)]
pub struct Tx {
    pub version: u32,
    pub tx_ins: Vec<TxInput>,
    pub tx_outs: Vec<TxOutput>,
    pub locktime: u32,
    pub testnet: bool,
    pub segwit: bool
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxInput>, tx_outs: Vec<TxOutput>, locktime: u32, testnet: bool, segwit: bool) -> Self {
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
        
        // Serialize tx_ins length as varint
        let inputs = &self.tx_ins;
        result.extend_from_slice(&encode_varint(inputs.len() as u64));

        // Serialize tx_ins, using different logic for coinbase & BIP-34 transactions
        if self.is_coinbase() && self.is_bip_34() {
            result.extend_from_slice(&inputs[0].serialize_bip_34());
        } else {
            for input in inputs {
                result.extend_from_slice(&input.serialize());
            }
        }
        
        // Serialize tx_outs
        let outputs = self.tx_outs.clone();
        result.extend_from_slice(&encode_varint(outputs.len() as u64));
        for output in outputs {
            result.extend(output.serialize());
        }
        
        // Serialize locktime (4 bytes, little endian)
        result.extend_from_slice(&self.locktime.to_le_bytes());

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

                let witness = input.witness.unwrap();
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
        let current_pos = stream.position();
        stream.set_position(current_pos + 4);
        let mut marker_byte = [0u8; 1];
        stream.read_exact(&mut marker_byte).unwrap();
        let marker = marker_byte[0];
        stream.set_position(current_pos);
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
                TxInput::parse(stream).unwrap()
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
                TxInput::parse(stream).unwrap()
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
                }
                let input = input.set_witness(Some(items.clone()));
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
            .map(|input| input.value(self.testnet))
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
    pub fn sig_hash(&self, sig_hash_type: &SigHashType, tx_index: usize, p2sh: bool) -> Vec<u8> {
        let inputs = &self.tx_ins;
        let original_input = inputs[tx_index].clone();
        let original_script_sig = original_input.script_sig.clone();
        original_input.empty_script_sig();
        let mut redeem_script: Option<Script> = None;
        // extract the redeem script if P2SH
        if p2sh {
            redeem_script = Some(original_script_sig.get_redeem_script());
        }
        let mut modified_inputs: Vec<TxInput> = inputs
            .iter()
            .map(|input| {
                input.empty_script_sig()
            })
            .collect();
        // can we remove this clone()?
        let mut current_input = modified_inputs[tx_index].clone();
        // TODO Can we deduce the sig_hash_type based on the last byte of the script_sig?
        // let script_sig = current_input.get_script_sig();
        current_input = current_input.replace_script_sig(self.testnet, redeem_script);
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
        serialized_tx.extend_from_slice(&sighash);
        hash256(&serialized_tx)
    }

    pub fn verify_input(&self, sig_hash_type: SigHashType, index: usize) -> bool {
        let input: &TxInput = &self.tx_ins[index];
        let script_pubkey = input.script_pubkey(self.testnet);
        // z calculated differently for a segwit tx
        let z: Vec<u8>;
        let witness;
        // could refactor to a match statement
        if script_pubkey.is_p2sh_script_pubkey() {
            let command = input.script_sig.commands[input.script_sig.commands.len() - 1].clone();
            let mut raw_redeem = vec![];
            raw_redeem.extend_from_slice(&[command.len() as u8]);
            raw_redeem.extend_from_slice(&command);
            let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_redeem);
            let redeem_script = Script::parse(&mut stream).unwrap();
            if redeem_script.is_p2wpkh_script_pubkey() {
                z = self.sig_hash_bip143(index, Some(redeem_script), None);
                witness = input.clone().witness;
            } else if redeem_script.is_p2wsh_script_pubkey() {
                let input_witness = input.witness.clone().unwrap();
                let command = &input_witness[input_witness.len() - 1];
                let mut raw_witness = encode_varint(command.len() as u64);
                raw_witness.extend_from_slice(command);
                let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_witness);
                let witness_script = Script::parse(&mut stream).unwrap();
                z = self.sig_hash_bip143(index, None, Some(witness_script));
                witness = Some(input_witness);
            } else {
                z = self.sig_hash(&sig_hash_type, index,  true);
                witness = None;
            }
        } else if script_pubkey.is_p2wpkh_script_pubkey() {
            z = self.sig_hash_bip143(index, None, None);
            witness = input.clone().witness;
        } else if script_pubkey.is_p2wsh_script_pubkey() {
            let input_witness = input.witness.clone().unwrap();
            let command = &input_witness[input_witness.len() - 1];
            let mut raw_witness = encode_varint(command.len() as u64);
            raw_witness.extend_from_slice(command);
            let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_witness);
            let witness_script = Script::parse(&mut stream).unwrap();
            z = self.sig_hash_bip143(index, None, Some(witness_script));
            witness = Some(input_witness);
        } else { // legacy tx
            z = self.sig_hash(&sig_hash_type, index, false);
            let z_string = hex::encode(&z);
            dbg!(z_string);
            witness = None;
        }

        let script_sig = input.script_sig.clone();
        let combined_script = script_sig.concat(script_pubkey);
        combined_script.evaluate(z, witness)
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
        result.extend_from_slice(&tx_in.prev_index); // 4-byte little-endian index

        let script_code: Vec<u8>;
        if witness_script.is_some() {
            script_code = witness_script.unwrap().serialize();
        } else if redeem_script.is_some() {
            script_code = Script::p2pkh_script(redeem_script.unwrap().commands[1].clone()).serialize()
        } else {
            script_code = Script::p2pkh_script(tx_in.script_pubkey(self.testnet).commands[1].clone()).serialize();
        }
        result.extend_from_slice(&script_code);

        // Add tx_in value in little endian (8 bytes)
        result.extend_from_slice(&tx_in.value(self.testnet).to_le_bytes());

        // Add tx_in sequence in little endian (4 bytes)
        result.extend_from_slice(&tx_in.sequence);

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
            all_prevouts.extend(tx_in.prev_index);
            // Add sequence in little endian
            all_sequence.extend_from_slice(&tx_in.sequence);
        }
        
        hash256(&all_prevouts)
    }
    
    fn hash_sequence(&self) -> Vec<u8> {
        let mut all_sequence = Vec::new();
        
        for tx_in in &self.tx_ins {
            all_sequence.extend_from_slice(&tx_in.sequence);
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

    pub fn sign_input(&self, index: usize, private_key_str: &str, sig_hash_type: SigHashType, unsigned_input: TxInput) -> TxInput {
        // signing the tx - getting z
        let z = self.sig_hash(&sig_hash_type, index, false); // assumes not p2sh
        // Private key associated with the public key of the output we are spending from
        let private_key = PrivateKey::new(U256::from_str_radix(private_key_str, 16).unwrap());
        let der = private_key.sign(z).der();

        // Signature concatenated with the sig hash type as 1 byte
        let sig = [der, vec![sig_hash_type as u8]].concat();
        let sec = private_key.point().sec(true); // assuming compressed is true
        // script sig for p2pkh is the signature, the sig hash and the pub key
        let script_sig = Script::new(vec![sig, sec]);
        let mut prev_tx_id = [0u8; 32];
        prev_tx_id.copy_from_slice(&hex::decode(unsigned_input.get_prev_tx_id_be()).unwrap());
        let prev_index = unsigned_input.prev_index;
        let sequence = unsigned_input.sequence;
        let witness = unsigned_input.witness;
        let height = unsigned_input.height;
        // return a new signed input
        TxInput::new(prev_tx_id, prev_index, script_sig, sequence, witness, height)
    }

    pub fn sign_multiple_inputs(&self, input_signing_data: Vec<InputSigningData>) -> Vec<TxInput> {
        input_signing_data
            .into_iter()
            .map(|data| {
                self.sign_input(data.index, &data.private_key_str, data.sig_hash_type, data.input)
            })
            .collect()
    }

    pub fn is_coinbase(&self) -> bool {
        let zero_string = "0000000000000000000000000000000000000000000000000000000000000000";
        let zero_prev_tx_id: [u8; 32] = hex::decode(zero_string).unwrap().try_into().unwrap();

        let f_string = "ffffffff";
        let f_index: [u8; 4] = hex::decode(f_string).unwrap().try_into().unwrap();

        self.tx_ins.len() == 1 &&
        self.tx_ins[0].prev_index == f_index &&
        self.tx_ins[0].prev_tx_id == zero_prev_tx_id
    }

    pub fn coinbase_height(&self) -> u32 {
        let script_sig = &self.tx_ins[0].script_sig;
        let block_height: &Vec<u8> = &script_sig.commands[0];
        u32::from_le_bytes(block_height[..4].try_into().unwrap())
    }

    pub fn is_bip_34(&self) -> bool {
        self.tx_ins[0].height.is_some()
    }
}

impl fmt::Display for Tx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "tx_id: {}", self.id())?;
        writeln!(f, "version: {}", self.version)?;
        writeln!(f, "tx_ins:")?;
        for (i, input) in self.tx_ins.iter().enumerate() {
            writeln!(f, "{}: {}", i, input)?;
        }
        writeln!(f, "tx_outs:")?;
        for (i, output) in self.tx_outs.iter().enumerate() {
            writeln!(f, "{}: {}", i, output)?;
        }
        writeln!(f, "locktime: {}", self.locktime)
    }
}

impl fmt::Debug for Tx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tx")
            .field("version", &format!("{:08x}", self.version))
            .field("tx_ins", &self.tx_ins.iter().map(|input| format!("{:?}", input)).collect::<Vec<_>>())
            .field("tx_outs", &self.tx_outs.iter().map(|output| format!("{:?}", output)).collect::<Vec<_>>())
            .field("locktime", &format!("{:08x}", self.locktime))
            .field("testnet", &self.testnet)
            .field("segwit", &self.segwit)
            .finish()
    }
}
