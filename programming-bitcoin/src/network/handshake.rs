use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::network::network_envelope::NetworkEnvelope;
use std::io::Cursor;

#[tokio::main]
pub async fn handshake(host: &str, port: u32, network_envelope: NetworkEnvelope) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port);

    // Connect to the server
    let mut stream = TcpStream::connect(&addr).await?;
    println!("Connected to {}", addr);

    // Send the serialized message
    stream.write_all(&network_envelope.serialize()).await?;
    println!("Message sent");

    // Read and parse responses in a loop
    let mut buffer = vec![0; 1024];
    
    loop {
        let mut reader = Cursor::new(buffer.clone());
        let bytes_read = stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            println!("Connection closed by the server.");
            break;
        }

        // Parse the incoming message
        if let Ok(new_message) = NetworkEnvelope::parse(&mut reader) {
            println!("Received message:\n{}", new_message);
        }
    }

    Ok(())
}
