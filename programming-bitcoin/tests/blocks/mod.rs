use std::io::Cursor;

use primitive_types::U256;
use programming_bitcoin::blocks::{block_header::BlockHeader, utils::{bits_to_target, calculate_new_bits, target_to_bits, TWO_WEEKS}};

#[test]
fn parse_block() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    assert!(BlockHeader::parse(&mut stream).is_ok());
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
    let block = BlockHeader::new(version, prev_block, merkle_root, timestamp, bits, nonce);

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

    let block = BlockHeader::new(version, prev_block, merkle_root, timestamp, bits, nonce);

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

    let block = BlockHeader::new(version, prev_block, merkle_root, timestamp, bits, nonce);

    let output_hash = hex::encode(block.hash());
    let expected = String::from("0000000000000000007e9e4c586439b0cdbe13b1370bdd9435d76a644d047523");
    assert_eq!(output_hash, expected);
}

#[test]
fn bip9() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    let block = BlockHeader::parse(&mut stream).unwrap();

    assert!(block.bip9());
}

#[test]
fn bip91() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    let block = BlockHeader::parse(&mut stream).unwrap();

    assert!(!block.bip91());
}

#[test]
fn bip141() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    let block = BlockHeader::parse(&mut stream).unwrap();

    assert!(block.bip141());
}

#[test]
fn test_bits_to_target() {
    let bits: [u8; 4] = hex::decode("e93c0118").unwrap().try_into().unwrap();
    let output = bits_to_target(bits); // output is in big endian
    let expected = U256::from_str_radix("0000000000000000013ce9000000000000000000000000000000000000000000", 16).unwrap(); // big endian
    assert_eq!(output, expected);
}

#[test]
fn test_difficulty() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    let block = BlockHeader::parse(&mut stream).unwrap();
    let output = block.difficulty();
    let expected = 888171856257.3206;
    let epsilon = 1.0;
    assert!((output - expected).abs() < epsilon);
}

#[test]
fn test_check_pow() {
    let raw_block = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_block);
    let block = BlockHeader::parse(&mut stream).unwrap();
    assert!(block.check_pow());
}

#[test]
// #[ignore]
fn test_difficulty_adjustment() {
    // from exercise 12
    let last_block = hex::decode("02000020f1472d9db4b563c35f97c428ac903f23b7fc055d1cfc26000000000000000000b3f449fcbe1bc4cfbcb8283a0d2c037f961a3fdf2b8bedc144973735eea707e1264258597e8b0118e5f00474").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(last_block);
    let last_block = BlockHeader::parse(&mut stream).unwrap();
    
    // from exercise 12
    let first_block = hex::decode("000000203471101bbda3fe307664b3283a9ef0e97d9a38a7eacd8800000000000000000010c8aba8479bbaa5e0848152fd3c2289ca50e1c3e58c9a4faaafbdf5803c5448ddb845597e8b0118e43a81d3").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(first_block);
    let first_block = BlockHeader::parse(&mut stream).unwrap();

    let last_timestamp = u32::from_le_bytes(last_block.timestamp);
    let first_timestamp = u32::from_le_bytes(first_block.timestamp);
    dbg!(last_timestamp);
    dbg!(first_timestamp);
    let mut time_differential = last_timestamp - first_timestamp;

    if time_differential > TWO_WEEKS * 4 {
        time_differential = TWO_WEEKS * 4;
    } else if time_differential < TWO_WEEKS / 4 {
        time_differential = TWO_WEEKS / 4
    }

    let new_target = last_block.target() * time_differential / TWO_WEEKS;
    // from p175 - incorrect
    // let expected_target = U256::from_str_radix("0000000000000000007615000000000000000000000000000000000000000000", 16).unwrap();
    let expected_target = U256::from_str_radix("0000000000000000018d30aa2cdc67600f9a9342cdc67600f9a9342cdc67600f", 16).unwrap();
    let formatted_new_target = format!("{:064x}", new_target);
    let formatted_expected_target = format!("{:064x}", expected_target);
    println!("{}", formatted_new_target);
    println!("{}", formatted_expected_target);
    assert_eq!(new_target, expected_target);
}

#[test]
fn test_target_to_bits() {
    let target = U256::from_str_radix("0000000000000000013ce9000000000000000000000000000000000000000000", 16).unwrap(); // big endian
    let output_bits = target_to_bits(target); // produces little endian
    println!("output bits: {}", hex::encode(output_bits));
    let expected_bits: [u8; 4] = hex::decode("e93c0118").unwrap().try_into().unwrap(); // little endian
    // let expected_bits: [u8; 4] = hex::decode("18013ce9").unwrap().try_into().unwrap(); // big endian
    println!("expected bits: {}", hex::encode(expected_bits));
    assert_eq!(output_bits, expected_bits);
}

#[test]
fn test_calculate_new_bits() {
    let first_block = hex::decode("000000203471101bbda3fe307664b3283a9ef0e97d9a38a7eacd8800000000000000000010c8aba8479bbaa5e0848152fd3c2289ca50e1c3e58c9a4faaafbdf5803c5448ddb845597e8b0118e43a81d3").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(first_block);
    let first_block = BlockHeader::parse(&mut stream).unwrap();
    dbg!(hex::encode(first_block.hash())); // 471744

    let last_block = hex::decode("02000020f1472d9db4b563c35f97c428ac903f23b7fc055d1cfc26000000000000000000b3f449fcbe1bc4cfbcb8283a0d2c037f961a3fdf2b8bedc144973735eea707e1264258597e8b0118e5f00474").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(last_block);
    let last_block = BlockHeader::parse(&mut stream).unwrap();
    dbg!(hex::encode(last_block.hash())); // 473759

    let new_bits = calculate_new_bits(first_block, last_block); // 18018d30
    dbg!(hex::encode(new_bits)); // new bits in little endian
    // expected (little endian): 308d0118
    let expected_bits: [u8; 4] = hex::decode("308d0118").unwrap().try_into().unwrap(); // little endian
    // let expected_bits: [u8; 4] = hex::decode("18018d30").unwrap().try_into().unwrap(); // big endian (as seen on explorer)
    dbg!(hex::encode(expected_bits));
    assert_eq!(new_bits, expected_bits);
}
