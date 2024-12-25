use std::io::{Cursor, Error};

use super::messages::{pong::PongMessage, verack::VerAckMessage, version::VersionMessage};

pub trait NetworkMessage where Self: Sized {
    fn command(&self) -> &str;
    fn serialize(&self) -> Vec<u8>;
    fn parse(&self, stream: &mut Cursor<Vec<u8>>) -> Result<Self, Error>;
}

#[derive(Clone)]
pub enum NetworkMessages {
    Version(VersionMessage),
    VerAck(VerAckMessage),
    Pong(PongMessage),
}

impl NetworkMessage for NetworkMessages {
    fn command(&self) -> &str {
        match self {
            NetworkMessages::Version(msg) => msg.command(),
            NetworkMessages::VerAck(msg) => msg.command(),
            NetworkMessages::Pong(msg) => msg.command(),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        match self {
            NetworkMessages::Version(msg) => msg.serialize(),
            NetworkMessages::VerAck(msg) => msg.serialize(),
            NetworkMessages::Pong(msg) => msg.serialize(),
        }
    }

    fn parse(&self, reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        match self {
            NetworkMessages::Version(msg) => {
                let version_msg = VersionMessage::parse(msg, reader)?;
                Ok(NetworkMessages::Version(version_msg))
            },
            NetworkMessages::VerAck(msg) => {
                let verack_msg = VerAckMessage::parse(msg, reader)?;
                Ok(NetworkMessages::VerAck(verack_msg))
            },
            NetworkMessages::Pong(msg) => {
                let pong_msg = PongMessage::parse(msg, reader)?;
                Ok(NetworkMessages::Pong(pong_msg))
            },
        }
    }

}

impl Default for NetworkMessages {
    fn default() -> Self {
        NetworkMessages::Version(VersionMessage::new_default_message())
    }
}
