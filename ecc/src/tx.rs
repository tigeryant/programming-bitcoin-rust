use crate::hash256::hash256;
use crate::varint::{ read_varint, encode_varint };
use crate::tx_input::TxInput;
use crate::tx_output::TxOutput;
use std::io::{Cursor, Read};

#[derive(Clone)]
pub struct Tx { // use getter methods rather than setting these methods to public
    version: u32,
    tx_ins: Vec<TxInput>,
    pub tx_outs: Vec<TxOutput>,
    locktime: u32,
    testnet: bool
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxInput>, tx_outs: Vec<TxOutput>, locktime: u32, testnet: bool) -> Self {
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
        // Serialize tx but exclude the last byte (testnet flag)
        let serialized = self.serialize();
        let hash_input = &serialized[..serialized.len()-1];
        
        // Get hash256 of serialized tx (excluding testnet flag)
        let hash = hash256(hash_input);
        
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

        result.push(self.testnet as u8);
        
        result
    }

    // TODO finish this parse method
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Self {
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

        // Parse testnet flag (1 byte)
        let mut testnet_buffer = [0u8; 1];
        stream.read_exact(&mut testnet_buffer).unwrap();
        let testnet = testnet_buffer[0] != 0;

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
}
