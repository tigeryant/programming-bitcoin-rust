use std::io::{Cursor, Error};

use crate::{blocks::block_header::BlockHeader, network::network_message::NetworkMessage, transactions::tx::Tx, utils::varint::{encode_varint, read_varint}};

#[derive(Clone)]
pub struct BlockMessage {
    pub command: String,
    pub header: BlockHeader,
    pub tx_count: u64,
    pub txs: Vec<Tx>
}

impl BlockMessage {
    pub fn new(header: BlockHeader, txs: Vec<Tx>) -> Self {
        let command = String::from("block");

        let tx_count = txs.len() as u64;

        Self {
            command,
            header,
            tx_count,
            txs,
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

        let header = BlockHeader::serialize(&self.header);
        result.extend_from_slice(&header);

        result.extend_from_slice(&encode_varint(self.tx_count));

        for tx in &self.txs {
            result.extend_from_slice(&tx.serialize());
        }

        result
    }

    fn parse(&self, reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let command = String::from("block");

        let header = BlockHeader::parse(reader).unwrap();

        let tx_count = read_varint(reader)?;

        // set testnet as true for now
        let testnet = true;

        let mut txs = Vec::with_capacity(tx_count as usize);
        for _ in 0..tx_count {
            txs.push(Tx::parse(reader, testnet));
        }

        Ok(Self {
            command,
            header,
            tx_count,
            txs
        })
    }

    async fn default_async(_: &str) -> Result<Self, Error> {
        Ok(Self::new(BlockHeader::default(), vec![]))
    }
}

impl Default for BlockMessage {
    fn default() -> Self {
        Self::new(BlockHeader::default(), vec![])
    }
}
