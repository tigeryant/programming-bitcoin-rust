use crate::utils::hash256::hash256;

pub fn merkle_parent(hash_0: Vec<u8>, hash_1: Vec<u8>) -> Vec<u8> {
    let mut combined = hash_0;
    combined.extend_from_slice(&hash_1);
    hash256(&combined)
}

pub fn merkle_parent_level(mut hashes: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    if hashes.len() % 2 == 1 {
        hashes.push(hashes[hashes.len() - 1].clone());
    }

    (0..hashes.len())
        .step_by(2)
        .map(|i| merkle_parent(hashes[i].clone(), hashes[i + 1].clone()))
        .collect::<Vec<Vec<u8>>>()
}

pub fn merkle_root(hashes: Vec<&str>) -> Vec<u8> {
    let hashes = hashes
        .into_iter()
        .map(|hash| hex::decode(hash).unwrap())
        .collect::<Vec<Vec<u8>>>();

    let mut current_hashes = hashes;

    while current_hashes.len() > 1 {
        current_hashes = merkle_parent_level(current_hashes);
    }

    current_hashes[0].clone()
}
