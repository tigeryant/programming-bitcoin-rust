use primitive_types::U256;

#[derive(Debug)]
pub struct Signature {
    r: U256,
    s: U256
}

impl Signature {
    pub fn new(r: U256, s: U256) -> Self {
        Self {
            r,
            s
        }
    }

    pub fn r(&self) -> U256 {
        self.r
    }

    pub fn s(&self) -> U256 {
        self.s
    }

    // Returns the signature in Distinguished Encoding Rules as a byte vector
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
        let mut sbin: Vec<u8> = self.s.to_big_endian().to_vec();
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
    
    /// Returns a Signature from a raw sig encoded as a byte vector (DER)
    pub fn parse(raw_sig: Vec<u8>) -> Self {
        let r_length = raw_sig[3];
        // check the value at index 4
        let mut start_index = 4;
        if raw_sig[start_index] == 00 {
            start_index += 1;
        }
        // let r_end_index = start_index + 31; // should always be 31
        let r_end_index = start_index + (r_length as usize) - 2;
        let r = U256::from_big_endian(&raw_sig[start_index..=r_end_index]); // the final index is one more than the end of the slice
        // can refactor
        let marker_byte_index = r_end_index + 1;
        let s_length_index = marker_byte_index + 1;
        // let s_length = raw_sig[s_length_index];
        let mut s_start_index = s_length_index + 1;
        if raw_sig[s_start_index] == 00 {
            s_start_index += 1;
        }
        let s_end_index = s_start_index + 31; // should always be 32 bytes?
        // let s_end_index = s_start_index + (s_length as usize) - 1;
        let s = U256::from_big_endian(&raw_sig[s_start_index..=s_end_index]);
        Self {
            r,
            s
        }
    }
    
}
