use std::fmt;
use std::io::{Cursor, Read};
use crate::script::script::Script;

#[derive(Clone, Debug)]
pub struct TxOutput {
    amount: u64,
    script_pubkey: Script
}

impl TxOutput {
    pub fn new(amount: u64, script_pubkey: Script) -> Self {
        Self {
            amount,
            script_pubkey
        }
    }

    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Self {
        let mut amount_buffer= [0u8; 8];
        cursor.read_exact(&mut amount_buffer).unwrap();
        let amount: u64 = u64::from_le_bytes(amount_buffer); 
        let script_pubkey = Script::parse(cursor).unwrap();

        Self {
            amount,
            script_pubkey
        }
    }

    /// Serializes the transaction output into a byte vector
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Serialize amount, 8 bytes, little endian
        result.extend_from_slice(&self.amount.to_le_bytes());

        // Serialize the script_pubkey
        result.extend(self.script_pubkey.serialize());

        result
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    pub fn get_script_pubkey(&self) -> Script {
        self.script_pubkey.clone()
    }
}



impl fmt::Display for TxOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "amount: {} satoshis", self.amount)?;
        writeln!(f, "script_pubkey:\n{}", self.script_pubkey)
    }
}
