use primitive_types::U256;
use crate::{point::Point, signature::Signature};
use crate::secp256k1_params::S256Params;
use crate::s256point::S256Point;

pub struct PrivateKey {
    secret: U256,
    point: Point
}

impl PrivateKey {
    pub fn new(self, secret: U256) -> PrivateKey {
        let g = S256Params::g();
        Self {
            secret,
            point: S256Point::multiply(&g, secret)
        }
    }

    pub fn sign(self, z: U256) -> Signature {
        // Generate random k between 0 and N
        let k = U256::from_dec_str("45").unwrap(); // TODO: Replace with cryptographic random number generation
        
        // Calculate r = (k*G).x
        let g = S256Params::g();
        let k_times_g = &g * k;
        let r = k_times_g.x().unwrap();
        
        // Calculate k_inv using Fermat's little theorem
        let k_inv = k.pow(S256Params::n() - U256::from(2)) % S256Params::n();
        
        // Calculate s = (z + r*secret) * k_inv % N
        let mut s = ((z + r.num() * self.secret) * k_inv) % S256Params::n();
        
        // If s > N/2, set s = N - s (to ensure low S values)
        if s > S256Params::n() / 2 {
            s = S256Params::n() - s;
        }
        
        Signature::new(r.num(), s)
    }
}
