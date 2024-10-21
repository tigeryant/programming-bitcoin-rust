use num_bigint::{BigInt, ToBigInt};
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Point {
    x: Option<BigInt>,
    y: Option<BigInt>,
    a: BigInt,
    b: BigInt,
}

impl Point {
    pub fn new(x: Option<BigInt>, y: Option<BigInt>, a: BigInt, b: BigInt) -> Self {
        match (x, y) {
            (Some(x), Some(y)) => {
                if y.pow(2) != x.pow(3) + &a * &x + &b {
                    panic!("({x}, {y}) is not on the curve")
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
}

impl PartialEq for &Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

// implement point addition (Add)
impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        if (self.a != other.a) | (self.b != other.b) {
            panic!("Points {self:?}, {other:?} are not on the same curve.");
        }
        if self.x.is_none() {
            // self is point at infinity or additive identity
            return other.clone();
        }
        if other.x.is_none() {
            // other is point at infinity or additive identity
            return self.clone();
        }
        // if we add more if statements here, consider using a match statement instead
        if (self.x == other.x) && (self.y != other.y) {
            return Point {
                x: None,
                y: None,
                a: self.a.clone(),
                b: self.b.clone(),
            };
        }
        // coding point addition for when x != x
        if self.x != other.x {
            // consider defining these in a scope where they are available to the next if block, and wrap them in Some() too - maybe not possible as they're defined differently below
            // check what methods you can change to normal operators
            let y2 = other.y.clone().unwrap();
            let y1 = &self.y.clone().unwrap();
            let x2 = other.x.clone().unwrap();
            let x1 = &self.x.clone().unwrap();
            // finding the slope with (y2 - y1) / (x2 - x1)
            // try use proper operators here
            let slope = (y2.checked_sub(y1))
                .unwrap()
                .checked_div(&(x2.checked_sub(x1)).unwrap())
                .unwrap();

            // calculating x3 with s^^2 - x1 - x2
            let x3 = slope.pow(2).checked_sub(x1).unwrap().checked_sub(&x2);
            // calculating y3 = s(x1 - x3) - y1
            let y3 = slope
                .checked_mul(&(x1.checked_sub(&x3.clone().unwrap())).unwrap())
                .unwrap()
                .checked_sub(y1);

            return Point {
                x: x3,
                y: y3,
                a: self.a.clone(),
                b: self.b.clone(),
            };
        }

        // coding point addition for when P1 = P2
        // use Some(value instead of unwraps)
        if self == other {
            // slope = (3x1^^2 + a) / (2y1)
            let x1 = &self.x.clone().unwrap();
            let y1 = &self.y.clone().unwrap();
            let slope = ((3 * x1.pow(2) + self.a.clone()) as BigInt)
                .checked_div(&(2.to_bigint().unwrap() * y1))
                .unwrap();
            // x3 = slope^^2 - 2x1
            let x3 = (slope.pow(2) - (2 * x1)) as BigInt;
            // y3 = slope(x1 - x3) - y1
            let y3 = (slope * (x1 - x3.clone())) - y1;

            return Point {
                x: Some(x3),
                y: Some(y3),
                a: self.a.clone(),
                b: self.b.clone(),
            };
        }

        if self == other && self.y == Some(BigInt::ZERO * self.x.clone().unwrap()) {
            return Point {
                x: None,
                y: None,
                a: self.a.clone(),
                b: self.b.clone(),
            };
        }

        // placeholder to satisfy compiler
        // or panic? seeing as there are no other options
        // use unreachable in future
        self.clone()
        // Point {

        // }
    }
}
