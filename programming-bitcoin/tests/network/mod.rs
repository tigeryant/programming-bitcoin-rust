use std::io::Cursor;

use programming_bitcoin::network::network_envelope::NetworkEnvelope;
use programming_bitcoin::network::handshake::handshake;
use programming_bitcoin::network::network_envelope::{TESTNET_NETWORK_MAGIC, MAINNET_NETWORK_MAGIC};
use programming_bitcoin::network::messages::version::VersionMessage;
use programming_bitcoin::network::network_message::NetworkMessage;
use std::time::{SystemTime, UNIX_EPOCH};


#[test]
fn test_new_network_message() {
    let command = "verack";
    let payload = hex::decode("f9beb4d976657273696f6e0000000000650000005f1a69d2721101000100000000000000bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d010000000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001").unwrap();
    let testnet = true;
    let network_message = NetworkEnvelope::new(command, payload.clone(), testnet);

    let message_testnet = match network_message.magic {
        TESTNET_NETWORK_MAGIC => true,
        MAINNET_NETWORK_MAGIC => false,
        _ => false
    };

    let mut command_bytes = [0u8; 12];
    for (i, byte) in command.bytes().enumerate() {
        if i >= 12 { break; }
        command_bytes[i] = byte;
    }

    assert_eq!(message_testnet, testnet);
    assert_eq!(network_message.command, command_bytes);
    assert_eq!(network_message.payload, payload);
}

#[test]
fn test_parse_network_message() {
    let raw_message = hex::decode("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_message);
    let output_message = NetworkEnvelope::parse(&mut stream);
    assert!(output_message.is_ok());
    let output_message = output_message.unwrap();
    println!("{}", hex::encode(output_message.clone().serialize()));
    println!("{}", output_message);
}

#[test]
fn test_new_version_message() {
    let version: u32 = 70015;
    let services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let timestamp: Option<u64> = Some(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs());
    let receiver_services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let receiver_ip: [u8; 16] = hex::decode("00000000000000000000ffff00000000").unwrap().try_into().unwrap();
    let receiver_port: u16 = 8333;
    let sender_services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let sender_ip: [u8; 16] = hex::decode("00000000000000000000ffff00000000").unwrap().try_into().unwrap();
    let sender_port: u16 = 8333;
    let nonce: Option<u64> = Some(rand::random::<u64>());
    let user_agent: &str = "/programmingblockchain:0.1/";
    let latest_block: u32 = 0;
    let relay : bool= true;

    let version_message = VersionMessage::new(version, services, timestamp, receiver_services, receiver_ip, receiver_port, sender_services, sender_ip, sender_port, nonce, user_agent, latest_block, relay);

    assert_eq!(version, u32::from_le_bytes(version_message.version));
    assert_eq!(services, version_message.services);
    assert_eq!(timestamp, Some(u64::from_le_bytes(version_message.timestamp)));
    assert_eq!(receiver_services, version_message.receiver_services);
    assert_eq!(receiver_ip, version_message.receiver_ip);
    assert_eq!(receiver_port, u16::from_be_bytes(version_message.receiver_port));
    assert_eq!(sender_services, version_message.sender_services);
    assert_eq!(sender_ip, version_message.sender_ip);
    assert_eq!(sender_port, u16::from_be_bytes(version_message.sender_port));
    assert_eq!(nonce, Some(u64::from_le_bytes(version_message.nonce)));
    assert_eq!(user_agent, String::from_utf8(version_message.user_agent).unwrap());
    assert_eq!(latest_block, version_message.latest_block);
    assert_eq!(relay, version_message.relay);
}

#[test]
fn test_serialize_version_message() {
    let version: u32 = 70015;
    let services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let timestamp: Option<u64> = Some(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs());
    let receiver_services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let receiver_ip: [u8; 16] = hex::decode("00000000000000000000ffff00000000").unwrap().try_into().unwrap();
    let receiver_port: u16 = 8333;
    let sender_services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let sender_ip: [u8; 16] = hex::decode("00000000000000000000ffff00000000").unwrap().try_into().unwrap();
    let sender_port: u16 = 8333;
    let nonce: Option<u64> = Some(rand::random::<u64>());
    let user_agent: &str = "/programmingblockchain:0.1/";
    let latest_block: u32 = 0;
    let relay : bool= true;

    let version_message = VersionMessage::new(version, services, timestamp, receiver_services, receiver_ip, receiver_port, sender_services, sender_ip, sender_port, nonce, user_agent, latest_block, relay);   
    dbg!(hex::encode(version_message.serialize()));
}

#[test]
#[ignore]
fn test_handshake() {
    let host: &str = "192.168.2.4"; // node ip
    let port: u32 = 18333; // default testnet port

    // Create and serialize a version message
    let version = VersionMessage::new_default_message();
    let serialized_version = version.serialize();
    dbg!(hex::encode(serialized_version));
    let network_envelope = NetworkEnvelope::new("version", version.serialize(), true);

    handshake(host, port, network_envelope).unwrap();
}
