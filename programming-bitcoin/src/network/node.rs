use std::io::{Cursor, Error};

use tokio::io::{AsyncWriteExt, AsyncReadExt};
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
        let command = message.command();
        let envelope = NetworkEnvelope::new(command, message.serialize(), self.testnet);

        if self.logging {
            println!("Sending {}\n{}\n", command, envelope);
        }

        // Send the serialized message
        self.stream.write_all(&envelope.serialize()).await?;
        println!("{} message sent", command);

        Ok(())
    }

    // Read message from socket
    pub async fn read(&mut self) -> Result<NetworkEnvelope, Error> {
        let mut buffer = vec![0; 1024];
        let bytes_read = self.stream.read(&mut buffer).await?;
        let mut reader = Cursor::new(buffer[..bytes_read].to_vec());
        NetworkEnvelope::parse(&mut reader)
    }

    pub async fn wait_for<T: NetworkMessage>(&mut self, message_types: Vec<T>) -> Result<T, Box<dyn std::error::Error>> {
        loop {
            let envelope = self.read().await?;
            println!("Received message: \n{}", envelope);

            let command = String::from_utf8_lossy(&envelope.command)
            .trim_matches(char::from(0))
            .to_string();
            
            // Handle automatic protocol responses
            match command.as_str() {
                "version" => {
                    println!("Version message received (wait_for)");
                    return Ok(T::parse(&T::default_async(command.as_str()).await, &mut Cursor::new(envelope.payload)).unwrap());
                },
                "ping" => {
                    println!("ping received (wait_for). Sending pong");
                    self.send(PongMessage::new(envelope.payload)).await?;
                }
                "verack" => {
                    println!("Verack received (wait_for)");
                    return Ok(T::parse(&T::default_async(command.as_str()).await, &mut Cursor::new(envelope.payload)).unwrap());
                }
                cmd if message_types.iter().any(|m| m.command() == cmd) => {
                    println!("Unrecognised message received (wait_for)");
                    return Ok(T::parse(&T::default_async(cmd).await, &mut Cursor::new(envelope.payload)).unwrap());
                }
                _ => continue // Unknown message, keep waiting
            }
        }
    }

    pub async fn handshake(host: &str, port: u32) -> Result<(), Error> {
        let testnet = true;
        let logging = true;

        let mut node = Self::new(host, port, testnet, logging).await.unwrap();

        let version = VersionMessage::new_default_message().await;

        let verack = VerAckMessage::new();

        node.send(version.clone()).await.unwrap();
        println!("Version message sent");

        let mut verack_received = false;

        let mut version_received = false;

        let message_types: Vec<NetworkMessages> = vec![
            NetworkMessages::Version(version),
            NetworkMessages::VerAck(verack)
        ];

        while !(verack_received && version_received) {
            let message = node.wait_for(message_types.clone()).await.unwrap();

            match message {
                NetworkMessages::VerAck(_) => {
                    println!("Verack received. verack_received = true. Sending verack\nHANDSHAKE COMPLETE");
                    let _ = node.send(VerAckMessage::new()).await;
                    verack_received = true;
                },
                NetworkMessages::Version(_) => {
                    println!("Version received. version_received = true. Waiting for verack...");
                    version_received = true;
                },
                _ => ()
            }
        }
        dbg!(version_received);
        dbg!(verack_received);
        Ok(())
    }
}
