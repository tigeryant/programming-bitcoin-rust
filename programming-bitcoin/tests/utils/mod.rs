use programming_bitcoin::utils::{base58::decode_base58, hash160::hash160, hash256::hash256, rng};

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

#[test]
fn test_hash256_rev() {
    let input = "020000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff26033b093704ffa07467537069646572506f6f6c2f312f398032a80103b606ed19000000000000ffffffff05220200000000000022512028202d4b19bfed17d9f7f9528e4b0433c9b78399bbaf81aa4df4549e8eb527f68598c82100000000160014f65616071e14d79e45b30c5e968ae40e6ecce95f0000000000000000266a24aa21a9edd8c74c43129f200d0bedbe6fe75a161f28ca6b8ce9fa48fb0752f8ec950907b400000000000000002f6a2d434f52450164db24a662e20bbdf72d1cc6e973dbb2d12897d596a6689031f48a857d344e1a42fdb272bb15d6210000000000000000126a10455853415401120f080304111f1200130120000000000000000000000000000000000000000000000000000000000000000000000000";
    let input_bytes = hex::decode(input).unwrap();
    let result = hash256(&input_bytes);
    // hash.into_iter().rev().collect() // reverse to get little endian
    println!("Hash256 result (BE): {}", hex::encode(result));
}
