use primitive_types::U256;

use crate::utils::hash256::hash256;

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

pub fn decode_base58(address: &str) -> Result<Vec<u8>, String> {
    // Base58 chars
    let base58_chars: Vec<char> = BASE58_ALPHABET.chars().collect();
    
    // Map characters to their index in the Base58 alphabet
    let mut base58_map = std::collections::HashMap::new();
    for (i, &c) in base58_chars.iter().enumerate() {
        base58_map.insert(c, i as u8);
    }

    // Decode the string
    let mut decoded: Vec<u8> = vec![];
    for c in address.chars() {
        let value = base58_map.get(&c).ok_or_else(|| format!("Invalid Base58 character: {}", c))?;
        let mut carry = *value as u32;
        for byte in decoded.iter_mut().rev() {
            carry += (*byte as u32) * 58;
            *byte = (carry % 256) as u8;
            carry /= 256;
        }
        while carry > 0 {
            decoded.insert(0, (carry % 256) as u8);
            carry /= 256;
        }
    }

    // Handle leading zeroes in the Base58 string
    for c in address.chars() {
        if c == '1' {
            decoded.insert(0, 0);
        } else {
            break;
        }
    }

    // Validate checksum
    if decoded.len() < 4 {
        return Err("Invalid address length".to_string());
    }
    let (payload, checksum) = decoded.split_at(decoded.len() - 4);

    // there is already a utility function for this - see hash256
    use sha2::{Digest, Sha256};
    let computed_checksum = Sha256::digest(Sha256::digest(payload));
    if &computed_checksum[..4] != checksum {
        return Err("Checksum verification failed".to_string());
    }

    // Return the hash (payload without the version byte)
    Ok(payload[1..].to_vec())
}
