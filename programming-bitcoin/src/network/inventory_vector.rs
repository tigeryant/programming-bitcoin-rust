pub struct InventoryVector {
    object_type: u32, // little endian
    hash: [u8; 32]
}

impl InventoryVector {
    pub fn new(object_type: u32, hash: [u8; 32]) -> Self {
        Self {
            object_type,
            hash
        }
    } 
}
