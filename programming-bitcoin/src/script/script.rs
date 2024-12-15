use std::fmt;
use std::io::{ Cursor, Read, Error };

use crate::utils::varint::{encode_varint, read_varint};
use crate::script::op::{self, create_op_code_names, encode_num, OpFunction};


#[derive(Clone, Debug)]
pub struct Script {
    pub commands: Vec<Vec<u8>>
}

impl Script {
    pub fn new(commands: Vec<Vec<u8>>) -> Self {
        Self {
            commands
        }
    }

    pub fn new_empty_script() -> Self {
        let commands = vec![];
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

    pub fn evaluate(self, z: Vec<u8>, witness: Option<Vec<Vec<u8>>>) -> bool {
        let mut commands = self.commands.clone();
        let mut stack = vec![];
        // let altstack = vec![];
        while !commands.is_empty() {
            let cmd = commands.remove(0);
            let is_op_code = cmd.len() == 1; // if the command is of length 1, evaluate it as an op_code
            if is_op_code {
                let op_code = cmd[0];
                let names = op::create_op_code_names();
                let op_name = *names.get(&op_code).unwrap();
                dbg!(&op_name);
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

                // Check for P2SH
                if commands.len() == 3 &&
                commands[0][0] == 0xa9 && // OP_HASH160
                commands[1].len() > 1 && // redeem script is a data element
                commands[1].len() == 20 && // redeem script is 20 bytes long
                commands[2][0] == 0x87 { // OP_EQUAL
                    commands.pop();
                    let h160 = commands.pop().unwrap();
                    commands.pop();
                    let op_hash160_result = op::op_hash160(&mut stack);
                    if !op_hash160_result {
                        return false;
                    }
                    stack.push(h160);
                    let op_equal_result = op::op_equal(&mut stack);
                    if !op_equal_result {
                        return false;
                    }
                    let op_verify_result = op::op_verify(&mut stack);
                    if !op_verify_result {
                        dbg!("bad p2sh h160");
                        return false;
                    }
                    let mut redeem_script = encode_varint(cmd.len() as u64);
                    redeem_script.extend(cmd);
                    let mut stream = Cursor::new(redeem_script);
                    commands.extend(Script::parse(&mut stream).unwrap().get_commands());
                }

                // Check for native segwit (P2WPKH)
                if stack.len() == 2 && stack[0] == vec![] && stack[1].len() == 20 {
                    let h160 = stack.pop().unwrap();
                    stack.pop();
                    commands.extend(witness.clone().unwrap());
                    commands.extend(Script::p2pkh_script(h160).get_commands());
                }

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

    /// Takes a hash160 and returns the p2pkh script_pubkey
    pub fn p2pkh_script(h160: Vec<u8>) -> Self {
        let raw_hash = h160;
        // OP_DUP, OP_HASH160, hash160 data element, OP_EQUALVERIFY, OP_CHECKSIG
        let commands: Vec<Vec<u8>> = vec![vec![0x76], vec![0xa9], raw_hash, vec![0x88], vec![0xac]];
        // script_pubkey
        Script::new(commands)
    }

    /// Takes a hash160 and returns the p2wpkh script_pubkey
    pub fn p2wpkh_script(h160: Vec<u8>) -> Self {
        Self::new(vec![vec![0x00], h160])
    }

    pub fn get_commands(self) -> Vec<Vec<u8>> {
        self.commands
    }

    pub fn is_p2wpkh(&self) -> bool { // This is for script_pubkey
        let length_2 = self.commands.len() == 2;
        let first_byte_zero = self.commands[0] == vec![0x00];
        let second_element_data = self.commands[1].len() > 1;
        let data_20_long = self.commands[1].len() == 20;

        length_2 && first_byte_zero && second_element_data && data_20_long
    }

    pub fn is_p2sh(&self) -> bool { // This is for script_pubkey
        self.commands[0][0] == 0xa9 && // OP_HASH160
        self.commands[1].len() > 1 && // redeem script is a data element
        self.commands[1].len() == 20 && // redeem script is 20 bytes long
        self.commands[2][0] == 0x87 // OP_EQUAL 
    }

    pub fn is_p2sh_script_sig(&self) -> bool {
        // what do we know about the lengths of these elements?
        self.commands[0][0] == 0x00 && // OP_0
        self.commands[1].len() > 1 && // signature script is a data element
        self.commands[2].len() > 1 && // pubkey is a data element
        self.commands[3].len() > 1 // redeem script is a data element
    }

    pub fn get_redeem_script(&self) -> Self {
        let mut redeem_script = vec![];
        redeem_script.extend_from_slice(&encode_varint(self.commands[3].len() as u64));
        redeem_script.extend_from_slice(&self.commands[3]);
        let mut stream = Cursor::new(redeem_script);
        Self::parse(&mut stream).unwrap()
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_code_names = create_op_code_names();
        // Add check for script type (P2PKH, P2SH, etc) for more descriptive output

        // First write the script length
        writeln!(f, "Length: {} byte(s)", self.raw_serialize().len())?;
        writeln!(f, "Data:")?;
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
