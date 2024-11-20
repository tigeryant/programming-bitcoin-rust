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

    // fetch a tx from the UTXO set or return it from the cache
    pub fn fetch(&self, tx_id: &str, testnet: bool, fresh: bool) -> Result<Tx, Box<dyn std::error::Error>> { // note the use of dynamic dispatch here
        let mut cache = self.cache.borrow_mut();
        if fresh || !cache.contains_key(tx_id) {
            let api_url = Self::get_url(testnet);
            let url = format!("{}/tx/{}/hex", api_url, tx_id);
            let response = reqwest::blocking::get(url)?.text()?;
            let raw = hex::decode(response.trim())?;
            let mut cursor = Cursor::new(raw);
            let tx = Tx::parse(&mut cursor, testnet); // do we need the ? operator?
            // let tx = Tx::parse(&mut cursor)?;

            if tx.id() != tx_id {
                return Err(format!("not the same id: {} vs {}", tx.id(), tx_id).into());
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
