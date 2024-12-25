use std::io::{Cursor, Error};

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use super::messages::pong::PongMessage;
use super::messages::verack::VerAckMessage;
use super::messages::version::VersionMessage;
use super::network_envelope::NetworkEnvelope;
use super::network_message::{NetworkMessage, NetworkMessages};

pub struct Node {
    pub testnet: bool,
    pub logging: bool,
    pub stream: TcpStream,

}

impl Node {
    pub async fn new(host: &str, port: u32, testnet: bool, logging: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(addr).await?;

        Ok(Self {
            testnet,
            logging,
            stream
        })
    }

    // Send a message to the connected node
    pub async fn send<T: NetworkMessage>(&mut self, message: T) -> Result<(), Box<dyn std::error::Error>> {
        let envelope = NetworkEnvelope::new(message.command(), message.serialize(), self.testnet);

        if self.logging {
            println!("Sending {}", envelope);
        }

        // Send the serialized message
        self.stream.write_all(&envelope.serialize()).await?;
        println!("Message sent");

        Ok(())
    }

    // Read message from socket
    pub async fn read(&self) -> Result<NetworkEnvelope, Error> {
        let buffer = vec![0; 1024];
        let mut reader = Cursor::new(buffer);
        NetworkEnvelope::parse(&mut reader)
    }

    pub async fn wait_for<T: NetworkMessage + Default>(&mut self, message_types: Vec<T>) -> Result<T, Box<dyn std::error::Error>> {
        loop {
            let envelope = self.read().await?;

            let command = String::from_utf8_lossy(&envelope.command)
            .trim_matches(char::from(0))
            .to_string();
            
            // Handle automatic protocol responses
            match command.as_str() {
                "version" => {
                    self.send(VerAckMessage::new()).await?;
                },
                "ping" => {
                    // pong must return the nonce found in the ping
                    self.send(PongMessage::new(envelope.payload)).await?;
                }
                cmd if message_types.iter().any(|m| m.command() == cmd) => {
                    // Parse and return the expected message type
                    return Ok(T::parse(&T::default(), &mut Cursor::new(envelope.payload)).unwrap());
                }
                _ => continue // Unknown message, keep waiting
            }
        }
    }

    pub async fn handshake<T: NetworkMessage>() -> Result<(), Error> {
        let host = "192.168.2.4";
        let port = 18333;
        let testnet = true;
        let logging = true;

        let mut node = Self::new(host, port, testnet, logging).await.unwrap();

        let version = VersionMessage::new_default_message();

        let verack = VerAckMessage::new();

        node.send(version.clone()).await.unwrap();

        let mut verack_received = false;

        let mut version_received = false;

        let message_types: Vec<NetworkMessages> = vec![
            NetworkMessages::Version(version),
            NetworkMessages::VerAck(verack)
        ];

        while !verack_received && !version_received {
            let message = node.wait_for(message_types.clone()).await.unwrap();

            match message {
                NetworkMessages::VerAck(_) => verack_received = true,
                NetworkMessages::Version(_) => {
                    version_received = true;
                    let _ = node.send(VerAckMessage::new()).await;
                },
                _ => ()
            }
        }

        Ok(())
    }
}