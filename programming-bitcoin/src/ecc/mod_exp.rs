use primitive_types::U256;
use num_bigint::BigUint;

pub fn mod_exp(base: U256, exp: U256, modulus: U256) -> U256 {
    // Convert U256 to BigUint
    let base_bytes = U256::to_big_endian(&base);
    // let exp_bytes = exp.to_be_bytes();
    let exp_bytes = U256::to_big_endian(&exp);
    // let modulus_bytes = modulus.to_be_bytes();
    let modulus_bytes = U256::to_big_endian(&modulus);
    
    let base_biguint = BigUint::from_bytes_be(&base_bytes);
    let exp_biguint = BigUint::from_bytes_be(&exp_bytes);
    let modulus_biguint = BigUint::from_bytes_be(&modulus_bytes);
    
    // Perform modular exponentiation with BigUint
    let result_biguint = base_biguint.modpow(&exp_biguint, &modulus_biguint);
    
    // Convert back to U256
    let result_bytes = result_biguint.to_bytes_be();
    U256::from_big_endian(&result_bytes)
}
