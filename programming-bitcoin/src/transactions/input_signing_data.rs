use crate::utils::sig_hash_type::SigHashType;

use super::tx_input::TxInput;

pub struct InputSigningData {
    pub index: usize,
    pub private_key_str: String,
    pub sig_hash_type: SigHashType,
    pub input: TxInput
}

impl InputSigningData {
    pub fn new(index: usize, private_key_str: String, sig_hash_type: SigHashType, input: TxInput) -> Self {
        Self {
            index,
            private_key_str,
            sig_hash_type,
            input
        }
    }
}