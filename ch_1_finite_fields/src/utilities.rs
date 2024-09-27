use primitive_types::U256;

/// Calculate modular inverse of `num` mod `prime` using the extended Euclidean algorithm
pub fn mod_inverse(num: U256, prime: U256) -> Option<U256> {
    // Convert U256 to i128 for signed arithmetic in intermediate calculations
    let mut t: i128 = 0;
    let mut new_t: i128 = 1;
    let mut r: i128 = prime.as_u128() as i128;
    let mut new_r: i128 = num.as_u128() as i128;

    // Extended Euclidean Algorithm
    while new_r != 0 {
        let quotient = r / new_r;

        // Update t and r using signed arithmetic
        let temp_t = t;
        t = new_t;
        new_t = temp_t - quotient * new_t;

        let temp_r = r;
        r = new_r;
        new_r = temp_r - quotient * new_r;
    }

    // If r != 1, no modular inverse exists
    if r != 1 {
        return None;
    }

    // Ensure t is positive by converting back to U256 and adjusting if necessary
    if t < 0 {
        t += prime.as_u128() as i128;
    }

    // Return t as U256
    Some(U256::from(t as u128))
}
