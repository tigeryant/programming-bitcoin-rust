use rand::rngs::OsRng;
use rand::RngCore;
use primitive_types::U256;

pub fn get_random_u256() -> U256 {
    let mut rng = OsRng;
    let mut bytes = [0u8; 32]; // 32 bytes = 256 bits
    rng.fill_bytes(&mut bytes);

    U256::from_big_endian(&bytes)
}
