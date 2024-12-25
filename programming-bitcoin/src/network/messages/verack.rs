use std::io::{Cursor, Error};

use crate::network::network_message::NetworkMessage;

#[derive(Clone)]
pub struct VerAckMessage {
    pub command: String,
}

impl VerAckMessage {
    pub fn new() -> Self {
        let command = String::from("verack");

        Self {
            command
        }
    }
}

impl NetworkMessage for VerAckMessage {
    fn command(&self) -> &str {
        &self.command
    }

    fn serialize(&self) -> Vec<u8> {
        vec![]
    }

    fn parse(&self, _: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let command = String::from("verack");
        Ok(Self {
            command
        })
    }
}

impl Default for VerAckMessage {
    fn default() -> Self {
        Self::new()
    }
}