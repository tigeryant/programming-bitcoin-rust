use std::fmt;
use std::io::{Cursor, Error, Read};

use crate::script::op::{self, create_op_code_names, encode_num, OpFunction};
use crate::utils::varint::{encode_varint, read_varint};

use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct Script {
    pub commands: Vec<Vec<u8>>,
}

impl Script {
    pub fn new(commands: Vec<Vec<u8>>) -> Self {
        Self { commands }
    }

    pub fn new_empty_script() -> Self {
        let commands = vec![];
        Self { commands }
    }

    /// Parses a script from a byte vector
    pub fn parse(reader: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut commands = vec![];
        let mut count = 0;
        let length = read_varint(reader)?;
        while count < length {
            let mut current = [0u8; 1];
            reader.read_exact(&mut current)?;
            count += 1;
            let current_byte = current[0];
            if (1..=75).contains(&current_byte) {
                // the next n bytes are an element
                let n = current_byte;
                let mut cmd = vec![0u8; n as usize];
                reader.read_exact(&mut cmd)?;
                commands.push(cmd);
                count += n as u64;
            } else if current_byte == 76 {
                // op_pushdata1, so the next byte tells us how many bytes to read
                let data_length = read_varint(reader)?;
                let mut cmd = vec![0u8; data_length as usize];
                reader.read_exact(&mut cmd)?;
                commands.push(cmd);
                count += data_length + 1;
            } else if current_byte == 77 {
                // op_pushdata2, so the next two bytes tells us how many bytes to read
                let data_length = read_varint(reader)?;
                let mut cmd = vec![0u8; data_length as usize];
                reader.read_exact(&mut cmd)?;
                commands.push(cmd);
                count += data_length + 2;
            } else {
                // it is an op_code we add to the stack
                let op_code = current_byte;
                commands.push(vec![op_code]);
            }
        }

        if count != length {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Script parsing failed: parsed length does not match expected length",
            ));
        }

        Ok(Self { commands })
    }

    pub fn parse_script_sig(
        reader: &mut Cursor<Vec<u8>>,
    ) -> Result<(Self, Option<u32>), Error> {
        let initial_position = reader.position();
        match Self::parse(reader) {
            Ok(script) => Ok((script, None)),
            // If standard parsing fails, try BIP-34 parsing
            Err(_) => {
                reader.set_position(initial_position);
                let mut commands = vec![];

                let length = read_varint(reader)?;

                // Height (BIP-34 requirement)
                let position_before_varint = reader.position();
                let height_length = read_varint(reader)?;
                let position_after_varint = reader.position();
                let varint_length = position_after_varint - position_before_varint;

                let mut padded_bytes = [0u8; 4];
                let mut height_bytes = vec![0u8; height_length as usize];
                reader.read_exact(&mut height_bytes)?;
                let bytes_clone = height_bytes.clone();
                padded_bytes[..height_length as usize].copy_from_slice(&bytes_clone);
                let height = u32::from_le_bytes(padded_bytes);
                commands.push(height_bytes);

                // Read the remaining arbitrary data
                let remaining_length = length - height_length - varint_length;
                if remaining_length > 0 {
                    let mut arbitrary_data = vec![0u8; remaining_length as usize];
                    reader.read_exact(&mut arbitrary_data)?;
                    commands.push(arbitrary_data);
                }

                let script = Self { commands };
                Ok((script, Some(height)))
            }
        }
    }

    fn raw_serialize(&self) -> Vec<u8> {
        let mut result = vec![];
        for cmd in &self.commands {
            if cmd.len() == 1 {
                // must be an opcode
                let op_code = cmd[0];
                result.push(op_code);
            } else {
                let length = cmd.len();
                if length < 76 {
                    // encode length as a single byte
                    result.push(length as u8);
                } else if length <= 0xff {
                    // op_pushdata1, then encode length as a byte
                    result.push(76);
                    result.push(length as u8);
                } else if length <= 520 {
                    // op_pushdata2, then encode length as two bytes
                    result.push(77);
                    result.extend_from_slice(&length.to_le_bytes()[..2]);
                } else {
                    // if it's longer than 520 bytes it's invalid - error
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

    // Serializes a BIP-34 compliant script into a byte vector
    pub fn serialize_bip_34(&self) -> Vec<u8> {
        let mut result = self.raw_serialize_bip_34();
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

    // Serialize a BIP-34 script by appending the arbitrary data without a length varint
    fn raw_serialize_bip_34(&self) -> Vec<u8> {
        let mut result = vec![];
        let height_cmd = &self.commands[0];
        let length = height_cmd.len();
        if length < 76 {
            // encode length as a single byte
            result.push(length as u8);
        } else if length <= 0xff {
            // op_pushdata1, then encode length as a byte
            result.push(76);
            result.push(length as u8);
        } else if length <= 520 {
            // op_pushdata2, then encode length as two bytes
            result.push(77);
            result.extend_from_slice(&length.to_le_bytes()[..2]);
        } else {
            // if it's longer than 520 bytes it's invalid - error
            panic!("too long a cmd");
        }
        result.extend_from_slice(height_cmd);

        let arb_data = &self.commands[1];
        result.extend_from_slice(arb_data);

        result
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
                commands[2][0] == 0x87
                {
                    // OP_EQUAL
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

                // Could change these conditions to use the specific methods we're using below
                // Check for native segwit (P2WPKH)
                if stack.len() == 2 && stack[0] == vec![] && stack[1].len() == 20 {
                    let h160 = stack.pop().unwrap();
                    stack.pop();
                    commands.extend(witness.clone().unwrap());
                    commands.extend(Script::p2pkh_script(h160).get_commands());
                }

                // Check for P2WSH
                if stack.len() == 2 && stack[0] == vec![] && stack[1].len() == 32 {
                    let s256 = stack.pop().unwrap();
                    stack.pop();
                    let witness = witness.clone().unwrap();
                    commands.extend_from_slice(&witness[..witness.len() - 1]);
                    let witness_script = &witness[witness.len() - 1];
                    let witness_hash = Sha256::digest(witness_script).to_vec();
                    if s256 != witness_hash {
                        println!(
                            "Bad SHA256.\n{}\n{}",
                            hex::encode(s256),
                            hex::encode(witness_hash)
                        );
                        return false;
                    }
                    let mut stream = encode_varint(witness_script.len() as u64);
                    stream.extend_from_slice(witness_script);
                    let mut stream = Cursor::new(stream);
                    let witness_script_commands = Script::parse(&mut stream).unwrap().commands;
                    commands.extend_from_slice(&witness_script_commands);
                }
            }
        }
        if stack.is_empty() {
            return false;
        }
        if stack.pop() == Some(encode_num(0)) {
            return false;
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

    /// Takes a hash256 and returns the p2wsh script_pubkey
    pub fn p2wsh_script(h256: Vec<u8>) -> Self {
        Self::new(vec![vec![0x00], h256])
    }

    pub fn get_commands(self) -> Vec<Vec<u8>> {
        self.commands
    }

    pub fn get_redeem_script(&self) -> Self {
        let mut redeem_script = vec![];
        redeem_script.extend_from_slice(&encode_varint(self.commands[3].len() as u64));
        redeem_script.extend_from_slice(&self.commands[3]);
        let mut stream = Cursor::new(redeem_script);
        Self::parse(&mut stream).unwrap()
    }

    pub fn is_p2pk_script_pubkey(&self) -> bool {
        // pubkey followed by OP_CHECKSIG
        let first_element_data = self.commands[0].len() > 1;
        let second_element_checksig = self.commands[1] == vec![0xac];

        first_element_data && second_element_checksig
    }

    pub fn is_p2wpkh_script_pubkey(&self) -> bool {
        // OP_0 and a 20 byte hash
        let length_2 = self.commands.len() == 2;
        let first_byte_zero = self.commands[0][0] == 0x00;
        let second_element_data = self.commands[1].len() > 1;
        let data_20_long = self.commands[1].len() == 20;

        length_2 && first_byte_zero && second_element_data && data_20_long
    }

    pub fn is_p2wsh_script_pubkey(&self) -> bool {
        // OP_0 and a 32 byte (SHA256) hash
        let length_2 = self.commands.len() == 2;
        let first_byte_zero = self.commands[0][0] == 0x00;
        let second_element_data = self.commands[1].len() > 1;
        let data_32_long = self.commands[1].len() == 32;

        length_2 && first_byte_zero && second_element_data && data_32_long
    }

    pub fn is_p2sh_script_pubkey(&self) -> bool {
        // OP_HASH160, 20 byte hash, OP_EQUAL
        self.commands[0][0] == 0xa9 && // OP_HASH160
        self.commands[1].len() > 1 && // hash is a data element
        self.commands[1].len() == 20 && // hash is 20 bytes long
        self.commands[2][0] == 0x87 // OP_EQUAL
    }

    pub fn is_p2sh_script_sig(&self) -> bool {
        // what do we know about the lengths of these elements?
        // OP_0, signature, pubkey, redeem script
        self.commands[0][0] == 0x00 && // OP_0
        self.commands[1].len() > 1 && // signature script is a data element
        self.commands[2].len() > 1 && // pubkey is a data element
        self.commands[3].len() > 1 // redeem script is a data element
    }

    pub fn is_p2tr_script_pubkey(&self) -> bool {
        // OP_1, data
        self.commands[0][0] == 0x51 && // OP_1
        self.commands[1].len() > 1 // data element
    }

    pub fn is_p2pkh_script_pubkey(&self) -> bool {
        // OP_DUP, OP_HASH160, data, OP_EQUALVERIFY, OP_CHECKSIG
        // let commands: Vec<Vec<u8>> = vec![vec![0x76], vec![0xa9], raw_hash, vec![0x88], vec![0xac]];
        self.commands[0][0] == 0x76 && // OP_DUP
        self.commands[1][0] == 0xa9 && // OP_HASH160
        self.commands[2].len() > 1 && // data
        self.commands[3][0] == 0x88 && // OP_EQUALVERIFY
        self.commands[4][0] == 0xac // OP_CHECKSIG
    }

    pub fn is_redeem_script(&self) -> bool {
        // Possibly a redeem script
        self.commands.len() == 1 && // contains one element
        self.commands[0].len() > 1 // element is a data element
    }

    pub fn script_type(&self) -> String {
       match self {
            _script_type if self.commands.is_empty() => String::from("script commands empty"),
            _script_type if self.is_redeem_script() => String::from("1 data element - (possibly redeem script)"),
            _script_type if self.is_p2sh_script_pubkey() => String::from("P2SH"),
            _script_type if self.is_p2wpkh_script_pubkey() => String::from("P2WPKH"),
            _script_type if self.is_p2tr_script_pubkey() => String::from("P2TR"),
            _script_type if self.is_p2pk_script_pubkey() => String::from("P2PK"),
            _script_type if self.is_p2pkh_script_pubkey() => String::from("P2PKH"),
            _script_type if self.is_p2wsh_script_pubkey() => String::from("P2WSH"),
            _ => String::from("unknown")
        } 
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_code_names = create_op_code_names();

        writeln!(f, "  Script type: {}", self.script_type())?;
        writeln!(f, "  Length: {} byte(s)", self.raw_serialize().len())?;
        writeln!(f, "  Data:")?;
        self.commands.iter().try_fold((), |_, cmd| {
            if cmd.len() == 1 {
                let op_name = op_code_names
                    .get(&cmd[0])
                    .map_or(format!("NO OP CODE FOUND ({})", cmd[0]), |name| {
                        name.to_string()
                    });
                writeln!(f, "\t     {} ", op_name)
            } else {
                let mut hex_string = String::with_capacity(cmd.len() * 2);
                let mut ascii_string = String::with_capacity(cmd.len());
                
                cmd.iter().for_each(|byte| {
                    hex_string.push_str(&format!("{:02x}", byte));
                    // Convert byte to ASCII char if printable, otherwise use a dot
                    ascii_string.push(if byte.is_ascii_graphic() {
                        *byte as char
                    } else {
                        '.'
                    });
                });
                
                writeln!(f, "\t     {} (ASCII: {})", hex_string, ascii_string)
            }

        })
    }
}

impl fmt::Debug for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_code_names = create_op_code_names();

        writeln!(f, "Script {{")?;
        writeln!(f, "  Script type: {}", self.script_type())?;
        writeln!(f, "  Raw bytes: {}", hex::encode(self.serialize()))?;
        writeln!(f, "    Length: {} byte(s)", self.raw_serialize().len())?;
        writeln!(f, "    Data:")?;
        
        self.commands.iter().try_fold((), |_, cmd| {
            if cmd.len() == 1 {
                let op_name = op_code_names
                    .get(&cmd[0])
                    .map_or(format!("NO OP CODE FOUND ({})", cmd[0]), |name| {
                        name.to_string()
                    });
                writeln!(f, "        {} ", op_name)
            } else {
                let mut hex_string = String::with_capacity(cmd.len() * 2);
                let mut ascii_string = String::with_capacity(cmd.len());
                
                cmd.iter().for_each(|byte| {
                    hex_string.push_str(&format!("{:02x}", byte));
                    ascii_string.push(if byte.is_ascii_graphic() {
                        *byte as char
                    } else {
                        '.'
                    });
                });
                
                writeln!(f, "        {} (ASCII: {})", hex_string, ascii_string)
            }
        })?;
        write!(f, "}}")
    }
}
