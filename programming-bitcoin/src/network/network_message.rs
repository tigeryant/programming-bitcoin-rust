use std::io::{Cursor, Error};

use super::messages::{get_headers::GetHeadersMessage, headers::HeadersMessage, pong::PongMessage, verack::VerAckMessage, version::VersionMessage};

pub trait NetworkMessage where Self: Sized {
    fn command(&self) -> &str;
    fn serialize(&self) -> Vec<u8>;
    fn parse(&self, stream: &mut Cursor<Vec<u8>>) -> Result<Self, Error>;
    async fn default_async(cmd: &str) -> Result<Self, Error>;
}

#[derive(Clone)]
pub enum NetworkMessages {
    Version(VersionMessage),
    VerAck(VerAckMessage),
    Pong(PongMessage),
    GetHeaders(GetHeadersMessage),
    Headers(HeadersMessage),
}

impl NetworkMessage for NetworkMessages {
    fn command(&self) -> &str {
        match self {
            NetworkMessages::Version(msg) => msg.command(),
            NetworkMessages::VerAck(msg) => msg.command(),
            NetworkMessages::Pong(msg) => msg.command(),
            NetworkMessages::GetHeaders(msg) => msg.command(),
            NetworkMessages::Headers(msg) => msg.command(),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        match self {
            NetworkMessages::Version(msg) => msg.serialize(),
            NetworkMessages::VerAck(msg) => msg.serialize(),
            NetworkMessages::Pong(msg) => msg.serialize(),
            NetworkMessages::GetHeaders(msg) => msg.serialize(),
            NetworkMessages::Headers(msg) => msg.serialize(),
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
            NetworkMessages::GetHeaders(msg) => {
                let get_headers_msg = GetHeadersMessage::parse(msg, reader)?;
                Ok(NetworkMessages::GetHeaders(get_headers_msg))
            },
            NetworkMessages::Headers(msg) => {
                let headers_msg = HeadersMessage::parse(msg, reader)?;
                Ok(NetworkMessages::Headers(headers_msg))
            },
        }
    }

    async fn default_async(cmd: &str) -> Result<Self, Error> {
        match cmd {
            "version" => Ok(NetworkMessages::Version(VersionMessage::default_async(cmd).await.unwrap())),
            "verack" => Ok(NetworkMessages::VerAck(VerAckMessage::default_async(cmd).await.unwrap())),
            "getheaders" => Ok(NetworkMessages::GetHeaders(GetHeadersMessage::default_async(cmd).await.unwrap())),
            "headers" => Ok(NetworkMessages::Headers(HeadersMessage::default_async(cmd).await.unwrap())),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unsupported command: {}", cmd)
            ))
        }
    }
}

// reimplement this
/*
impl Default for NetworkMessages {
    fn default() -> Self {
        NetworkMessages::Version(VersionMessage::new_default_message().await)
    }
}
*/
