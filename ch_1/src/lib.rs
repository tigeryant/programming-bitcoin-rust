use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug)]
pub struct FieldElement {
    num: u32,
    prime: u32
}

impl FieldElement {
    // do we need to change this to allow for negative numbers?
    pub fn new (num: u32, prime: u32) -> Self {
        if num >= prime {
            panic!("num not in range 0 to {}", prime - 1);
        }
        FieldElement {
            num,
            prime
        }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
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

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
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

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
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

pub trait Pow {
    type Output;
    fn pow(self, exponent: u32) -> Self::Output;
}

impl Pow for FieldElement {
    type Output = Self;

    fn pow(self, exponent: u32) -> Self::Output {
        let num = self.num.pow(exponent) % self.prime;
        FieldElement {
            num,
            prime: self.prime, 
        }
    }
}
