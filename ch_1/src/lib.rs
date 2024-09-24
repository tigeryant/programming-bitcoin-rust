#[derive(Debug)]
pub struct FieldElement {
    num: u32,
    prime: u32
}

impl FieldElement {
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
