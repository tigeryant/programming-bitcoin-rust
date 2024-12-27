use crate::{
    network::{get_block_tip::get_block_tip, network_message::NetworkMessage},
    utils::varint::encode_varint,
};
use std::io::{Cursor, Error};

pub struct GetHeadersMessage {
    pub command: String,
    pub version: u32,
    pub num_hashes: u64,
    pub start_block: u32,
    pub end_block: u32,
}

impl GetHeadersMessage {
    pub fn new(version: u32, num_hashes: u64, start_block: u32, end_block: Option<u32>) -> Self {
        let command = String::from("getheaders");

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

impl NetworkMessage for GetHeadersMessage {
    fn command(&self) -> &str {
        &self.command
    }

    // Serializes an instance of self into a byte vector
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend_from_slice(&self.version.to_le_bytes());

        result.extend_from_slice(&encode_varint(self.num_hashes));

        result.extend_from_slice(&self.start_block.to_le_bytes());

        result.extend_from_slice(&self.end_block.to_le_bytes());

        result
    }

    fn parse(&self, _: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        todo!()
    }

    async fn default_async(_: &str) -> Result<Self, Error> {
        Ok(Self::new(70015, 1, get_block_tip().await.unwrap(), None))
    }
}
