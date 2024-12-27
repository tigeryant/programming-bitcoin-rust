// use crate::network::network_message::NetworkMessage;

pub struct GetHeadersMessage {
    pub command: String,
    pub version: u32,
    pub num_hashes: Vec<u8>,
    pub start_block: u32,
    pub end_block: u32,
}

impl GetHeadersMessage {
    pub fn new(version: u32, num_hashes: u64, start_block: u32, end_block: Option<u32>) -> Self {
        let command = String::from("getheaders");

        let num_hashes = num_hashes.to_le_bytes().to_vec();

        let end_block = end_block.unwrap_or(0);

        Self {
            command,
            version,
            num_hashes,
            start_block,
            end_block,
        }
    }
}

/*
impl NetworkMessage for GetHeadersMessage {
    todo!();
    // command
    // serialize
    // parse
    // default_async
}
*/
