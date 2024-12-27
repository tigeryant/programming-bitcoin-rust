use std::io::{Cursor, Error};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
    pub async fn new(
        host: &str,
        port: u32,
        testnet: bool,
        logging: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(&addr).await?;
        println!("Connected to {}", addr);

        Ok(Self {
            testnet,
            logging,
            stream,
        })
    }

    // Send a message to the connected node
    pub async fn send<T: NetworkMessage>(
        &mut self,
        message: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command = message.command();
        let envelope = NetworkEnvelope::new(command, message.serialize(), self.testnet);

        if self.logging {
            println!("Sending {} message:\n{}", command, envelope);
        }

        // Send the serialized message
        self.stream.write_all(&envelope.serialize()).await?;

        Ok(())
    }

    pub async fn read(&mut self) -> Result<NetworkEnvelope, Error> {
        // Read header first (24 bytes: magic + command + length + checksum)
        let mut header = vec![0; 24];
        self.stream.read_exact(&mut header).await?;

        // Parse payload length from header (bytes 16-20)
        let payload_length = u32::from_le_bytes(header[16..20].try_into().unwrap()) as usize;

        // Read exact payload length
        let mut payload = vec![0; payload_length];
        self.stream.read_exact(&mut payload).await?;

        // Combine header and payload
        let mut full_message = header;
        full_message.extend(payload);

        // Parse the complete message
        let mut reader = Cursor::new(full_message);
        NetworkEnvelope::parse(&mut reader)
    }

    pub async fn listen<T: NetworkMessage>(
        &mut self,
    ) -> Result<T, Box<dyn std::error::Error>> {
        loop {
            let envelope = self.read().await?;
            let command = String::from_utf8_lossy(&envelope.command)
                .trim_matches(char::from(0))
                .to_string();

            println!("Received {} message:\n{}", command, envelope);

            // Handle automatic protocol responses
            // Returns the parsed message if it's version or verack, responds to ping
            match command.as_str() {
                "version" | "verack" => {
                    return Ok(T::parse(
                        // rewrite this so it's readable
                        &T::default_async(command.as_str()).await.unwrap(),
                        &mut Cursor::new(envelope.payload),
                    )
                    .unwrap());
                }
                "ping" => {
                    self.send(PongMessage::new(envelope.payload)).await?;
                }
                _ => continue, // Message unrelated to handshake, keep waiting
            }
        }
    }

    pub async fn handshake(&mut self) -> Result<(), Error> {
        let version = VersionMessage::new_default_message().await;

        self.send(version).await.unwrap();

        let mut verack_received = false;

        let mut version_received = false;

        while !(verack_received && version_received) {
            let received_message = self.listen().await.unwrap();

            match received_message {
                NetworkMessages::VerAck(_) => {
                    self.send(VerAckMessage::new()).await.unwrap();
                    println!("HANDSHAKE COMPLETE");
                    verack_received = true;
                }
                NetworkMessages::Version(_) => {
                    println!("Waiting for verack...");
                    version_received = true;
                }
                _ => (),
            }
        }
        Ok(())
    }
}
