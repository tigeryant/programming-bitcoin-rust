use crate::hash256::hash256;
use crate::varint::{ read_varint, encode_varint };
use crate::tx_input::TxInput;
use std::io::{Cursor, Read};

pub struct Tx {
    version: u32,
    tx_ins: Vec<TxInput>,
    tx_outs: Vec<u32>,
    locktime: u32,
    testnet: bool
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxInput>, tx_outs: Vec<u32>, locktime: u32, testnet: bool) -> Self {
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
        // Get hash256 of serialized tx
        let hash = hash256(&self.serialize());
        // Reverse to get little endian
        hash.into_iter().rev().collect()
    }

    // Edit this later
    // And a serialize method:
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Serialize version (4 bytes, little endian)
        result.extend_from_slice(&self.version.to_le_bytes());
        
        // Serialize tx_ins
        let inputs = &self.tx_ins;
        result.extend_from_slice(&encode_varint(inputs.len() as u64));

        for input in inputs {
            result.extend(input.serialize());
        }
        
        // Serialize tx_outs
        result.extend(self.tx_outs.iter().flat_map(|tx_out| tx_out.to_le_bytes()));
        
        // Serialize locktime (4 bytes, little endian)
        result.extend_from_slice(&self.locktime.to_le_bytes());
        
        result
    }

    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Self {
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).unwrap();
        let version = u32::from_le_bytes(buffer);

        let varint = read_varint(stream);
        // depending on the result, (assuming it's Ok(u64)), we know how many inputs we need to parse
        
        // for now, let's assume it's a u64 with a value of 1 - that is, there is a single input
        let input_count: u64 = 1;
        let tx_ins: Vec<TxInput> = (0..input_count)
            .map(|_| {
                // for each element in the range, return a TxInput by calling TxInput::parse()
                TxInput::parse(stream)
            })
            .collect();

        todo!();
        // Self {
        //     version,
        //     tx_ins
        // }
    }

}