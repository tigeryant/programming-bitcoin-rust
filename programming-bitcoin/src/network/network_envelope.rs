use std::fmt;

const MAINNET_NETWORK_MAGIC: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9]; 
const TESTNET_NETWORK_MAGIC: [u8; 4] = [0x0b, 0x11, 0x09, 0x07];

pub struct NetworkEnvolope {
    pub magic: [u8; 4],
    pub command: [u8; 12],
    pub payload: Vec<u8>
}

impl NetworkEnvolope {
    pub fn new(command: [u8; 12], payload: Vec<u8>, testnet: bool) -> Self {
        let magic = match testnet {
            true => TESTNET_NETWORK_MAGIC,
            _ => MAINNET_NETWORK_MAGIC
        };

        Self {
            magic,
            command,
            payload
        }
    }
}

impl fmt::Display for NetworkEnvolope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command_str = String::from_utf8_lossy(&self.command)
            .trim_matches(char::from(0))
            .to_string();
            
        write!(f, "Command: {}\nPayload: {}", 
            command_str,
            hex::encode(&self.payload)
        )
    }
}
