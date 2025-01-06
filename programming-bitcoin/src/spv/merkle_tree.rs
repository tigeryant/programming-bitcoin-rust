pub struct MerkleTree {
    pub total_leaves: u32,
    pub max_depth: u32,
    pub nodes: Vec<Vec<Option<Vec<u8>>>>,
    pub current_depth: u32,
    pub current_index: u32
}

impl MerkleTree {
    pub fn new(total_leaves: u32) -> Self {
        let max_depth = (total_leaves as f64).log2().ceil() as u32;
        let nodes = (0..max_depth + 1)
            .map(|depth| {
                // Calculate number of items at this depth
                let num_items = (total_leaves as f64 / 2f64.powi((max_depth - depth) as i32)).ceil() as usize;
                // Create vector of None values with length num_items
                vec![None; num_items]
            })
            .collect();

        let current_depth: u32 = 0;
        let current_index: u32 = 0;

        Self {
            total_leaves,
            max_depth,
            nodes,
            current_depth,
            current_index,
        }

    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new(27)
    }
}

use std::fmt;

impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Vec::new();
        
        for (depth, level) in self.nodes.iter().enumerate() {
            let mut items = Vec::new();
            
            for (index, h) in level.iter().enumerate() {
                let short = match h {
                    None => "None".to_string(),
                    Some(hash) => format!("{}...", hex::encode(&hash[..4]))
                };

                if depth as u32 == self.current_depth && index as u32 == self.current_index {
                    items.push(format!("*{}*", short));
                } else {
                    items.push(short);
                }
            }
            
            result.push(items.join(", "));
        }
        
        write!(f, "{}", result.join("\n"))
    }
}
