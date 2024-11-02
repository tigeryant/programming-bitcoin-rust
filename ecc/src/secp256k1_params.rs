use primitive_types::U256;
use crate::s256point::S256Point;
use crate::point::Point;

const P: &str = "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";
const N: &str = "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
const A: &str = "0x0";
const B: &str = "0x7";

const GX: &str = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
const GY: &str = "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

pub struct S256Params;

impl S256Params {
    pub fn p() -> U256 {
        U256::from_str_radix(P, 16).unwrap()
    }

    pub fn n() -> U256 {
        U256::from_str_radix(N, 16).unwrap()
    }

    pub fn a() -> U256 {
        U256::from_str_radix(A, 16).unwrap()
    }

    pub fn b() -> U256 {
        U256::from_str_radix(B, 16).unwrap()
    }

    pub fn g() -> Point {
        let x = U256::from_str_radix(GX, 16).unwrap();
        let y = U256::from_str_radix(GY, 16).unwrap();
        S256Point::new_s256_point(Some(x), Some(y))
    }

    pub fn gx() -> U256 {
        U256::from_str_radix(GX, 16).unwrap()
    }

    pub fn gy() -> U256 {
        U256::from_str_radix(GY, 16).unwrap()
    }
}
