use std::time::{SystemTime, UNIX_EPOCH};

pub struct VersionMessage {
    command: String,
    version: [u8; 4],
    services: [u8; 8],
    timestamp: [u8; 8],
    receiver_services: [u8; 8],
    receiver_ip: [u8; 16],
    receiver_port: [u8; 2],
    sender_services: [u8; 8],
    sender_ip: [u8; 16],
    sender_port: [u8; 2],
    nonce: [u8; 8],
    user_agent: Vec<u8>,
    latest_block: u32,
    relay: bool,
}

impl VersionMessage {
    pub fn new(
        version: [u8; 4],
        services: [u8; 8],
        timestamp: Option<[u8; 8]>,
        receiver_services: [u8; 8],
        receiver_ip: [u8; 16],
        receiver_port: [u8; 2],
        sender_services: [u8; 8],
        sender_ip: [u8; 16],
        sender_port: [u8; 2],
        nonce: Option<[u8; 8]>,
        user_agent: Vec<u8>,
        latest_block: u32,
        relay: bool,
    ) -> Self {
        let command = String::from("version");

        let timestamp: [u8; 8] = match timestamp {
            Some(timestamp) => timestamp,
            None => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_le_bytes(),
        };

        let nonce: [u8; 8] = match nonce {
            Some(nonce) => nonce,
            None => rand::random::<u64>().to_le_bytes(),
        };

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
