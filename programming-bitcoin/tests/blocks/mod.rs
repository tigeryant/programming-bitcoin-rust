use std::io::Cursor;

use programming_bitcoin::blocks::block::Block;

#[test]
fn parse_block() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    assert!(Block::parse(&mut stream).is_ok());
}

#[test]
fn new_block() {
    let version: [u8; 4] = hex::decode("02000020").unwrap().try_into().unwrap();
    let prev_block: [u8; 32] = hex::decode("8ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd000000000000000000").unwrap().try_into().unwrap();
    let merkle_root: [u8; 32] = hex::decode("5b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be").unwrap().try_into().unwrap();
    let timestamp: [u8; 4] = hex::decode("02000020").unwrap().try_into().unwrap();
    let bits: [u8; 4] = hex::decode("02000020").unwrap().try_into().unwrap();
    let nonce: [u8; 4] = hex::decode("02000020").unwrap().try_into().unwrap();
    let block = Block::new(version, prev_block, merkle_root, timestamp, bits, nonce);

    assert_eq!(block.version, version);
    assert_eq!(block.prev_block, prev_block);
    assert_eq!(block.merkle_root, merkle_root);
    assert_eq!(block.timestamp, timestamp);
    assert_eq!(block.bits, bits);
    assert_eq!(block.nonce, nonce);
}
