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

// part of ecc?
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

// part of ecc?
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
fn test_point_from_sec() {
    // uncompressed sec, as seen from the prepended 0x04
    let raw_sec = hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
    Point::point_from_sec(raw_sec);
    // TODO add test cases for uncompressed - 0x02 and 0x03
}
