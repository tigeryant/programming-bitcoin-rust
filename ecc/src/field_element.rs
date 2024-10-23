use crate::mod_exp::mod_exp;
use primitive_types::U256;
use std::ops::{Add, Div, Mul, Sub};
use std::fmt;

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
}

impl PartialEq for &FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_num_{:064x}_prime_{:064x}", self.num, self.prime)
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
        let num = (self.num - other.num) % self.prime;
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
        let num = (self.num * other.num) % self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul<u32> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, other: u32) -> FieldElement {
        let p = self.prime;
        let m = (self.num * other) % p;

        FieldElement {
            num: m,
            prime: p
        }
    }
}

impl Div for &FieldElement {
    type Output = FieldElement;

    fn div(self, other: Self) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different fields");
        }
        // Division is defined as multiplication by the inverse
        let inverse = other.mod_inverse();
        let num = (self.num * inverse.num) % self.prime;
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
