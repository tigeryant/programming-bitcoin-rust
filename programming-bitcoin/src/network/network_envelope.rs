use std::{fmt, io::{Cursor, Error, Read}};

use crate::utils::hash256::hash256;

pub const MAINNET_NETWORK_MAGIC: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9]; 
pub const TESTNET_NETWORK_MAGIC: [u8; 4] = [0x0b, 0x11, 0x09, 0x07]; // testnet3

#[derive(Clone)]
pub struct NetworkEnvelope {
    pub magic: [u8; 4],
    pub command: [u8; 12],
    pub payload: Vec<u8>
}

impl NetworkEnvelope {
    pub fn new(command: &str, payload: Vec<u8>, testnet: bool) -> Self {
        let magic = match testnet {
            true => TESTNET_NETWORK_MAGIC,
            _ => MAINNET_NETWORK_MAGIC
        };

        let mut command_bytes = [0u8; 12];
        for (i, byte) in command.bytes().enumerate() {
            if i >= 12 { break; }
            command_bytes[i] = byte;
        }

        Self {
            magic,
            command: command_bytes,
            payload
        }
    }

    // Parses a NetworkEnvelope from a byte stream
    pub fn parse(reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        let mut command = [0u8; 12];
        reader.read_exact(&mut command)?;

        let mut payload_length = [0u8; 4];
        reader.read_exact(&mut payload_length)?;

        let payload_length = u32::from_le_bytes(payload_length);

        let mut parsed_checksum = [0u8; 4];
        reader.read_exact(&mut parsed_checksum)?;
        
        let mut payload = vec![0u8; payload_length as usize];
        reader.read_exact(&mut payload)?;
        
        let expected_checksum: [u8; 4] = hash256(&payload)[..4].try_into().unwrap();

        if parsed_checksum != expected_checksum {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Checksum mismatch"));
        }

        Ok(Self {
            magic,
            command,
            payload
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend_from_slice(&self.magic);

        result.extend_from_slice(&self.command);

        let payload_length: [u8; 4] = (self.payload.len() as u32).to_le_bytes();
        result.extend_from_slice(&payload_length);

        let payload_checksum: [u8; 4] = hash256(&self.payload)[..4].try_into().unwrap();
        result.extend_from_slice(&payload_checksum);

        result.extend_from_slice(&self.payload);

        result
    }

}

impl fmt::Display for NetworkEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command_str = String::from_utf8_lossy(&self.command)
            .trim_matches(char::from(0))
            .to_string();
            
        let magic: String = match self.magic {
            MAINNET_NETWORK_MAGIC => "mainnet".to_string(),
            TESTNET_NETWORK_MAGIC => "testnet".to_string(),
            _ => "invalid magic".to_string()
        };

        let payload = match self.payload.len() {
            0 => "<no payload>",
            _ => &hex::encode(&self.payload)
        };

        write!(f, "Magic: {}\nCommand: {}\nPayload: {}", 
            magic,
            command_str,
            payload,
        )
    }
}
