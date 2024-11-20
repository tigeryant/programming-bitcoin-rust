use std::io::Cursor;

use programming_bitcoin::transactions::tx::Tx;
use ecc::{field_element::*, mod_exp::mod_exp, private_key::PrivateKey, secp256k1_params::S256Params};
use primitive_types::U256;
use ecc::point::Point;
use programming_bitcoin::utils::rng;
use programming_bitcoin::ecc;

#[test]
fn a_equals_itself() {
    let element_a = FieldElement::new(U256::from(3), U256::from(13));
    assert!(&element_a == &element_a);
}

#[test]
fn a_not_equals_b() {
    let element_a = FieldElement::new(U256::from(3), U256::from(13));
    let element_b = FieldElement::new(U256::from(12), U256::from(13));
    assert!(!(&element_a == &element_b));
}

#[test]
fn a_add_b_eq_c() {
    let element_a = FieldElement::new(U256::from(7), U256::from(13));
    let element_b = FieldElement::new(U256::from(12), U256::from(13));
    let element_c = FieldElement::new(U256::from(6), U256::from(13));
    assert!(&(&element_a + &element_b) == &element_c);
}

#[test]
fn a_sub_b_eq_c() {
    let element_a = FieldElement::new(U256::from(12), U256::from(13));
    let element_b = FieldElement::new(U256::from(7), U256::from(13));
    let element_c = FieldElement::new(U256::from(5), U256::from(13));
    assert_eq!(&(&element_a - &element_b), &element_c);
}

#[test]
fn a_mul_b_eq_c() {
    let element_a = FieldElement::new(U256::from(3), U256::from(13));
    let element_b = FieldElement::new(U256::from(12), U256::from(13));
    let element_c = FieldElement::new(U256::from(10), U256::from(13));
    assert_eq!(&(&element_a * &element_b), &element_c);
}

#[test]
fn a_div_b_eq_c() {
    let element_a = FieldElement::new(U256::from(7), U256::from(19));
    let element_b = FieldElement::new(U256::from(5), U256::from(19));
    let element_c = FieldElement::new(U256::from(9), U256::from(19));
    assert_eq!(&(&element_a / &element_b), &element_c);
}

#[test]
fn a_pow_3_eq_b() {
    let element_a = FieldElement::new(U256::from(3), U256::from(13));
    let element_b = FieldElement::new(U256::from(1), U256::from(13));
    assert_eq!(&element_a.pow(U256::from(3)), &element_b);
}

#[test]
fn test_on_curve() {
    let zero = U256::zero();
    let prime = U256::from(223);
    let a = FieldElement::new(zero, prime);
    let b = FieldElement::new(U256::from(7), prime);

    // for a valid point, assert is ok. for an invalid point, assert they panic
    let x = FieldElement::new(U256::from(192), prime);
    let y = FieldElement::new(U256::from(105), prime);
    let valid_point = Point::new(Some(x), Some(y), a, b);
    dbg!(valid_point);
}

#[test]
fn p_parameter() {
    let output = S256Params::p();
    let expected = U256::from_str_radix("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap();
    dbg!(output);
    dbg!(expected);
    assert_eq!(output, expected);
}

#[test]
fn gx_cubed() {
    let base = S256Params::gx();
    let exp = U256::from_str_radix("0x3", 16).unwrap();
    let modulus = S256Params::p();

    let expected = U256::from_str_radix("0x4866d6a5ab41ab2c6bcc57ccd3735da5f16f80a548e5e20a44e4e9b8118c26eb", 16).unwrap();
    let output = mod_exp(base, exp, modulus);
    assert_eq!(output, expected);
}

#[test]
fn new_insecure_address() {
    let secret = U256::from(12345); // Use cryptographically random number generation in production
    let private_key = PrivateKey::new(secret);

    // Get public key point from private key
    let public_key = private_key.point();

    // Get testnet address (compressed format)
    let address = public_key.address(true, true); // compressed=true, testnet=true
    let expected = "mhSfwmFGmD5KcJxUfVdxrfe55uCqkptc6a";
    println!("{address}");
    assert_eq!(expected, address);
}

#[test]
fn new_address() {
    let secret = rng::get_random_u256();
    // WARNING: This prints the private key to the console
    println!("Secret (private key) in hex: {:#x}", secret);
    let private_key = PrivateKey::new(secret);

    // Get public key point from private key
    let public_key = private_key.point();

    // Get testnet address (compressed format)
    let address = public_key.address(true, true);
    println!("Address: {address}");
}

#[test]
fn random_u256() {
    let random_number = rng::get_random_u256();
    println!("Random U256: {random_number}");
}

// add tests here for parsing the individual components of the tx - version, inputs, outputs, locktime (and testnet?)
#[test]
fn tx_parse_tx() { // run with --nocapture to see the debug
    let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, true);
    dbg!(tx);
}
