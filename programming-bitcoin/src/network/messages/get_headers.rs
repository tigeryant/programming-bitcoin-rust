use crate::{
    network::{get_tip_hash::get_tip_hash, network_message::NetworkMessage},
    utils::varint::{encode_varint, read_varint},
};
use std::io::{Cursor, Read, Error};

#[derive(Clone)]
pub struct GetHeadersMessage {
    pub command: String,
    pub version: u32,
    pub num_hashes: u64,
    pub start_block: Vec<u8>,
    pub end_block: Vec<u8>,
}

impl GetHeadersMessage {
    // start and end block given in little endian
    pub fn new(version: u32, num_hashes: u64, start_block: Vec<u8>, end_block: Option<Vec<u8>>) -> Self {
        let command = String::from("getheaders");

        let end_block = end_block.unwrap_or(vec![0; 32]);

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

        result.extend_from_slice(&self.start_block);

        result.extend_from_slice(&self.end_block);

        result
    }

    fn parse(&self, reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let command = String::from("getheaders");

        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let version = u32::from_le_bytes(version);

        let num_hashes = read_varint(reader)?;

        let mut start_block = vec![];
        reader.read_exact(&mut start_block)?;

        let mut end_block = vec![];
        reader.read_exact(&mut end_block)?;

        Ok(Self {
            command,
            version,
            num_hashes,
            start_block,
            end_block,
        })
    }

    async fn default_async(_: &str) -> Result<Self, Error> {
        Ok(Self::new(70015, 1, get_tip_hash().await.unwrap(), None))
    }
}
