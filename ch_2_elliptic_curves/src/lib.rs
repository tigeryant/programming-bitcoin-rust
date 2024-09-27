use std::ops::Add;

#[derive(Debug)]
pub struct Point {
    a: i128,
    b: i128,
    x: Option<i128>,
    y: Option<i128>
}

/*
impl Point {
    pub fn new (x: Option<i128>, y: Option<i128>, a: i128, b: i128) -> Self {
        // ignore this if statement if x and y are None, when x and y are None this is the point at infinity
        // if x == None && y == None {
        //     return Point {
        //         a,
        //         b,
        //         x: None,
        //         y: None
        //     };
        
        // TODO re-write this - use a match with a pattern to check if x and y are both None

        if y.pow(2) != x.pow(3) + a * x + b {
            panic!("({}, {}) is not on the curve", x, y)
        }
        Point {
            a,
            b,
            x,
            y
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
        
        Point {

        }
    }
}
 */