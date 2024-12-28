use std::io::Cursor;

use programming_bitcoin::blocks::block::Block;
// use programming_bitcoin::blocks::utils::calculate_new_bits_from_previous;
use programming_bitcoin::network::get_tip_hash::get_tip_hash;
use programming_bitcoin::network::messages::get_headers::GetHeadersMessage;
use programming_bitcoin::network::messages::headers::HeadersMessage;
use programming_bitcoin::network::network_envelope::NetworkEnvelope;
use programming_bitcoin::network::messages::version::VersionMessage;
use programming_bitcoin::network::network_envelope::{
    MAINNET_NETWORK_MAGIC, TESTNET_NETWORK_MAGIC,
};
use programming_bitcoin::network::network_message::{NetworkMessage, NetworkMessages};
use programming_bitcoin::network::node::Node;
use std::time::{SystemTime, UNIX_EPOCH};

pub const PI_TESTNET_NODE_IP: &str = "192.168.2.4";
pub const DEFAULT_TESTNET_PORT: u32 = 18333;

pub const PUBLIC_TESTNET_NODE_IP: &str = "89.117.19.191";

pub static TESTNET_GENESIS_BLOCK_HASH: [u8; 32] = [
    0x43, 0x49, 0x7f, 0xd7, 0xf8, 0x26, 0x95, 0x71,
    0x08, 0xf4, 0xa3, 0x0f, 0xd9, 0xce, 0xc3, 0xae,
    0xba, 0x79, 0x97, 0x20, 0x84, 0x9e, 0x0e, 0xad,
    0x01, 0xea, 0x33, 0x09, 0x00, 0x00, 0x00, 0x00
];

pub static TESTNET_GENESIS_RAW_HEADER: [u8; 80] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x3b, 0xa3, 0xed, 0xfd,
    0x7a, 0x7b, 0x12, 0xb2, 0x7a, 0xc7, 0x2c, 0x3e,
    0x67, 0x76, 0x8f, 0x61, 0x7f, 0xc8, 0x1b, 0xc3,
    0x88, 0x8a, 0x51, 0x32, 0x3a, 0x9f, 0xb8, 0xaa,
    0x4b, 0x1e, 0x5e, 0x4a, 0xda, 0xe5, 0x49, 0x4d,
    0xff, 0xff, 0x00, 0x1d, 0x1a, 0xa4, 0xae, 0x18
];


pub static LOWEST_BITS: [u8; 4] = [0xff, 0xff, 0x00, 0x1d];

#[test]
fn test_new_network_message() {
    let command = "verack";
    let payload = hex::decode("f9beb4d976657273696f6e0000000000650000005f1a69d2721101000100000000000000bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d010000000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001").unwrap();
    let testnet = true;
    let network_message = NetworkEnvelope::new(command, payload.clone(), testnet);

    let message_testnet = match network_message.magic {
        TESTNET_NETWORK_MAGIC => true,
        MAINNET_NETWORK_MAGIC => false,
        _ => false,
    };

    let mut command_bytes = [0u8; 12];
    for (i, byte) in command.bytes().enumerate() {
        if i >= 12 {
            break;
        }
        command_bytes[i] = byte;
    }

    assert_eq!(message_testnet, testnet);
    assert_eq!(network_message.command, command_bytes);
    assert_eq!(network_message.payload, payload);
}

#[test]
fn test_parse_network_message() {
    let raw_message = hex::decode("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
    let mut stream: Cursor<Vec<u8>> = Cursor::new(raw_message);
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
    let timestamp: Option<u64> = Some(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    let receiver_services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let receiver_ip: [u8; 16] = hex::decode("00000000000000000000ffff00000000")
        .unwrap()
        .try_into()
        .unwrap();
    let receiver_port: u16 = 8333;
    let sender_services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let sender_ip: [u8; 16] = hex::decode("00000000000000000000ffff00000000")
        .unwrap()
        .try_into()
        .unwrap();
    let sender_port: u16 = 8333;
    let nonce: Option<u64> = Some(rand::random::<u64>());
    let user_agent: &str = "/programmingblockchain:0.1/";
    let latest_block: u32 = 0;
    let relay: bool = true;

    let version_message = VersionMessage::new(
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
    );

    assert_eq!(version, u32::from_le_bytes(version_message.version));
    assert_eq!(services, version_message.services);
    assert_eq!(
        timestamp,
        Some(u64::from_le_bytes(version_message.timestamp))
    );
    assert_eq!(receiver_services, version_message.receiver_services);
    assert_eq!(receiver_ip, version_message.receiver_ip);
    assert_eq!(
        receiver_port,
        u16::from_be_bytes(version_message.receiver_port)
    );
    assert_eq!(sender_services, version_message.sender_services);
    assert_eq!(sender_ip, version_message.sender_ip);
    assert_eq!(sender_port, u16::from_be_bytes(version_message.sender_port));
    assert_eq!(nonce, Some(u64::from_le_bytes(version_message.nonce)));
    assert_eq!(
        user_agent,
        String::from_utf8(version_message.user_agent).unwrap()
    );
    assert_eq!(latest_block, version_message.latest_block);
    assert_eq!(relay, version_message.relay);
}

#[test]
fn test_serialize_version_message() {
    let version: u32 = 70015;
    let services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let timestamp: Option<u64> = Some(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    let receiver_services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let receiver_ip: [u8; 16] = hex::decode("00000000000000000000ffff00000000")
        .unwrap()
        .try_into()
        .unwrap();
    let receiver_port: u16 = 8333;
    let sender_services: [u8; 8] = hex::decode("0000000000000000").unwrap().try_into().unwrap();
    let sender_ip: [u8; 16] = hex::decode("00000000000000000000ffff00000000")
        .unwrap()
        .try_into()
        .unwrap();
    let sender_port: u16 = 8333;
    let nonce: Option<u64> = Some(rand::random::<u64>());
    let user_agent: &str = "/programmingblockchain:0.1/";
    let latest_block: u32 = 0;
    let relay: bool = true;

    let version_message = VersionMessage::new(
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
    );
    dbg!(hex::encode(version_message.serialize()));
}

#[tokio::test]
async fn test_node_handshake() {
    // let host = PI_TESTNET_NODE_IP;
    let host = PUBLIC_TESTNET_NODE_IP;
    let port = DEFAULT_TESTNET_PORT;
    let testnet = true;
    let logging = true;
    let mut node = Node::new(host, port, testnet, logging).await.unwrap();

    assert!(Node::handshake(&mut node).await.is_ok());
}

#[tokio::test]
async fn test_get_headers() {
    // let host = PI_TESTNET_NODE_IP;
    let host = PUBLIC_TESTNET_NODE_IP;
    let port = DEFAULT_TESTNET_PORT;
    let testnet = true;
    let logging = true;
    let mut node = Node::new(host, port, testnet, logging).await.unwrap();

    node.handshake().await.unwrap();

    let tip_hash = get_tip_hash().await.unwrap();
    let getheaders = GetHeadersMessage::new(70015, 1, tip_hash, Some(TESTNET_GENESIS_BLOCK_HASH.to_vec()));
    node.send(getheaders).await.unwrap();

    let mut headers_received = false;

    while !(headers_received) {
        let received_message: NetworkMessages = node.listen().await.unwrap();

        if let NetworkMessages::Headers(_) = received_message { headers_received = true }
    }
    assert!(headers_received);
}

#[tokio::test]
async fn get_validate_headers() {
    let host = PUBLIC_TESTNET_NODE_IP;
    let port = DEFAULT_TESTNET_PORT;
    let testnet = true;
    let logging = true;
    let mut node = Node::new(host, port, testnet, logging).await.unwrap();

    let mut stream: Cursor<Vec<u8>> =  Cursor::new(TESTNET_GENESIS_RAW_HEADER.to_vec());
    let mut previous = Block::parse(&mut stream).unwrap();   
    // these are for the difficulty checking (mainnet)
    // let mut first_epoch_timestamp = previous.timestamp;
    // let mut expected_bits = LOWEST_BITS;

    let mut count: u32 = 1;

    node.handshake().await.unwrap();

    for i in 0..19 {
        let previous_hash = previous.hash().into_iter().rev().collect::<Vec<u8>>();
        let getheaders = GetHeadersMessage::new(70015, 1, previous_hash, None);

        node.send(getheaders).await.unwrap();

        let mut headers_received = false;

        let mut received_message: NetworkMessages;
        let mut headers = HeadersMessage::default_async("headers").await.unwrap();

        while !(headers_received) {
            received_message = node.listen().await.unwrap();
    
            if let NetworkMessages::Headers(header_message) = received_message { 
                headers_received = true;
                headers = header_message;
            }
        }

        let blocks = headers.blocks;

        for header in blocks {
            dbg!(hex::encode(header.hash()));
            if !header.check_pow() {
                panic!("Invalid PoW at block batch {i}");
            }
            let previous_hash = previous.hash().into_iter().rev().collect::<Vec<u8>>();
            if header.prev_block.to_vec() != previous_hash {
                panic!("Discontinuous block at headers batch {i}")
            }
            // THESE WILL NOT WORK FOR TESTNET AS THE DIFFICULTY ALGORITHM IS DIFFERENT
            // Reimplement test for mainnet or update the difficulty checking algorithm

            // if count % 2016 == 0 {
            //     let time_differential = u32::from_le_bytes(previous.timestamp) - u32::from_le_bytes(first_epoch_timestamp);
            //     expected_bits = calculate_new_bits_from_previous(previous.bits, time_differential);
            //     dbg!(hex::encode(expected_bits));
            //     first_epoch_timestamp = header.timestamp;
            // }
            // if header.bits != expected_bits {
            //     panic!("Bad bits at headers batch {i}")
            // }
            previous = header;
            count += 1;
        }
    }
}
