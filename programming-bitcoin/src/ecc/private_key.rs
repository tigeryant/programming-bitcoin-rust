use num_bigint::BigUint;
use primitive_types::U256;
use crate::utils::base58::encode_base58_checksum;
use crate::{ecc::point::Point, ecc::signature::Signature};
use crate::ecc::secp256k1_params::S256Params;
use crate::ecc::s256point::S256Point;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct PrivateKey {
    secret: U256,
    point: Point
}

impl PrivateKey {
    pub fn new(secret: U256) -> PrivateKey {
        let g = S256Params::g();
        Self {
            secret,
            point: S256Point::multiply(&g, secret)
        }
    }

    pub fn point(self) -> Point {
        self.point
    }

    pub fn sign(&self, z: Vec<u8>) -> Signature {
        // Generate random k between 0 and N
        // Should be using cryptographic randomness here
        let z = U256::from_big_endian(&z);
        let k = self.deterministic_k(z);
        
        // Calculate r = (k*G).x
        let g = S256Params::g();
        let k_times_g = &g * k;
        let r = k_times_g.x().unwrap();
        
        // Calculate k_inv using Fermat's little theorem
        // let k_inv = k.pow(S256Params::n() - U256::from(2)) % S256Params::n();
        let k_big = BigUint::from_bytes_be(&k.to_big_endian());
        let n_minus_2 = BigUint::from_bytes_be(&(S256Params::n() - U256::from(2)).to_big_endian());
        let n = BigUint::from_bytes_be(&S256Params::n().to_big_endian());
        let k_inv_big = k_big.modpow(&n_minus_2, &n);
        let k_inv = U256::from_big_endian(&k_inv_big.to_bytes_be());
        
        // Calculate s = (z + r*secret) * k_inv % N
        // let mut s = ((z + r.num() * self.secret) * k_inv) % S256Params::n();

        // Convert existing values to BigUint
        let z_big = BigUint::from_bytes_be(&z.to_big_endian());
        let r_big = BigUint::from_bytes_be(&r.num().to_big_endian());
        let secret_big = BigUint::from_bytes_be(&self.secret.to_big_endian());
        let k_inv_big = BigUint::from_bytes_be(&k_inv.to_big_endian());
        let n_big = BigUint::from_bytes_be(&S256Params::n().to_big_endian());

        // Calculate s using BigUint operations
        let s_big = ((&z_big + &r_big * &secret_big) * &k_inv_big) % &n_big;

        // Convert back to U256
        let mut s = U256::from_big_endian(&s_big.to_bytes_be());
        
        // If s > N/2, set s = N - s (to ensure low S values)
        if s > S256Params::n() / 2 {
            s = S256Params::n() - s;
        }
        
        Signature::new(r.num(), s)
    }

    // From RFC 6979
    pub fn deterministic_k(&self, z: U256) -> U256 {
        type HmacSha256 = Hmac<Sha256>;
        
        // Initialize k and v
        let mut k = vec![0u8; 32];
        let mut v = vec![1u8; 32];
        
        // Adjust z if needed
        let mut z = z;
        if z > S256Params::n() {
            z -= S256Params::n();
        }
        
        // Convert values to bytes
        let z_bytes = z.to_big_endian();
        let secret_bytes = self.secret.to_big_endian();
        
        // First round
        let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
        hmac.update(&[&v[..], &[0u8], &secret_bytes[..], &z_bytes[..]].concat());
        k = hmac.finalize().into_bytes().to_vec();
        
        let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
        hmac.update(&v);
        v = hmac.finalize().into_bytes().to_vec();
        
        // Second round
        let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
        hmac.update(&[&v[..], &[1u8], &secret_bytes[..], &z_bytes[..]].concat());
        k = hmac.finalize().into_bytes().to_vec();
        
        let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
        hmac.update(&v);
        v = hmac.finalize().into_bytes().to_vec();
        
        // Generate k
        loop {
            let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes().to_vec();
            
            let candidate = U256::from_big_endian(&v);
            if candidate >= U256::one() && candidate < S256Params::n() {
                return candidate;
            }
            
            let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
            hmac.update(&[&v[..], &[0u8]].concat());
            k = hmac.finalize().into_bytes().to_vec();
            
            let mut hmac = HmacSha256::new_from_slice(&k).unwrap();
            hmac.update(&v);
            v = hmac.finalize().into_bytes().to_vec();
        }
    }

    // Returns private key in Wallet Import Format (WIF)
    pub fn wif(&self, compressed: bool, testnet: bool) -> String {
        // Convert secret to big-endian bytes
        let secret_bytes = self.secret.to_big_endian();
        
        // Set prefix based on network
        let prefix = if testnet { vec![0xef] } else { vec![0x80] };
        
        // Set suffix based on compression
        let suffix = if compressed { vec![0x01] } else { vec![] };
        
        // Combine all parts
        let mut result = prefix;
        result.extend_from_slice(&secret_bytes);
        result.extend_from_slice(&suffix);
        
        encode_base58_checksum(&result)
    }
}
