use std::io::{Cursor, Error};

use crate::{network::{inventory::Inventory, network_message::NetworkMessage}, utils::varint::{encode_varint, read_varint}};

#[derive(Clone)]
pub struct GetDataMessage {
    pub command: String,
    pub count: u64,
    pub inventory_vec: Vec<Inventory>
}

impl GetDataMessage {
    pub fn new(count: u64, inventory_vec: Vec<Inventory>) -> Self {
        let command = String::from("getdata");

        Self {
            command,
            count,
            inventory_vec,
        }
    }
}

impl NetworkMessage for GetDataMessage {
    fn command(&self) -> &str {
        &self.command
    }

    // Serializes an instance of self into a byte vector
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend_from_slice(&encode_varint(self.count));

        for inventory in &self.inventory_vec {
            result.extend_from_slice(&inventory.serialize());
        }

        result
    }

    fn parse(&self, reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let command = String::from("getdata");

        let count = read_varint(reader)?;

        let mut inventory_vec = Vec::with_capacity(count as usize);
        for _ in 0..count {
            inventory_vec.push(Inventory::parse(reader).unwrap());
        }

        Ok(Self {
            command,
            count,
            inventory_vec,
        })
    }

    async fn default_async(_: &str) -> Result<Self, Error> {
        Ok(Self::new(0, vec![]))
    }
}
