pub struct MerkleTree {
    pub total_leaves: u32,
    pub max_depth: usize,
    pub nodes: Vec<Vec<Option<Vec<u8>>>>,
    pub current_depth: usize,
    pub current_index: usize
}

impl MerkleTree {
    pub fn new(total_leaves: u32) -> Self {
        let max_depth = (total_leaves as f64).log2().ceil() as usize;
        let nodes = (0..max_depth + 1)
            .map(|depth| {
                // Calculate number of items at this depth
                let num_items = (total_leaves as f64 / 2f64.powi((max_depth - depth) as i32)).ceil() as usize;
                // Create vector of None values with length num_items
                vec![None; num_items]
            })
            .collect();

        let current_depth: usize = 0;
        let current_index: usize = 0;

        Self {
            total_leaves,
            max_depth,
            nodes,
            current_depth,
            current_index,
        }
    }

    pub fn up(&mut self) {
        self.current_depth -= 1;
        self.current_index /= 2;
    }

    pub fn left(&mut self) {
        self.current_depth += 1;
        self.current_index *= 2;
    }

    pub fn right(&mut self) {
        self.current_depth += 1;
        self.current_index = self.current_index * 2 + 1;
    }

    pub fn root(&self) -> Option<Vec<u8>> {
        self.nodes[0][0].clone()
    }

    pub fn set_current_node(&mut self, value: Option<Vec<u8>>) {
        self.nodes[self.current_depth][self.current_index] = value;
    }

    pub fn get_left_node(&self) -> Option<Vec<u8>> {
        self.nodes[self.current_depth + 1][self.current_index * 2].clone()
    }

    pub fn get_right_node(&self) -> Option<Vec<u8>> {
        self.nodes[self.current_depth + 1][self.current_index * 2 + 1].clone()
    }

    pub fn is_leaf(&self) -> bool {
        self.current_depth == self.max_depth
    }

    pub fn right_exists(&mut self) -> bool {
        self.nodes[self.current_depth + 1].len() > self.current_index * 2 + 1
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

                if depth == self.current_depth && index == self.current_index {
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
