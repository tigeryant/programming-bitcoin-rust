// use std::ops::Add;
use num_bigint::BigInt;

#[derive(Debug)]
pub struct Point {
    x: Option<BigInt>,
    y: Option<BigInt>,
    a: BigInt,
    b: BigInt
}

impl Point {
    pub fn new (x: Option<BigInt>, y: Option<BigInt>, a: BigInt, b: BigInt) -> Self {
        match (x, y) {
            (Some(x), Some(y)) => {
                if y.pow(2) != x.pow(3) + &a * &x + &b {
                    panic!("({}, {}) is not on the curve", x, y)
                }
                Self {
                    x: Some(x),
                    y: Some(y),
                    a,
                    b
                }
            },
            (None, None) => {
                Self {
                    x: None,
                    y: None,
                    a,
                    b
                }
            }
            _ => { panic!("Invalid parameters to Point::new()")}
        }

    }
}

impl PartialEq for &Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

// implement point addition (Add)
/*
impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        
        Point {

        }
    }
}
*/
