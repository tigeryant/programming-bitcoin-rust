use crate::ecc::mod_exp::mod_exp;
use num_bigint::BigUint;
use primitive_types::U256;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct FieldElement {
    num: U256,
    prime: U256,
}

impl FieldElement {
    pub fn new(num: U256, prime: U256) -> Self {
        if num >= prime {
            panic!("num not in range 0 to {}", prime - 1);
        }
        FieldElement { num, prime }
    }

    pub fn mod_inverse(&self) -> FieldElement {
        // Fermat's Little Theorem: a^(p-1) ≡ 1 (mod p), so a^(p-2) is the inverse
        self.pow(self.prime - 2)
    }

    pub fn is_zero(&self) -> bool {
        self.num == U256::from(0)
    }

    pub fn num(&self) -> U256 {
        self.num
    }

    pub fn sqrt(&self) -> Self {
        let p = self.prime;
        let exp = (p + U256::one()) / U256::from(4);
        let num = self.num.pow(exp);
        Self::new(num, self.prime)
    }
}

impl PartialEq for &FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FieldElement_num_{:064x}_prime_{:064x}",
            self.num, self.prime
        )
    }
}

impl Add for &FieldElement {
    type Output = FieldElement;

    fn add(self, other: Self) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different fields")
        }
        let num = (self.num + other.num) % self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Sub for &FieldElement {
    type Output = FieldElement;

    fn sub(self, other: Self) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different fields")
        }
        
        // Convert to BigUint
        let a = BigUint::from_bytes_be(&self.num.to_big_endian());
        let b = BigUint::from_bytes_be(&other.num.to_big_endian());
        let p = BigUint::from_bytes_be(&self.prime.to_big_endian());
        
        // Perform subtraction and modulo
        let result = if a >= b {
            (a - b) % p
        } else {
            (p.clone() + a - b) % p
        };
        
        // Convert back to U256
        let num = U256::from_big_endian(&result.to_bytes_be());
        
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for &FieldElement {
    type Output = FieldElement;

    fn mul(self, other: Self) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different fields")
        }
        
        // Convert to BigUint
        let a = BigUint::from_bytes_be(&self.num.to_big_endian());
        let b = BigUint::from_bytes_be(&other.num.to_big_endian());
        let p = BigUint::from_bytes_be(&self.prime.to_big_endian());
        
        // Perform multiplication and modulo
        let result = (a * b) % p;
        
        // Convert back to U256
        let num = U256::from_big_endian(&result.to_bytes_be());
        
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul<u32> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, other: u32) -> FieldElement {
        let p = BigUint::from_bytes_be(&self.prime.to_big_endian());
        let a = BigUint::from_bytes_be(&self.num.to_big_endian());
        let b = BigUint::from(other);
        
        let result = (a * b) % p;
        let num = U256::from_big_endian(&result.to_bytes_be());
        
        FieldElement { num, prime: self.prime }
    }
}

impl Div for &FieldElement {
    type Output = FieldElement;

    fn div(self, other: Self) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different fields");
        }
        let inverse = other.mod_inverse();
        
        // Convert to BigUint
        let a = BigUint::from_bytes_be(&self.num.to_big_endian());
        let b = BigUint::from_bytes_be(&inverse.num.to_big_endian());
        let p = BigUint::from_bytes_be(&self.prime.to_big_endian());
        
        // Perform multiplication and modulo
        let result = (a * b) % p;
        
        // Convert back to U256
        let num = U256::from_big_endian(&result.to_bytes_be());
        
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}


pub trait Pow {
    type Output;
    fn pow(self, exp: U256) -> Self::Output;
}

impl Pow for &FieldElement {
    type Output = FieldElement;

    fn pow(self, exp: U256) -> FieldElement {
        let num = mod_exp(self.num, exp, self.prime);
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}
