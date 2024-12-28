use crate::utils::hash256::hash256;

pub fn merkle_parent(hash_0: Vec<u8>, hash_1: Vec<u8>) -> Vec<u8> {
    let mut combined = hash_0;
    combined.extend_from_slice(&hash_1);
    hash256(&combined)
}
