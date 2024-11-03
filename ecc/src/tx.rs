use crate::hash256::hash256;

pub struct Tx {
    version: u32,
    tx_ins: Vec<u32>,
    tx_outs: Vec<u32>,
    locktime: u32,
    testnet: bool
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<u32>, tx_outs: Vec<u32>, locktime: u32, testnet: bool) -> Self {
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet
        }
    }

    pub fn id(&self) -> String {
        // Convert transaction hash to hex string
        hex::encode(self.hash())
    }
    
    fn hash(&self) -> Vec<u8> {
        // Get hash256 of serialized tx
        let hash = hash256(&self.serialize());
        // Reverse to get little endian
        hash.into_iter().rev().collect()
    }

    // Edit this later
    // And a serialize method:
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Serialize version (4 bytes, little endian)
        result.extend_from_slice(&self.version.to_le_bytes());
        
        // Serialize tx_ins
        result.extend(self.tx_ins.iter().flat_map(|tx_in| tx_in.to_le_bytes()));
        
        // Serialize tx_outs
        result.extend(self.tx_outs.iter().flat_map(|tx_out| tx_out.to_le_bytes()));
        
        // Serialize locktime (4 bytes, little endian)
        result.extend_from_slice(&self.locktime.to_le_bytes());
        
        result
    }

    pub fn parse(serialization: Vec<u8>) -> u32 {
        u32::from_le_bytes([
            serialization[0],
            serialization[1], 
            serialization[2],
            serialization[3]
        ])
    }

}