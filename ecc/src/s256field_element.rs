use crate::field_element::FieldElement;
use primitive_types::U256;

pub struct S256FieldElement;

impl S256FieldElement {
    // Returns a field element without having to pass the specific p for secp256k1
    pub fn new_s256_field(num: U256) -> FieldElement {
        FieldElement::new(num, get_prime())
    }
}

fn get_prime() -> U256 {
    U256::from_str_radix("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap()
}
