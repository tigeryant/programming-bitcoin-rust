use std::io::Cursor;

use programming_bitcoin::blocks::block::Block;

#[test]
fn parse_block() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    assert!(Block::parse(&mut stream).is_ok());
}

// #[test]
// fn new_block() {

// }
