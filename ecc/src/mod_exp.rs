use primitive_types::U256;

// Modular exponentiation by squaring
pub fn mod_exp(base: U256, exp: U256, modulus: U256) -> U256 {
    let mut result = U256::from(1);
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > U256::zero() {
        if exp % 2 == U256::from(1) {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    result
}
