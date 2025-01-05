use std::io::{Cursor, Error};

use crate::{blocks::block::Block, network::network_message::NetworkMessage };

#[derive(Clone)]
pub struct BlockMessage {
    pub command: String,
    pub block: Block,
}

impl BlockMessage {
    pub fn new(block: Block) -> Self {
        let command = String::from("block");

        Self {
            command,
            block,
        }
    }
}

impl NetworkMessage for BlockMessage {
    fn command(&self) -> &str {
        &self.command
    }

    // Serializes an instance of self into a byte vector
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        let block = &self.block.serialize();
        result.extend_from_slice(block);

        result
    }

    fn parse(&self, reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let command = String::from("block");

        let block = Block::parse(reader).unwrap();

        Ok(Self {
            command,
            block,
        })
    }

    async fn default_async(_: &str) -> Result<Self, Error> {
        Ok(Self::new(Block::default()))
    }
}

impl Default for BlockMessage {
    fn default() -> Self {
        Self::new(Block::default())
    }
}
