use crate::field_element::*;
use crate::s256field_element::S256FieldElement;
use primitive_types::U256;
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Self {
        match (x, y) {
            (Some(x), Some(y)) => {
                let x_cubed = x.pow(U256::from(3));
                let ax = &a * &x;
                let right_side = &(&x_cubed + &ax) + &b;
                let y_squared = y.pow(U256::from(2));

                if &y_squared != &right_side {
                    panic!("({x:?}, {y:?}) is not on the curve")
                }
                Self {
                    x: Some(x),
                    y: Some(y),
                    a,
                    b,
                }
            }
            (None, None) => Self {
                x: None,
                y: None,
                a,
                b,
            },
            _ => {
                panic!("Invalid parameters to Point::new()")
            }
        }
    }

    // Returns the point at infinity
    pub fn new_infinity(&self) -> Self {
        Self {
            x: None,
            y: None,
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }

    // Move these into the S256Point struct - the Point struct should remain general purpose
    // Creates a new point on the secp256k1 curve
    pub fn new_secp256k1(x: FieldElement, y: FieldElement) -> Self {
        // change a and b to constants or methods
        let a = S256FieldElement::new_s256_field(U256::from(0));
        let b = S256FieldElement::new_s256_field(U256::from(7));
        Self::new(Some(x), Some(y), a, b)
    }

    // Get the point at infinity on the secp256k1 curve
    pub fn new_secp256k1_infinity() -> Self {
        // change a and b to constants or methods
        let a = S256FieldElement::new_s256_field(U256::from(0));
        let b = S256FieldElement::new_s256_field(U256::from(7));
        Self::new(None, None, a, b)
    }
}

impl PartialEq for &Point {
    fn eq(&self, other: &Self) -> bool {
        let x_eq = match (&self.x, &other.x) {
            (Some(x1), Some(x2)) => x1 == x2,
            (None, None) => true,
            _ => false,
        };

        let y_eq = match (&self.y, &other.y) {
            (Some(y1), Some(y2)) => y1 == y2,
            (None, None) => true,
            _ => false,
        };

        x_eq && y_eq && &self.a == &other.a && &self.b == &other.b
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        if (&self.a != &other.a) | (&self.b != &other.b) {
            panic!("Points {self:?}, {other:?} are not on the same curve.");
        }
        if self.x.is_none() {
            // self is point at infinity or additive identity - create an abstraction for this (is_infinity() method)
            return other.clone();
        };
        if other.x.is_none() {
            // other is point at infinity or additive identity - create an abstraction for this (is_infinity() method)
            return self.clone();
        };

        // We can unwrap these as we know they're not none
        let x1 = &self.x.clone().unwrap();
        let y1 = &self.y.clone().unwrap();
        let x2 = &other.x.clone().unwrap();
        let y2 = &other.y.clone().unwrap();

        // Handles the vertical line case
        if x1 == x2 && y1 != y2 {
            return self.new_infinity();
        };

        // Point addition for when x1 is not equal to x2
        if x1 != x2 {
            let slope = &(y2 - y1) / &(x2 - x1);
            let x3 = &(&slope.pow(U256::from(2)) - x1) - x2;
            let y3 = &(&slope * &(x1 - &x3)) - y1;

            return Point {
                x: Some(x3),
                y: Some(y3),
                a: self.a.clone(),
                b: self.b.clone(),
            };
        };

        // Point addition for when p1 == p2
        // The slope is (3 * x1^2 + a) / (2 * y1)
        // x3 = slope^^2 - 2x1
        // y3 = slope(x1 - x3) - y1
        if self == other {
            // Handling case when tangent line is vertical
            if y1.is_zero() {
                return self.new_infinity();
            }

            let term1 = &(x1.pow(U256::from(2))) * 3u32;
            let term2 = self.a.clone();
            let term3 = y1 * 2;

            let slope = &(&term1 + &term2) / &term3;

            let x3 = &(slope.pow(U256::from(2))) - &(x1 * 2u32);
            let y3 = &(&slope * &(x1 - &x3)) - y1;

            return Point {
                x: Some(x3),
                y: Some(y3),
                a: self.a.clone(),
                b: self.b.clone(),
            };
        };

        unreachable!();
    }
}

impl Mul<U256> for &Point {
    type Output = Point;

    // Scalar multiplication using binary expansion
    fn mul(self, coefficient: U256) -> Self::Output {
        let mut coef = coefficient;
        // current represents the point that’s at the current bit. The first
        // time through the loop it represents 1 × self; the second time it will
        // be 2 × self, the third time 4 × self, then 8 × self, and so on. We
        // double the point each time. In binary the coefficients are 1, 10,
        // 100, 1000, 10000, etc.
        let mut current = self.clone();
        // We start the result at 0, or the point at infinity.
        let mut result = self.new_infinity();

        while coef > U256::from(0u32) {
            // We are looking at whether the rightmost bit is a 1. If it is,
            // then we add the value of the current bit.
            if coef & U256::from(1u32) == U256::from(1u32) {
                result = &result + &current.clone();
            }
            // We need to double the point until we’re past how big the
            // coefficient can be.
            current = &current.clone() + &current.clone();
            // We bit-shift the coefficient to the right.
            coef >>= 1;
        }

        result
    }
}
