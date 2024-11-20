use primitive_types::U256;

#[derive(Debug)]
pub struct Signature {
    pub r: U256,
    pub s: U256
}

impl Signature {
    pub fn new(r: U256, s: U256) -> Self {
        Self {
            r,
            s
        }
    }

    // Returns the signature in Distinguished Encoding Rules
    pub fn der(&self) -> Vec<u8> {
        // Convert r to big-endian bytes and trim leading zeros
        let mut rbin: Vec<u8> = self.r.to_big_endian().to_vec();
        while rbin.first() == Some(&0) && rbin.len() > 1 {
            rbin.remove(0);
        }
        
        // Add 0x00 byte if high bit is set
        if rbin[0] & 0x80 != 0 {
            rbin.insert(0, 0x00);
        }
        
        // Build r component
        let mut result = vec![0x02, rbin.len() as u8];
        result.extend_from_slice(&rbin);
        
        // Convert s to big-endian bytes and trim leading zeros
        let mut sbin: Vec<u8> = self.r.to_big_endian().to_vec();
        while sbin.first() == Some(&0) && sbin.len() > 1 {
            sbin.remove(0);
        }
        
        // Add 0x00 byte if high bit is set
        if sbin[0] & 0x80 != 0 {
            sbin.insert(0, 0x00);
        }
        
        // Build s component
        result.push(0x02);
        result.push(sbin.len() as u8);
        result.extend_from_slice(&sbin);
        
        // Wrap in sequence
        let mut final_result = vec![0x30, result.len() as u8];
        final_result.extend(result);
        
        final_result
    }
    
}
