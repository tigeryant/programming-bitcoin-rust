use std::{cell::RefCell, collections::HashMap, io::Cursor};
use reqwest;
use crate::transactions::tx::Tx;

pub struct TxFetcher {
    cache: RefCell<HashMap<String, Tx>>,
}

impl TxFetcher {
    pub fn get_url(testnet: bool) -> String {
        if testnet {
            String::from("https://blockstream.info/testnet/api/")
        } else {
            String::from("https://blockstream.info/api/")
        }
    }

    /// Fetches a tx from the UTXO set via an API (or returns it from the cache)
    // expects the tx_id in big endian encoding
    pub fn fetch(&self, tx_id: &str, testnet: bool, fresh: bool) -> Result<Tx, Box<dyn std::error::Error>> {
        let mut cache = self.cache.borrow_mut();
        if fresh || !cache.contains_key(tx_id) {
            let api_url = Self::get_url(testnet);
            let url = format!("{}/tx/{}/hex", api_url, tx_id);
            let response = reqwest::blocking::get(url)?;

            let status = response.status();
            let response_text = response.text()?;
            
            // Check status code and include response text in error message
            if !status.is_success() {
                return Err(format!("HTTP request failed with status: {} - Response: {}", status, response_text).into());
            }

            let raw = hex::decode(response_text.trim())?;
            let mut cursor = Cursor::new(raw);
            let tx = Tx::parse(&mut cursor, testnet);
            
            if tx.id() != tx_id {
                return Err(format!("not the same id: tx.id(): {} vs tx_id: {}", tx.id(), tx_id).into());
            }
            cache.insert(tx_id.to_string(), tx);
        }
        Ok(cache.get(tx_id).unwrap().clone())
    }


    /// Builds the TxFetcher
    pub fn build() -> TxFetcher {
        TxFetcher {
            cache: RefCell::new(HashMap::new())
        }
    }
}
