use std::io::{Cursor, Read};

use crate::{script::btc_script::Script, transactions::tx_fetcher::TxFetcher};

use super::tx::Tx;

#[derive(Clone, Debug)]
pub struct TxInput {
    prev_tx_id: [u8; 32], // should this be prev?
    prev_index: [u8; 4], // should this be prev?
    script_sig: Script,
    sequence: [u8; 4]
}

impl TxInput {
    // do we need this method?
    pub fn new(prev_tx_id: [u8; 32], prev_index: [u8; 4], script_sig: Script, sequence: [u8; 4]) -> Self {
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
        let script_sig = Script::parse(cursor).unwrap();
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
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Serialize prev_tx (no need to reverse)
        result.extend(&self.prev_tx_id);

        // Serialize prev_index in little endian
        let prev_index_le = self.prev_index;
        result.extend(prev_index_le);

        // Serialize script_sig
        result.extend(self.script_sig.serialize());

        // Serialize sequence
        let sequence = self.sequence;
        result.extend(sequence);

        result
    }

    pub fn fetch_tx(&self, testnet: bool, fresh: bool) -> Tx {
        let tx_id_hex = hex::encode(self.prev_tx_id);
        // need a fetcher instance here - may not be the best place for this
        let fetcher = TxFetcher::build();
        TxFetcher::fetch(&fetcher, &tx_id_hex, testnet, fresh).unwrap()
    }

    /// Get the output value by looking up the tx hash. Returns the amount in satoshi.
    pub fn value(&self) -> u64 {
        let tx = &self.fetch_tx(true, true);
        let index = u32::from_le_bytes(self.prev_index) as usize;
        tx.get_tx_outs()[index].get_amount()
    }

    /// Get the ScriptPubKey by looking up the tx hash
    pub fn script_pubkey(&self, testnet: bool) -> Script {
        let tx = &self.fetch_tx(testnet, true);
        let index = u32::from_le_bytes(self.prev_index) as usize;
        tx.get_tx_outs()[index].get_script_pubkey()
    }
}
