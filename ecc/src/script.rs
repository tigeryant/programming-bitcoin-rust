pub struct Script {
    commands: Vec<Vec<u8>>
}

impl Script {
    pub fn new(commands: Option<Vec<u8>>) -> Self {
        match commands {
            Some(cmds) => Self { 
                commands: vec![cmds] 
            },
            None => Self { 
                commands: vec![] 
            }
        }
    }

    
}