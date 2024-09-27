pub mod utilities;

use primitive_types::U256;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use utilities::*;

#[derive(Debug)]
pub struct FieldElement {
    num: U256,
    prime: U256
}

impl FieldElement {
    // update to allow for negative numbers?
    pub fn new (num: U256, prime: U256) -> Self {
        if num >= prime {
            panic!("num not in range 0 to {}", prime - 1);
        }
        FieldElement {
            num,
            prime
        }
    }
}

impl PartialEq for &FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

// TODO update to handle overflows properly
impl Add for &FieldElement {
    type Output = FieldElement;

    fn add(self, other: Self) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different fields")
        }
        let num = (self.num + other.num) % self.prime;
        FieldElement {
            num,
            prime : self.prime
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
            prime : self.prime
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
            prime : self.prime
        }
    }
}

impl Div for &FieldElement {
    type Output = FieldElement;

    fn div(self, other: Self) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different fields")
        }
        let num = (self.num * (other.num.pow(self.prime - 2))) % self.prime;
        FieldElement {
            num,
            prime : self.prime
        }
    }
}

pub trait Pow {
    type Output;
    fn pow(self, exponent: i128) -> Self::Output;
}

impl Pow for &FieldElement {
    type Output = FieldElement;

    fn pow(self, mut exponent: i128,) -> Self::Output {
        let mut base = self.num % self.prime;
        let prime = self.prime;
        
        if exponent < 0 {
            base = mod_inverse(base, prime).expect("Inverse does not exist");
            exponent = -exponent;
        }

        let mut result = U256::one();

        while exponent != 0 {
            if exponent % 2 == 1 {
                result = (result * base) % prime;
            }
            exponent /= 2;
            base = (base * base) % prime;
        }

        FieldElement {
            num: result,
            prime
        }
    }
}
