use crate::point::Point;
use crate::s256field_element::S256FieldElement;
use crate::secp256k1_params::S256Params;
use primitive_types::U256;

#[derive(Debug)]
pub struct S256Point;

impl S256Point {
    pub fn new_s256_point(x: Option<U256>, y: Option<U256>) -> Point {
        let a = S256FieldElement::new_s256_field(S256Params::a());
        let b = S256FieldElement::new_s256_field(S256Params::b());

        match x {
            Some(x) => {
                let x = S256FieldElement::new_s256_field(x);   
                let y = S256FieldElement::new_s256_field(y.unwrap());  
                Point::new(Some(x), Some(y), a, b)
            },
            None => Point::new(None, None, a, b)
        }
    }

    pub fn new_s256_infinity() -> Point {
        Self::new_s256_point(None, None)
    }

    // Multiplication by binary expansion. Mods the coefficient by N for efficiency
    pub fn multiply(point: &Point, coefficient: U256) -> Point {
        let mut coef = coefficient % S256Params::n();
        let mut current = point.clone();
        let mut result = point.new_infinity();

        while coef > U256::from(0u32) {
            if coef & U256::from(1u32) == U256::from(1u32) {
                result = &result + &current;
            }
            current = &current + &current;
            coef >>= 1;
        }

        result
    }
}

