use primitive_types::U256;

#[derive(Debug)]
pub struct Signature {
    pub r: U256,
    pub s: U256
}

impl Signature {
    pub fn new(r: U256, s: U256) -> Self {
        Self {
            r,
            s
        }
    }
}