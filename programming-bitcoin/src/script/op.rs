use std::collections::HashMap;

use crate::ecc::point::Point;
use crate::ecc::signature::Signature;
use crate::utils::hash256::hash256;
use crate::utils::hash160::hash160;

pub fn encode_num(num: i32) -> Vec<u8> {
    if num == 0 {
        return vec![];
    }
    let abs_num = num.abs();
    let negative = num < 0;
    let mut result = Vec::new();
    let mut remaining = abs_num;
    while remaining > 0 {
        result.push((remaining & 0xff) as u8);
        remaining >>= 8;
    }
    if result.last().unwrap() & 0x80 != 0 {
        if negative {
            result.push(0x80);
        } else {
            result.push(0);
        }
    } else if negative {
        let last = result.last_mut().unwrap();
        *last |= 0x80;
    }
    result
}

pub fn decode_num(element: &[u8]) -> i32 {
    if element.is_empty() {
        return 0;
    }
    let big_endian = element.iter().rev().cloned().collect::<Vec<_>>();
    let negative = big_endian[0] & 0x80 != 0;
    let mut result = if negative {
        (big_endian[0] & 0x7f) as i32
    } else {
        big_endian[0] as i32
    };
    for &c in &big_endian[1..] {
        result <<= 8;
        result += c as i32;
    }
    if negative {
        -result
    } else {
        result
    }
}

// temporarily removing until it's used
// fn op_0(stack: &mut Vec<Vec<u8>>) -> bool {
//     stack.push(encode_num(0));
//     true
// }

fn op_2(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(2));
    true
}

fn op_6(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(6));
    true
}

// 118 - OP_DUP
pub fn op_dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false
    }
    stack.push(stack[stack.len() - 1].clone());
    true
}

// 136 - OP_EQUAL
fn op_equal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let result = if item1 == item2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

// 147 - OP_ADD
fn op_add(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    stack.push(encode_num(num1 + num2));
    true
}

// 149 - OP_MUL - SHOULD BE DISBALED
fn op_mul(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    stack.push(encode_num(num1 * num2));
    true
}

// 169 - OP_HASH160
pub fn op_hash160(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false
    }
    let element = stack.pop().unwrap();
    stack.push(hash160(&element));
    true
}

// 170 - OP_HASH256
pub fn op_hash256(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false
    }
    let element = stack.pop().unwrap();
    stack.push(hash256(&element));
    true
}

// 172 - OP_CHECKSIG
fn op_checksig(stack: &mut Vec<Vec<u8>>, z: Vec<u8>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    
    // Get the public key and signature from stack
    let pub_key = stack.pop().unwrap();
    let signature_bytes = stack.pop().unwrap();
    
    // 1. Convert pub_key bytes to S256Point
    let pubkey_point = Point::parse_to_s256_point(pub_key);
    // 2. Convert signature bytes to Signature
    let signature = Signature::parse(signature_bytes);
    // 3. Verify signature using point.verify(z, signature)
    let result = pubkey_point.verify(z, signature);
    
    // Push result to stack (1 for valid, 0 for invalid)
    if result {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    result
}

// type StackOpFunc = fn(&mut Vec<Vec<u8>>, &mut Vec<Vec<u8>>) -> bool;

#[derive(Clone)]
pub enum OpFunction {
    StackOp(fn(&mut Vec<Vec<u8>>) -> bool),
    // StackItemsOp(fn(&mut Vec<Vec<u8>>, &mut Vec<u8>) -> bool),
    // StackAltStackOp(StackOpFunc),
    // StackHashOp(fn(&mut Vec<Vec<u8>>) -> bool), // has the same signature as StackOp anyway
    // StackLocktimeSequenceOp(fn(&mut Vec<Vec<u8>>, u32, u32) -> bool),
    StackSigOp(fn(&mut Vec<Vec<u8>>, Vec<u8>) -> bool),
}

// keys are in decimal
pub fn create_op_code_functions() -> HashMap<u8, OpFunction> {
    let mut op_code_functions = HashMap::new();
    op_code_functions.insert(82, OpFunction::StackOp(op_2));
    op_code_functions.insert(86, OpFunction::StackOp(op_6));
    op_code_functions.insert(118, OpFunction::StackOp(op_dup));
    op_code_functions.insert(135, OpFunction::StackOp(op_equal)); // same signature as StackHashOp
    op_code_functions.insert(147, OpFunction::StackOp(op_add));
    op_code_functions.insert(149, OpFunction::StackOp(op_mul)); // should be disabled
    op_code_functions.insert(169, OpFunction::StackOp(op_hash160));
    op_code_functions.insert(170, OpFunction::StackOp(op_hash256));
    op_code_functions.insert(172, OpFunction::StackSigOp(op_checksig));
    op_code_functions
}

pub fn create_op_code_names() -> HashMap<u8, &'static str> {
    let mut op_code_names = HashMap::new();
    op_code_names.insert(82, "OP_2");
    op_code_names.insert(86, "OP_6");
    // 99, 100
    op_code_names.insert(99, "OP_IF");
    op_code_names.insert(100, "OP_NOTIF");
    // 107, 108
    op_code_names.insert(107, "OP_TOALTSTACK");
    op_code_names.insert(108, "OP_FROMALTSTACK");
    op_code_names.insert(118, "OP_DUP");
    op_code_names.insert(135, "OP_EQUAL");
    op_code_names.insert(147, "OP_ADD");
    op_code_names.insert(149, "OP_MUL"); // SHOULD BE DISABLED
    // 172, 173, 174, 175
    op_code_names.insert(172, "OP_CHECKSIG");
    op_code_names.insert(173, "OP_CHECKSIGVERIFY");
    op_code_names.insert(174, "OP_CHECKMULTISIG");
    op_code_names.insert(175, "OP_CHECKMULTISIGVERIFY");
    op_code_names
}
