use std::io::{Cursor, Read};

use crate::varint::read_varint;

pub struct TxOutput {
    amount: u64,
    script_pubkey: Option<String>
}

impl TxOutput {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Self {
        // amount u64
        let mut amount_buffer= [0u8; 8];
        cursor.read_exact(&mut amount_buffer).unwrap();
        // read it as a u64
        let amount: u64 = u64::from_le_bytes(amount_buffer); 

        // ScriptPubKey - variable length encoded, preceded by a varint
        // script_pubkey? - placeholder
        let varint = read_varint(cursor);
        let script_pubkey = Some(String::from("script"));

        Self {
            amount,
            script_pubkey
        }
    }
}