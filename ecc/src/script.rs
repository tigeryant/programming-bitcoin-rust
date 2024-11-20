use std::io::{ Cursor, Read, Error };

use crate::varint::read_varint;

#[derive(Clone, Debug)]
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

    fn raw_serialize(&self) -> Vec<u8> {
        let mut result = vec![];
        for cmd in &self.commands {
            if cmd.len() == 1 { // must be an opcode
                let op_code = cmd[0];
                result.push(op_code);
            } else {
                let length = cmd.len();
                if length < 76 { // encode length as a single byte
                    result.push(length as u8);
                } else if length <= 0xff { // op_pushdata1, then encode length as a byte
                    result.push(76);
                    result.push(length as u8);
                } else if length <= 520 { // op_pushdata2, then encode length as two bytes
                    result.push(77);
                    result.extend_from_slice(&length.to_le_bytes()[..2]);
                } else { // if it's longer than 520 bytes it's invalid - error
                    panic!("too long a cmd");
                }
                result.extend_from_slice(cmd);
            }
        }
        result
    }

    /// Serializes the script into a byte vector
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = self.raw_serialize();
        let total = result.len();
        let mut length_bytes = vec![];
        if total < 0xfd {
            length_bytes.push(total as u8);
        } else if total <= 0xffff {
            length_bytes.push(0xfd);
            length_bytes.extend_from_slice(&(total as u16).to_le_bytes());
        } else {
            length_bytes.push(0xfe);
            length_bytes.extend_from_slice(&(total as u32).to_le_bytes());
        }
        length_bytes.append(&mut result);
        length_bytes
    }
}