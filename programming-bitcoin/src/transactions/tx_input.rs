use std::io::{Cursor, Error, Read};

use crate::{script::script::Script, transactions::tx_fetcher::TxFetcher};

use super::tx::Tx;

#[derive(Clone)]
pub struct TxInput {
    pub prev_tx_id: [u8; 32], // little endian
    pub prev_index: [u8; 4],
    pub script_sig: Script,
    pub sequence: [u8; 4],
    pub witness: Option<Vec<Vec<u8>>>,
    pub height: Option<u32>,
}

impl TxInput {
    // takes prev_tx_id in big endian, reverses it to little endian
    pub fn new(prev_tx_id_be: [u8; 32], prev_index: [u8; 4], script_sig: Script, sequence: [u8; 4], witness: Option<Vec<Vec<u8>>>, height: Option<u32>) -> Self {
        let mut tx_id_le = prev_tx_id_be;
        tx_id_le.reverse();
        Self {
            prev_tx_id: tx_id_le,
            prev_index,
            script_sig,
            sequence,
            witness,
            height,
        }
    }

    // may contain a witness - None for now
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut prev_tx_id= [0u8; 32];
        cursor.read_exact(&mut prev_tx_id)?;

        let mut prev_index= [0u8; 4];
        cursor.read_exact(&mut prev_index)?;
        
        let (script_sig, height) = Script::parse_script_sig(cursor)?;

        let mut sequence= [0u8; 4];
        cursor.read_exact(&mut sequence)?;

        let witness: Option<Vec<Vec<u8>>> = None;

        Ok(Self {
            prev_tx_id,
            prev_index,
            script_sig,
            sequence,
            witness,
            height,
        })
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

    /// Returns the byte serialization of an input whose script_sig is BIP-34 compliant
    pub fn serialize_bip_34(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Serialize prev_tx (no need to reverse)
        result.extend(&self.prev_tx_id);

        // Serialize prev_index in little endian
        let prev_index_le = self.prev_index;
        result.extend(prev_index_le);

        // Serialize script_sig
        result.extend(self.script_sig.serialize_bip_34());

        // Serialize sequence
        let sequence = self.sequence;
        result.extend(sequence);

        result
    }

    pub fn fetch_tx(&self, testnet: bool, fresh: bool) -> Tx {
        let tx_id_hex = self.get_prev_tx_id_be();
        // need a fetcher instance here - may not be the best place for this
        let fetcher = TxFetcher::build();
        TxFetcher::fetch(&fetcher, &tx_id_hex, testnet, fresh).unwrap()
    }

    /// Get the output value by looking up the tx hash. Returns the amount in satoshi.
    pub fn value(&self, testnet: bool) -> u64 {
        let tx = &self.fetch_tx(testnet, true);
        let index = u32::from_le_bytes(self.prev_index) as usize;
        tx.get_tx_outs()[index].get_amount()
    }

    /// Returns a TxInput whose script_sig field is empty (0), witness is none
    pub fn empty_script_sig(&self) -> Self {
        let empty_commands= vec![vec![0]];
        let empty_script_sig = Script::new(empty_commands);
        Self {
            prev_tx_id: self.prev_tx_id,
            prev_index: self.prev_index,
            script_sig: empty_script_sig,
            sequence: self.sequence,
            witness: self.witness.clone(),
            height: self.height,
        }
    } 

    /// Get the script_pubkey by looking up the tx hash
    pub fn script_pubkey(&self, testnet: bool) -> Script {
        let tx = &self.fetch_tx(testnet, true);
        let index = u32::from_le_bytes(self.prev_index) as usize;
        tx.get_tx_outs()[index].get_script_pubkey()
    }

    /// Returns a modified input (script_sig replaced with script_pubkey/redeem_script) for creating a signature hash
    pub fn replace_script_sig(&self, testnet: bool, redeem_script: Option<Script>) -> Self {
        let replacement: Script = match redeem_script {
            None => self.script_pubkey(testnet),
            Some(redeem_script) => redeem_script
        };
        Self {
            prev_tx_id: self.prev_tx_id,
            prev_index: self.prev_index,
            script_sig: replacement,
            sequence: self.sequence,
            witness: self.witness.clone(),
            height: self.height,
        }
    }

    /// reverses from little endian (stored and serialized) to big (displayed)
    pub fn get_prev_tx_id_be(&self) -> String {
        let mut reversed = self.prev_tx_id;
        reversed.reverse();
        hex::encode(reversed)
    }

    pub fn get_prev_tx_id_le(&self) -> [u8; 32] {
        self.prev_tx_id
    }

    pub fn set_witness(&self, witness: Option<Vec<Vec<u8>>>) -> Self {
        Self {
            witness,
            ..self.clone()
        }
    }

    pub fn witness_length(&self) -> u8 {
        self.witness.as_ref().map_or(0, |w| w.len() as u8)
    }
}


impl std::fmt::Display for TxInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let witness = match &self.witness {
            Some(w) => w.iter()
                .map(hex::encode)
                .collect::<Vec<String>>()
                .join(","),
            None => String::from("None")
        };

        let height_str = match self.height {
            Some(height) => height.to_string(),
            None => String::from("undefined")
        };

        let is_bip_34 = if self.height.is_some() { String::from("true") } else { String::from("false") };

        write!(f, "TxInput {{ \n  prev_tx_id (big endian): {}\n  prev_index: {}\n  script_sig: \n{}  sequence: {}\n  witness: {}\n  height: {}\n  is_bip_34: {}\n}}", 
            self.get_prev_tx_id_be(), // the encoding is not reversing it, and it's being displayed in big endian
            u32::from_le_bytes(self.prev_index),
            self.script_sig,
            hex::encode(self.sequence),
            witness,
            height_str,
            is_bip_34
        )
    }
}

impl std::fmt::Debug for TxInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TxInput")
            .field("prev_tx_id (BE)", &format!("value: {} hex: {}", self.get_prev_tx_id_be(), hex::encode(self.prev_tx_id)))
            .field("prev_index", &format!("value: {} hex: {}", u32::from_le_bytes(self.prev_index), hex::encode(self.prev_index)))
            .field("script_sig", &format!("value: {} hex: {}", self.script_sig, hex::encode(self.script_sig.serialize())))
            .field("sequence", &format!("value: {} hex: {}", u32::from_le_bytes(self.sequence), hex::encode(self.sequence)))
            .field("witness", &format!("value: {:?} hex: {}", self.witness, self.witness.as_ref().map_or("None".to_string(), |w| w.iter().map(hex::encode).collect::<Vec<String>>().join(","))))
            .field("height", &self.height)
            .field("is_bip_34", &self.height.is_some())
            .finish()
    }
}
