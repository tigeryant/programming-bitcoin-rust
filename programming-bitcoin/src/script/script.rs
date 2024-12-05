use std::fmt;
use std::io::{ Cursor, Read, Error };

use crate::utils::varint::read_varint;
use crate::script::op::{self, create_op_code_names, encode_num, OpFunction};

#[derive(Clone, Debug)]
pub struct Script {
    commands: Vec<Vec<u8>>
}

impl Script {
    pub fn new(commands: Vec<Vec<u8>>) -> Self {
        Self {
            commands
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
                "Parsing script failed",
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

    pub fn concat(self, other: Script) -> Self {
        let mut commands = self.commands;
        commands.extend(other.commands);
        Self { commands }
    }

    pub fn evaluate(self, z: Vec<u8>) -> bool { // z should be a U256
        let mut commands = self.commands.clone();
        dbg!(&commands);
        let mut stack = vec![];
        // let altstack = vec![];
        while !commands.is_empty() {
            let cmd = commands.remove(0);
            let is_op_code = cmd.len() == 1; // if the command is of length 1, evaluate it as an op_code
            if is_op_code {
                let op_code = cmd[0];
                dbg!(op_code);
                let names = op::create_op_code_names();
                let op_name = *names.get(&op_code).unwrap();
                let operations = op::create_op_code_functions();
                let op_function = operations.get(&op_code).unwrap().clone(); 
                let operation_result: bool = match op_function {
                    OpFunction::StackOp(func) => func(&mut stack),
                    OpFunction::StackSigOp(func) => func(&mut stack, z.clone()),
                };

                if !operation_result {
                    dbg!(format!("bad op: {op_name}"));
                    return false;
                }
            } else {
                // Handle data element by pushing to stack
                stack.push(cmd.clone());
            }
        }
        if stack.is_empty() {
            return false
        }
        if stack.pop() == Some(encode_num(0)) {
            return false
        }
        true
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_code_names = create_op_code_names();
        
        self.commands.iter().try_fold((), |_, cmd| {
            if cmd.len() == 1 {
                let op_name = op_code_names.get(&cmd[0])
                    .map_or(format!("NO OP CODE FOUND ({})", cmd[0]), |name| name.to_string());
                writeln!(f, "\t     {} ", op_name)
            } else {
                let mut hex_string = String::with_capacity(cmd.len() * 2);
                cmd.iter()
                    .for_each(|byte| hex_string.push_str(&format!("{:02x}", byte)));
                writeln!(f, "\t     {} ", hex_string)
            }
        })
    }
}
