use crate::utils::base58::encode_base58_checksum;

pub fn h160_to_p2sh_address(h160: &str, testnet: bool) -> String {
    let network_prefix: u8 = match testnet {
        true => 0xc4, // testnet prefix for P2SH
        false => 0x05 // mainnet prefix for P2SH
    };
    let mut h160_vec = hex::decode(h160).unwrap();
    h160_vec.insert(0, network_prefix);
    encode_base58_checksum(&h160_vec)
}

pub fn h160_to_p2pkh(h160: &str, testnet: bool) -> String {
    let network_prefix: u8 = match testnet {
        true => 0x6F, // testnet prefix for P2PKH
        false => 0x05 // mainnet prefix for P2PKH
    };
    let mut h160_vec = hex::decode(h160).unwrap();
    h160_vec.insert(0, network_prefix);
    encode_base58_checksum(&h160_vec)
}
