use std::io::{Cursor, Read};

#[derive(Clone)]
pub struct TxInput {
    prev_tx_id: [u8; 32],
    prev_index: [u8; 4],
    script_sig: Option<String>, // temporarily give this a type of String before we implement that struct
    // script_sig: Script,
    sequence: [u8; 4]
}

impl TxInput {
    // do we need this method?
    pub fn new(prev_tx_id: [u8; 32], prev_index: [u8; 4], script_sig: Option<String>, sequence: [u8; 4]) -> Self {
        Self {
            prev_tx_id,
            prev_index,
            script_sig,
            sequence
        }
    }

    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Self {
        // read 32 bytes for prev_tx (little endian)
        let mut prev_tx_id= [0u8; 32];
        cursor.read_exact(&mut prev_tx_id).unwrap();
        // read 4 bytes for the index (little endian)
        let mut prev_index= [0u8; 4];
        cursor.read_exact(&mut prev_index).unwrap();
        // script_sig? - return some placeholder for now
        let script_sig = Some(String::from("script"));
        // read 4 bytes for the sequence
        let mut sequence= [0u8; 4];
        cursor.read_exact(&mut sequence).unwrap();
        Self {
            prev_tx_id,
            prev_index,
            script_sig,
            sequence
        }
    }

    /// Returns the byte serialization of the transaction input
    /// TODO complete later
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Serialize prev_tx (no need to reverse)
        result.extend(&self.prev_tx_id);

        // Serialize prev_index in little endian
        let prev_index_le = self.prev_index;
        result.extend(prev_index_le);

        // Serialize script_sig
        // result.extend(self.script_sig.serialize());
        // PLACEHOLDER
        result.extend([0u8; 4]);

        // Serialize sequence in little endian
        let sequence = self.sequence;
        result.extend(sequence);

        result
    }
}
