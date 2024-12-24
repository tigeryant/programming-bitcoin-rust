use std::io::{Cursor, Error};

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use super::messages::pong::PongMessage;
use super::messages::verack::VerAckMessage;
use super::network_envelope::NetworkEnvelope;
use super::network_message::NetworkMessage;

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

    pub async fn wait_for<T: NetworkMessage>(&mut self, message_types: Vec<T>) -> Result<T, Box<dyn std::error::Error>> {
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
                    return Ok(T::parse(&mut Cursor::new(envelope.payload)).unwrap());
                }
                _ => continue // Unknown message, keep waiting
            }
        }
    }
}