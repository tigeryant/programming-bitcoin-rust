use std::io::{Cursor, Read, Error};

pub struct Block {
    version: [u8; 4], // little endian
    prev_block: [u8; 32], // little endian
    merkle_root: [u8; 32], // little endian
    timestamp: [u8; 4], // little endian
    bits: [u8; 4],
    nonce: [u8; 4]
}

impl Block {
    pub fn new(version: [u8; 4], prev_block: [u8; 32], merkle_root: [u8; 32], timestamp: [u8; 4], bits: [u8; 4], nonce: [u8; 4]) -> Self {
        Self {
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce
        }
    }

    // Parses a block from a byte stream
    pub fn parse(reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let mut prev_block = [0u8; 32];
        reader.read_exact(&mut prev_block)?;
        let mut merkle_root = [0u8; 32];
        reader.read_exact(&mut merkle_root)?;
        let mut timestamp = [0u8; 4];
        reader.read_exact(&mut timestamp)?;
        let mut bits = [0u8; 4];
        reader.read_exact(&mut bits)?;
        let mut nonce = [0u8; 4];
        reader.read_exact(&mut nonce)?;
        Ok(Self {
            version,
            prev_block,
            merkle_root,
            timestamp,
            bits,
            nonce
        })
    }
}