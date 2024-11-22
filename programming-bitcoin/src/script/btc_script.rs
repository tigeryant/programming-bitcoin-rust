use std::io::{ Cursor, Read, Error };

use crate::utils::varint::read_varint;
use crate::script::op::{self, OpFunction};

#[derive(Clone, Debug)] // Write a custom Debug implementation
pub struct Script {
    commands: Vec<Vec<u8>> // will this contain a byte array?
}

impl Script {
    pub fn new(commands: Option<Vec<Vec<u8>>>) -> Self { // consider removing Option here - is there a good reason to accept None? commands is private so we can't add to it later anyway
        match commands {
            Some(cmds) => Self { 
                commands: cmds 
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

    pub fn concat(self, other: Script) -> Self {
        let mut commands = self.commands;
        commands.extend(other.commands);
        Self { commands }
    }

    pub fn evaluate(self, z: Vec<u8>) -> bool { // should z be a stream?
        let mut commands = self.commands.clone();
        let mut stack = vec![];
        // let altstack = vec![];
        while !commands.is_empty() {
            let cmd = commands.remove(0); // remember commands is of type Vec<Vec<u8>> -> cmd will either be an element or an opcode
            // if the command is of length, evaluate it as an op_code
            let is_op_code = cmd.len() == 1;
            if is_op_code {
                let op_code = cmd[0];
                // retrieve the op_code_names hashmap
                let names = op::create_op_code_names();
                let operations = op::create_op_code_functions();
                // retrieve the specific operation from the operations HashMap
                let op_function = operations.get(&op_code).unwrap().clone(); 
                let operation = match op_function {
                    OpFunction::StackOp(func) => func(&mut stack),
                    _ => false
                    // TODO come back to these later (OP_IF and OP_NOTIF)
                    // OpFunction::StackItemsOp(func) => func(&mut stack, &mut commands), // expects a Vec<u8> - should we just pass one element of commands here?
                    // OpFunction::StackAltStackOp(func) => func(&mut stack, &mut alt_stack),
                    // OpFunction::StackHashOp(func) => func(&mut stack),
                    // OpFunction::StackLocktimeSequenceOp(func) => func(&mut stack, locktime, sequence),
                    // OpFunction::StackSigOp(func) => func(&mut stack, z),
                };
                
                let name = *names.get(&op_code).unwrap();
                if vec![99, 100].contains(&op_code) { // OP_IF and OP_NOTIF
                    // execute the command on the stack and commands
                    // examine how OP_IF and OP_NOTIF operate on their commands stack
                    // clearly define what the stack, commands and altstack are here
                    // if !operation(stack, commands) {
                    //     dbg!("bad op: {:?}", name);
                    //     return false;
                    // }
                    dbg!(format!("bad op: {name}"));
                    return false;
                } else if vec![107, 108].contains(&op_code) {
                    dbg!(format!("bad op: {name}"));
                    return false;
                } else if vec![172, 173, 174, 175].contains(&op_code) {
                    dbg!(format!("bad op: {name}"));
                    return false;
                } else {
                    dbg!(format!("bad op: {name}"));
                    return false;
                }
            } else {
                // Handle data element by pushing to stack
                stack.push(cmd.clone());
            }
            dbg!(cmd);
        }
        if stack.is_empty() {
            return false
        }
        if stack.pop() == Some(vec![0]) { // if the last element on the stack is a 0, fail the script by returning false
            return false
        }
        true
    }
}
