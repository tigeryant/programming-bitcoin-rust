use std::io::{ Cursor, Read, Error };

use crate::varint::read_varint;

pub struct Script {
    commands: Vec<Vec<u8>> // will this contain a byte array?
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

    /// Parses a script from a byte vector
    pub fn parse(reader: &mut Cursor<Vec<u8>>) -> Result<Script, Error> {
        let mut commands = vec![];
        let mut count = 0;
        let length = read_varint(reader)?;
        while count < length {
            let mut current = [0u8; 1];
            reader.read_exact(&mut current)?;
            count += 1;
            let current_byte = current[0];
            if (1..=75).contains(&current_byte) { // the next n bytes are an element
                let n = current_byte;
                let mut cmd = vec![0u8; n as usize];
                reader.read_exact(&mut cmd)?;
                commands.push(cmd);
                count += n as u64;
            } else if current_byte == 76 { // op_pushdata1, so the next byte tells us how many bytes to read
                let data_length = read_varint(reader)?;
                let mut cmd = vec![0u8; data_length as usize];
                reader.read_exact(&mut cmd)?;
                commands.push(cmd);
                count += data_length + 1;
            } else if current_byte == 77 { // op_pushdata2, so the next two bytes tells us how many bytes to read
                let data_length = read_varint(reader)?;
                let mut cmd = vec![0u8; data_length as usize];
                reader.read_exact(&mut cmd)?;
                commands.push(cmd);
                count += data_length + 2;
            } else { // it is an op_code we add to the stack
                let op_code = current_byte;
                commands.push(vec![op_code]);
            }
        }
        if count != length {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "parsing script failed",
            ));
        }
        Ok(Self { commands })
    }
}