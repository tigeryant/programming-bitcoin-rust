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
    // how to derive the command??
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
}
