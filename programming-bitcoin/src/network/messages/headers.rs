use crate::{
    blocks::block::Block, network::network_message::NetworkMessage, utils::varint::{encode_varint, read_varint}
};
use std::io::{Cursor, Error};

#[derive(Clone)]
pub struct HeadersMessage {
    pub command: String,
    pub blocks: Vec<Block>,
}

impl HeadersMessage {
    // start and end block given in little endian
    pub fn new(blocks: Vec<Block>) -> Self {
        let command = String::from("headers");

        Self {
            command,
            blocks
        }
    }
}

impl NetworkMessage for HeadersMessage {
    fn command(&self) -> &str {
        &self.command
    }

    // Serializes an instance of self into a byte vector
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend_from_slice(&encode_varint(self.blocks.len() as u64));

        for header in self.blocks.clone() {
            result.extend_from_slice(&header.serialize());
            result.extend_from_slice(&[0x00]);
        }

        result
    }

    fn parse(&self, reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let command = String::from("headers");

        let num_headers = read_varint(reader)?;
        let blocks: Vec<Block> = (0..num_headers)
            .map(|_| {
                let block = Block::parse(reader).unwrap();
                let num_txs = read_varint(reader).unwrap();
                if num_txs != 0 {
                    return Err(Error::new(std::io::ErrorKind::InvalidData, "Number of transactions must be 0 for headers message"))
                };
                Ok(block)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            command,
            blocks,
        })
    }

    async fn default_async(_: &str) -> Result<Self, Error> {
        Ok(Self::new(vec![]))
    }
}
