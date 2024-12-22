use std::time::{SystemTime, UNIX_EPOCH};

pub struct VersionMessage {
    pub command: String,
    pub version: [u8; 4],
    pub services: [u8; 8],
    pub timestamp: [u8; 8],
    pub receiver_services: [u8; 8],
    pub receiver_ip: [u8; 16],
    pub receiver_port: [u8; 2],
    pub sender_services: [u8; 8],
    pub sender_ip: [u8; 16],
    pub sender_port: [u8; 2],
    pub nonce: [u8; 8],
    pub user_agent: Vec<u8>,
    pub latest_block: u32,
    pub relay: bool,
}

impl VersionMessage {
    // take version as a u32, convert to [u8; 4]
    pub fn new(
        version: u32,
        services: [u8; 8],
        timestamp: Option<u64>,
        receiver_services: [u8; 8],
        receiver_ip: [u8; 16],
        receiver_port: u16,
        sender_services: [u8; 8],
        sender_ip: [u8; 16],
        sender_port: u16,
        nonce: Option<u64>,
        user_agent: &str,
        latest_block: u32,
        relay: bool,
    ) -> Self {
        let command = String::from("version");

        let version: [u8; 4] = version.to_le_bytes();

        let timestamp: [u8; 8] = match timestamp {
            Some(timestamp) => timestamp.to_le_bytes(),
            None => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_le_bytes(),
        };

        let receiver_port: [u8; 2] = receiver_port.to_le_bytes();

        let sender_port: [u8; 2] = sender_port.to_le_bytes();

        let nonce: [u8; 8] = match nonce {
            Some(nonce) => nonce.to_le_bytes(),
            None => rand::random::<u64>().to_le_bytes(),
        };

        let user_agent: Vec<u8> = user_agent.as_bytes().to_vec();

        Self {
            command,
            version,
            services,
            timestamp,
            receiver_services,
            receiver_ip,
            receiver_port,
            sender_services,
            sender_ip,
            sender_port,
            nonce,
            user_agent,
            latest_block,
            relay,
        }
    }

    // Serializes an instance of self into a byte vector
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend_from_slice(&self.version);

        result.extend_from_slice(&self.services);

        result.extend_from_slice(&self.timestamp);

        result.extend_from_slice(&self.receiver_services);

        result.extend_from_slice(&self.receiver_ip);

        result.extend_from_slice(&self.receiver_port);

        result.extend_from_slice(&self.sender_services);

        result.extend_from_slice(&self.sender_ip);

        result.extend_from_slice(&self.sender_port);

        result.extend_from_slice(&self.nonce);

        result.extend_from_slice(&self.user_agent);

        result.extend_from_slice(&self.latest_block.to_le_bytes()); // to big or little endian?

        result.extend_from_slice(&[u8::from(self.relay)]);

        result
    }
}
