use crate::field_element::FieldElement;
use crate::secp256k1_params::S256Params;
use primitive_types::U256;

#[derive(Debug)]
pub struct S256FieldElement;

impl S256FieldElement {
    // Returns a field element without having to pass the specific p for secp256k1
    pub fn new_s256_field(num: U256) -> FieldElement {
        FieldElement::new(num, S256Params::p())
    }
}
