use std::io::{ Cursor, Error };

use crate::{spv::utils::merkle_root, transactions::tx::Tx, utils::varint::{encode_varint, read_varint}};

use super::block_header::BlockHeader;

#[derive(Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Tx>
}

impl Block {
    pub fn new(header: BlockHeader, txs: Vec<Tx>) -> Self {
        Self {
            header,
            txs
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend(self.header.serialize());

        let tx_count: u64 = self.txs.len() as u64;

        result.extend_from_slice(&encode_varint(tx_count));

        for tx in &self.txs {
            result.extend_from_slice(&tx.serialize());
        }

        result
    }

    pub fn parse(reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let header = BlockHeader::parse(reader)?;

        let tx_count = read_varint(reader)?;

        // testnet true for now
        let testnet = true;

        let txs = (0..tx_count)
            .map(|_| Tx::parse(reader, testnet))
            .collect();

        Ok(Self {
            header,
            txs
        })
    }

    pub fn validate_merkle_root(&self) -> bool {
        let hashes = &self.txs.clone()
            .into_iter()
            .map(|tx| hex::decode(tx.id()).unwrap())
            .collect::<Vec<Vec<u8>>>();

        let mut computed_merkle_root: [u8; 32] = merkle_root(hashes.to_vec()).try_into().unwrap();

        // Reverse byte order from big to little endian
        computed_merkle_root.reverse();

        let expected = self.header.merkle_root;

        expected == computed_merkle_root
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new(BlockHeader::default(), vec![])
    }
}
