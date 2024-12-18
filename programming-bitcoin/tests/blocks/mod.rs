use std::io::Cursor;

use primitive_types::U256;
use programming_bitcoin::blocks::{block::Block, utils::bits_to_target};

#[test]
fn parse_block() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    assert!(Block::parse(&mut stream).is_ok());
}

#[test]
fn new_block() {
    // All fields in little endian
    let version: [u8; 4] = hex::decode("02000020").unwrap().try_into().unwrap();
    let prev_block: [u8; 32] = hex::decode("8ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd000000000000000000").unwrap().try_into().unwrap();
    let merkle_root: [u8; 32] = hex::decode("5b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be").unwrap().try_into().unwrap();
    let timestamp: [u8; 4] = hex::decode("1e77a759").unwrap().try_into().unwrap();
    let bits: [u8; 4] = hex::decode("e93c0118").unwrap().try_into().unwrap();
    let nonce: [u8; 4] = hex::decode("a4ffd71d").unwrap().try_into().unwrap();
    let block = Block::new(version, prev_block, merkle_root, timestamp, bits, nonce);

    assert_eq!(block.version, version);
    assert_eq!(block.prev_block, prev_block);
    assert_eq!(block.merkle_root, merkle_root);
    assert_eq!(block.timestamp, timestamp);
    assert_eq!(block.bits, bits);
    assert_eq!(block.nonce, nonce);
}

#[test]
fn serialize_block() {
    // All fields in little endian
    let version: [u8; 4] = hex::decode("02000020").unwrap().try_into().unwrap();
    let prev_block: [u8; 32] = hex::decode("8ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd000000000000000000").unwrap().try_into().unwrap();
    let merkle_root: [u8; 32] = hex::decode("5b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be").unwrap().try_into().unwrap();
    let timestamp: [u8; 4] = hex::decode("1e77a759").unwrap().try_into().unwrap();
    let bits: [u8; 4] = hex::decode("e93c0118").unwrap().try_into().unwrap();
    let nonce: [u8; 4] = hex::decode("a4ffd71d").unwrap().try_into().unwrap();

    let block = Block::new(version, prev_block, merkle_root, timestamp, bits, nonce);

    let output_bytes= block.serialize();
    let expected = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();

    assert_eq!(output_bytes, expected);
}

#[test]
fn hash_block() {
    // All fields in little endian
    let version: [u8; 4] = hex::decode("02000020").unwrap().try_into().unwrap();
    let prev_block: [u8; 32] = hex::decode("8ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd000000000000000000").unwrap().try_into().unwrap();
    let merkle_root: [u8; 32] = hex::decode("5b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be").unwrap().try_into().unwrap();
    let timestamp: [u8; 4] = hex::decode("1e77a759").unwrap().try_into().unwrap();
    let bits: [u8; 4] = hex::decode("e93c0118").unwrap().try_into().unwrap();
    let nonce: [u8; 4] = hex::decode("a4ffd71d").unwrap().try_into().unwrap();

    let block = Block::new(version, prev_block, merkle_root, timestamp, bits, nonce);

    let output_hash = hex::encode(block.hash());
    let expected = String::from("0000000000000000007e9e4c586439b0cdbe13b1370bdd9435d76a644d047523");
    assert_eq!(output_hash, expected);
}

#[test]
fn bip9() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    let block = Block::parse(&mut stream).unwrap();

    assert!(block.bip9());
}

#[test]
fn bip91() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    let block = Block::parse(&mut stream).unwrap();

    assert!(!block.bip91());
}

#[test]
fn bip141() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    let block = Block::parse(&mut stream).unwrap();

    assert!(block.bip141());
}

#[test]
fn test_bits_to_target() {
    let bits: [u8; 4] = hex::decode("e93c0118").unwrap().try_into().unwrap();
    let output = bits_to_target(bits);
    let expected = U256::from_str_radix("0000000000000000013ce9000000000000000000000000000000000000000000", 16).unwrap();
    assert_eq!(output, expected);
}
