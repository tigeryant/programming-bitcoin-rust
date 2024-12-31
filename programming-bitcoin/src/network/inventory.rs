use std::io::{Cursor, Read, Error};

#[derive(Clone)]
pub struct Inventory {
    object_type: u32, // little endian
    hash: [u8; 32]
}

impl Inventory {
    // update object type to take an enum variant (ObjectTypes) instead of a u32
    pub fn new(object_type: u32, hash: [u8; 32]) -> Self {
        Self {
            object_type,
            hash
        }
    } 

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend_from_slice(&self.object_type.to_le_bytes());

        result.extend_from_slice(&self.hash);

        result
    }

    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).unwrap();
        let object_type = u32::from_le_bytes(buffer);

        let mut hash = [0u8; 32];
        stream.read_exact(&mut hash)?;

        Ok(Self {
            object_type,
            hash
        })
    }
}
