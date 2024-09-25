#[derive(Debug)]
pub struct Point {
    a: i128,
    b: i128,
    x: i128,
    y: i128
}

impl Point {
    pub fn new (x: i128, y: i128, a: i128, b: i128) -> Self {
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
