pub struct TxInput {
    prev_tx: Vec<u8>,
    prev_index: Vec<u8>,
    script_sig: Option<String>, // temporarily give this a type of String before we implement that struct
    // script_sig: Script,
    sequence: Vec<u8>,
}

impl TxInput {
    pub fn new(prev_tx: Vec<u8>, prev_index: Vec<u8>, script_sig: Option<String>, sequence: Vec<u8>) -> Self {
        Self {
            prev_tx,
            prev_index,
            script_sig,
            sequence
        }
    }
}