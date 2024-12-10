use programming_bitcoin::utils::{base58::decode_base58, rng, hash160::hash160};

#[test]
fn random_u256() {
    let random_number = rng::get_random_u256();
    println!("Random U256: {random_number}");
}

#[test]
fn test_decode_base58() {
    // address 1
    let decoded1 = decode_base58("mzx5YhAH9kNHtcN481u6WkjeHjYtVeKVh2");
    let decoded_hex1 = hex::encode(decoded1.as_ref().unwrap());
    println!("Decoded hex 1: {}", decoded_hex1);
    assert!(decoded1.is_ok());
    let expected_hash1 = String::from("d52ad7ca9b3d096a38e752c2018e6fbc40cdf26f");
    assert_eq!(expected_hash1, decoded_hex1);

    // address 2
    let decoded2 = decode_base58("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
    let decoded_hex2 = hex::encode(decoded2.as_ref().unwrap());
    println!("Decoded hex 2: {}", decoded_hex2);
    assert!(decoded2.is_ok());
    let expected_hash2 = String::from("62e907b15cbf27d5425399ebf6f0fb50ebb88f18");
    assert_eq!(expected_hash2, decoded_hex2);
}

#[test]
fn test_hash160() {
    let input = "0208d9652010687a9125f621e3687554bf14c46a7acf26ed80453ad8ce95955668";
    let input_bytes = hex::decode(input).unwrap();
    let result = hash160(&input_bytes);
    println!("Hash160 result: {}", hex::encode(result));
}
