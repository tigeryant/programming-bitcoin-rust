use sha2::{Sha256, Digest};

pub fn hash256(bytes: &[u8]) -> Vec<u8> {
    let first_hash = Sha256::digest(bytes);
    let second_hash = Sha256::digest(first_hash);
    second_hash.to_vec()
}
