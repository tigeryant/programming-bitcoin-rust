use std::io::{Cursor, Read, Error};
use std::fmt;

use crate::utils::hash256::hash256;

pub struct Block { // all these fields are stored as little endian
    pub version: [u8; 4],
    pub prev_block: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: [u8; 4],
    pub bits: [u8; 4],
    pub nonce: [u8; 4]
}

impl Block {
    // all fields should be given in little endian
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

    /// Returns the byte serialization of the block
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend(self.version);

        result.extend(self.prev_block);

        result.extend(self.merkle_root);

        result.extend(self.timestamp);

        result.extend(self.bits);

        result.extend(self.nonce);

        result
    }

    pub fn hash(&self) -> Vec<u8> {
        let serialized = self.serialize();
        let hash = hash256(&serialized); // little endian
        // Reverse the result to return big endian
        hash.into_iter().rev().collect()
    }

    // Returns true to indicate BIP9 support
    pub fn bip9(&self) -> bool {
        let version: u32 = u32::from_le_bytes(self.version);
        version >> 29 == 0b001
    }

    // Returns true to indicate BIP91 support
    pub fn bip91(&self) -> bool {
        let version: u32 = u32::from_le_bytes(self.version);
        version >> 4 & 1 == 1
    }

    // Returns true to indicate BIP141 support
    pub fn bip141(&self) -> bool {
        let version: u32 = u32::from_le_bytes(self.version);
        version >> 1 & 1 == 1
    }
}

impl fmt::Display for Block {
    // note that these fields are all displayed in big endian
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Version: {}", hex::encode(self.version.iter().rev().cloned().collect::<Vec<u8>>()))?;
        writeln!(f, "Previous Block: {}", hex::encode(self.prev_block.iter().rev().cloned().collect::<Vec<u8>>()))?;
        writeln!(f, "Merkle Root: {}", hex::encode(self.merkle_root.iter().rev().cloned().collect::<Vec<u8>>()))?;
        writeln!(f, "Timestamp: {}", hex::encode(self.timestamp.iter().rev().cloned().collect::<Vec<u8>>()))?;
        writeln!(f, "Bits: {}", hex::encode(self.bits.iter().rev().cloned().collect::<Vec<u8>>()))?;
        writeln!(f, "Nonce: {}", hex::encode(self.nonce.iter().rev().cloned().collect::<Vec<u8>>()))
    }
}
