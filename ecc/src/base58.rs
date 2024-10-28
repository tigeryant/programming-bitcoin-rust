// put this in a utils directory
use primitive_types::U256;

use crate::hash256::hash256;

const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode_base58(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut leading_zeros = 0;
    // The purpose of this loop is to determine how many of the bytes at the
    // front are 0 bytes. We want to add them back at the end.
    for byte in bytes {
        if *byte == 0 {
            leading_zeros += 1;
        } else {
            break;
        }
    }
    let mut num = U256::from_big_endian(bytes);
    // This is the loop that figures out what Base58 digit to use.
    while num > U256::zero() {
        let div_rem = num.div_mod(U256::from(58u32));
        num = div_rem.0;  // quotient
        let remainder = div_rem.1;  // remainder
        result.push(BASE58_ALPHABET.chars().nth(remainder.as_u32() as usize).unwrap());
    }
    
    for _ in 0..leading_zeros {
        result.push(BASE58_ALPHABET.chars().next().unwrap());
    }
    result.chars().rev().collect()
}

// Encode bytes with a checksum
pub fn encode_base58_checksum(bytes: &[u8]) -> String {
    let mut result = bytes.to_vec();
    let hash = hash256(bytes);
    result.extend_from_slice(&hash[0..4]);
    encode_base58(&result)
}
