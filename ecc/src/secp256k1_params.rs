use primitive_types::U256;

const P: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";
const N: &str = "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
const A: &str = "0x0";
const B: &str = "0x7";

// Gx
// Gy

pub struct S256Params;

impl S256Params {
    pub fn P() -> U256 {
        U256::from_str_radix(P, 16).unwrap()
    }

    pub fn N() -> U256 {
        U256::from_str_radix(N, 16).unwrap()
    }

    pub fn A() -> U256 {
        U256::from_str_radix(A, 16).unwrap()
    }

    pub fn B() -> U256 {
        U256::from_str_radix(B, 16).unwrap()
    }
}