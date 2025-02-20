use std::io::Cursor;

use programming_bitcoin::blocks::block_header::BlockHeader;
// use programming_bitcoin::blocks::utils::calculate_new_bits_from_previous;
use programming_bitcoin::network::get_tip_hash::get_tip_hash;
use programming_bitcoin::network::inventory::Inventory;
use programming_bitcoin::network::messages::block::BlockMessage;
use programming_bitcoin::network::messages::get_data::GetDataMessage;
use programming_bitcoin::network::messages::get_headers::GetHeadersMessage;
use programming_bitcoin::network::messages::headers::HeadersMessage;
use programming_bitcoin::network::messages::version::VersionMessage;
use programming_bitcoin::network::network_envelope::NetworkEnvelope;
use programming_bitcoin::network::network_envelope::{
    MAINNET_NETWORK_MAGIC, TESTNET_NETWORK_MAGIC,
};
use programming_bitcoin::network::network_message::{NetworkMessage, NetworkMessages};
use programming_bitcoin::network::node::Node;
// use programming_bitcoin::transactions;
use programming_bitcoin::transactions::tx_input::TxInput;
use tokio::{ task, time };
use std::time::{SystemTime, UNIX_EPOCH};

use programming_bitcoin::transactions::tx_fetcher::TxFetcher;
use futures::future::join_all;
use std::time::Instant;

pub const PI_TESTNET_NODE_IP: &str = "192.168.2.4";
pub const DEFAULT_TESTNET_PORT: u32 = 18333;

pub const PUBLIC_TESTNET_NODE_IP: &str = "89.117.19.191";

pub static TESTNET_GENESIS_BLOCK_HASH: [u8; 32] = [
    0x43, 0x49, 0x7f, 0xd7, 0xf8, 0x26, 0x95, 0x71, 0x08, 0xf4, 0xa3, 0x0f, 0xd9, 0xce, 0xc3, 0xae,
    0xba, 0x79, 0x97, 0x20, 0x84, 0x9e, 0x0e, 0xad, 0x01, 0xea, 0x33, 0x09, 0x00, 0x00, 0x00, 0x00,
];

pub static TESTNET_GENESIS_RAW_HEADER: [u8; 80] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x3b, 0xa3, 0xed, 0xfd, 0x7a, 0x7b, 0x12, 0xb2, 0x7a, 0xc7, 0x2c, 0x3e,
    0x67, 0x76, 0x8f, 0x61, 0x7f, 0xc8, 0x1b, 0xc3, 0x88, 0x8a, 0x51, 0x32, 0x3a, 0x9f, 0xb8, 0xaa,
    0x4b, 0x1e, 0x5e, 0x4a, 0xda, 0xe5, 0x49, 0x4d, 0xff, 0xff, 0x00, 0x1d, 0x1a, 0xa4, 0xae, 0x18,
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
    let host = PI_TESTNET_NODE_IP;
    // let host = PUBLIC_TESTNET_NODE_IP;
    let port = DEFAULT_TESTNET_PORT;
    let testnet = true;
    let logging = true;
    let mut node = Node::new(host, port, testnet, logging).await.unwrap();

    assert!(Node::handshake(&mut node).await.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_get_headers() {
    // let host = PI_TESTNET_NODE_IP;
    let host = PUBLIC_TESTNET_NODE_IP;
    let port = DEFAULT_TESTNET_PORT;
    let testnet = true;
    let logging = true;
    let mut node = Node::new(host, port, testnet, logging).await.unwrap();

    node.handshake().await.unwrap();

    let tip_hash = get_tip_hash().await.unwrap();
    let getheaders = GetHeadersMessage::new(
        70015,
        1,
        tip_hash,
        Some(TESTNET_GENESIS_BLOCK_HASH.to_vec()),
    );
    node.send(getheaders).await.unwrap();

    let mut headers_received = false;

    while !(headers_received) {
        let received_message: NetworkMessages = node.listen().await.unwrap();

        if let NetworkMessages::Headers(_) = received_message {
            headers_received = true
        }
    }
    assert!(headers_received);
}

#[tokio::test]
#[ignore]
async fn get_validate_headers() {
    let host = PUBLIC_TESTNET_NODE_IP;
    let port = DEFAULT_TESTNET_PORT;
    let testnet = true;
    let logging = true;
    let mut node = Node::new(host, port, testnet, logging).await.unwrap();

    let mut stream: Cursor<Vec<u8>> = Cursor::new(TESTNET_GENESIS_RAW_HEADER.to_vec());
    let mut previous = BlockHeader::parse(&mut stream).unwrap();
    // these are for the difficulty checking (mainnet)
    // let mut first_epoch_timestamp = previous.timestamp;
    // let mut expected_bits = LOWEST_BITS;

    // let mut count: u32 = 1;

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
            // count += 1;
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_get_data() {
    let host = PUBLIC_TESTNET_NODE_IP;
    // let host = PI_TESTNET_NODE_IP;
    let port = DEFAULT_TESTNET_PORT;
    let testnet = true;
    let logging = true;
    let mut node = Node::new(host, port, testnet, logging).await.unwrap();

    node.handshake().await.unwrap();

    /*
    // call getheaders on a single block header hash
    // hash of an arbitrary recent block (height 3604184)
    let start_hash = hex::decode("0000000000000045618af6c594b1e04f9bb753c229b520cb3ca48db24cd92fa2").unwrap();
    let start_hash = start_hash.into_iter().rev().collect::<Vec<u8>>(); // convert from big to little endian
    // hash of an arbitrary recent block (height 3604185)
    let end_hash = hex::decode("00000000000003bc144d301e7951351c02f7fb1dd77e4024fc80397ec5a22fce").unwrap();
    let end_hash = end_hash.into_iter().rev().collect::<Vec<u8>>(); // convert from big to little endian
    let getheaders = GetHeadersMessage::new(70015, 1, start_hash, Some(end_hash));
    // call getdata for each of these headers
    */

    let block_hash =
        hex::decode("00000000000003bc144d301e7951351c02f7fb1dd77e4024fc80397ec5a22fce").unwrap();
    let block_hash = block_hash.into_iter().rev().collect::<Vec<u8>>(); // convert from big to little endian
    let block_hash: [u8; 32] = block_hash.try_into().unwrap();

    // object_type
    // 02 00 00 00 = MSG_BLOCK
    // 02 00 00 40 = MSG_WITNESS_BLOCK

    // currently using MSG_BLOCK (non witness data)
    let inventory = Inventory::new(2, block_hash);
    let inventory_vec = vec![inventory];

    let getdata = GetDataMessage::new(1, inventory_vec);

    node.send(getdata).await.unwrap();

    let mut block_received = false;

    let mut received_message: NetworkMessages;
    let mut block_msg: BlockMessage = BlockMessage::default();

    while !(block_received) {
        received_message = node.listen().await.unwrap();

        if let NetworkMessages::Block(block_message) = received_message {
            block_received = true;
            block_msg = block_message;
        }
    }

    let transactions = block_msg.block.txs.into_iter().enumerate().take(3000);

    let mut p2wpkh_txs: Vec<TxInput> = vec![];

    for (i, tx) in transactions {
        println!("Transaction {}: {}", i, tx.id());

        for input in tx.tx_ins.iter() {
            // guard againt coinbase tx input
            if input.prev_tx_id == [0u8; 32] {
                println!("coinbase tx input - no script type");
                continue;
            }

            let input_clone = input.clone();
            let script_pubkey = task::spawn_blocking(move || input_clone.script_pubkey(testnet))
                .await
                .unwrap();

            if script_pubkey.is_p2wpkh_script_pubkey() {
                p2wpkh_txs.push(input.clone());
                println!("Found P2PKH tx: {}", tx.id());
                println!("{}", p2wpkh_txs[0]);
                return;
            }
        }
    }
}

// Demonstrates concurrently making API calls in batches to fetch transactions
#[tokio::test]
#[ignore]
async fn fetch_transactions() {
    let host = PUBLIC_TESTNET_NODE_IP;
    // let host = PI_TESTNET_NODE_IP;
    let port = DEFAULT_TESTNET_PORT;
    let testnet = true;
    let logging = true;
    let mut node = Node::new(host, port, testnet, logging).await.unwrap();

    node.handshake().await.unwrap();

    let block_hash =
        hex::decode("00000000000003bc144d301e7951351c02f7fb1dd77e4024fc80397ec5a22fce").unwrap();
    let block_hash = block_hash.into_iter().rev().collect::<Vec<u8>>(); // convert from big to little endian
    let block_hash: [u8; 32] = block_hash.try_into().unwrap();

    let inventory = Inventory::new(2, block_hash); // MSG_BLOCK
    let inventory_vec = vec![inventory];

    let getdata = GetDataMessage::new(1, inventory_vec);

    node.send(getdata).await.unwrap();

    let mut block_received = false;

    let mut received_message: NetworkMessages;
    let mut block_msg: BlockMessage = BlockMessage::default();

    while !(block_received) {
        received_message = node.listen().await.unwrap();

        if let NetworkMessages::Block(block_message) = received_message {
            block_received = true;
            block_msg = block_message;
        }
    }

    let transactions = block_msg.block.txs.into_iter().enumerate().take(3000);
    let total_txs: usize = 80;
    let tx_ids: Vec<(usize, String)> = transactions.map(|(i, tx)| (i, tx.id())).take(total_txs).collect();

    let mut handles = vec![];

    for (i, tx_id) in tx_ids {
        println!("TXID {}: {}", i, tx_id);

        // Returns a future/task to then process later
        handles.push(TxFetcher::fetch_tx(tx_id, testnet));
    }

    let start = Instant::now();

    while !handles.is_empty() {
        let chunk_size = 10.min(handles.len());
        let handles_subset: Vec<_> = handles.drain(handles.len() - chunk_size..).collect();

        // Wait for all results in this batch
        let results = join_all(handles_subset).await;
        
        // Process all results in this batch
        for result in results {
            let tx = result.unwrap();
            println!("Successfully fetched tx: {}", tx.id());
        }

        println!("batch processed");

        time::sleep(time::Duration::from_millis(1750)).await
    }
    
    let duration = start.elapsed();

    println!("Fetched {} transactions in {:?}", total_txs, duration);
}
