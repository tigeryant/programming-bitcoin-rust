use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug)]
pub struct FieldElement {
    num: u128,
    prime: u128
}

impl FieldElement {
    // update to allow for negative numbers?
    pub fn new (num: u128, prime: u128) -> Self {
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
        let num = ((other.num.pow((self.prime - 2).try_into().unwrap())) * self.num) % self.prime;
        FieldElement {
            num,
            prime : self.prime
        }
    }
}

pub trait Pow {
    type Output;
    fn pow(self, exponent: u128) -> Self::Output;
}

impl Pow for &FieldElement {
    type Output = FieldElement;

    fn pow(self, exponent: u128) -> Self::Output {
        let num = self.num.pow(exponent.try_into().unwrap()) % self.prime;
        FieldElement {
            num,
            prime: self.prime, 
        }
    }
}
