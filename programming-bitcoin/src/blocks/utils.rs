use primitive_types::U256;

pub fn bits_to_target(mut bits: [u8; 4]) -> U256 {
    // Convert from little-endian to big-endian
    bits.reverse();

    // Extract the exponent and coefficient
    let exponent = bits[0] as u32;
    let coefficient = ((bits[1] as u32) << 16) | ((bits[2] as u32) << 8) | (bits[3] as u32);

    // Handle special case where coefficient is zero
    if coefficient == 0 {
        return U256::zero();
    }

    // Calculate the target
    U256::from(coefficient) * U256::from(256).pow(U256::from(exponent - 3))
}
