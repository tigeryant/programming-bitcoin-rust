use std::io::{Cursor, Error, Read};

use crate::network::network_message::NetworkMessage;

#[derive(Clone)]
pub struct PongMessage {
    pub command: String,
    pub nonce: [u8; 8]
}

impl PongMessage {
    pub fn new(nonce: Vec<u8>) -> Self {
        let command = String::from("pong");

        let nonce = nonce.try_into().unwrap();

        Self {
            command,
            nonce
        }
    }
}

impl NetworkMessage for PongMessage {
    fn command(&self) -> &str {
        &self.command
    }

    fn serialize(&self) -> Vec<u8> {
        vec![]
    }

    fn parse(&self, reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let command = String::from("pong");

        let mut nonce = [0u8; 8];
        reader.read_exact(&mut nonce)?;

        Ok(Self {
            command,
            nonce
        })
    }
}