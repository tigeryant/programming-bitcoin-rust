use sha2::{Sha256, Digest};
use ripemd::Ripemd160;

// combination of sha256 and ripemd160 hashes
pub fn hash160(input: &[u8]) -> Vec<u8> {
    let sha256_digest = Sha256::digest(input);
    let ripemd160_digest = Ripemd160::digest(sha256_digest);
    ripemd160_digest.to_vec()
}
