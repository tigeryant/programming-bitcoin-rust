use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::network::{network_envelope::NetworkEnvelope, version_message::VersionMessage};
use std::io::{Cursor, Read, Error};

#[tokio::main]
pub async fn handshake() -> Result<(), Box<dyn std::error::Error>> {
    let host = "testnet.programmingbitcoin.com";
    let port = 18333;
    let addr = format!("{}:{}", host, port);

    // Connect to the server
    let mut stream = TcpStream::connect(&addr).await?;
    println!("Connected to {}", addr);

    // Create and serialize a version message
    let version = VersionMessage::new_default_message();
    let envelope = NetworkEnvelope::new("version", version.serialize(), true);

    // Send the serialized message
    stream.write_all(&envelope.serialize()).await?;
    println!("Message sent");

    // Read and parse responses in a loop
    let mut buffer = vec![0; 1024];
    let mut reader = Cursor::new(buffer.clone());

    loop {
        let bytes_read = stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            println!("Connection closed by the server.");
            break;
        }

        // Parse the incoming message
        if let Ok(new_message) = NetworkEnvelope::parse(&mut reader) {
            println!("Received message: {}", new_message);
        }
    }

    Ok(())
}
