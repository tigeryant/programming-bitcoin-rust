use std::collections::HashMap;
use crate::utils::hash256::hash256;
use crate::utils::hash160::hash160;

// 118 - OP_DUP
pub fn op_dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false
    }
    stack.push(stack[stack.len()].clone());
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
fn op_checksig(_stack: &mut Vec<Vec<u8>>, _z: i64) -> bool {
    // unimplemented!() // placeholder
    true
}

// 172 - OP_CHECKSIG - STARTING POINT
/*
fn op_checksig(stack: &mut Vec<Vec<u8>>, z: i64) -> bool {
    if stack.len() < 2 {
        return false;
    }
    
    // Get the public key and signature from stack
    let pub_key = stack.pop().unwrap();
    let signature = stack.pop().unwrap();
    
    // TODO: Implement actual signature verification using S256Point and Signature
    // 1. Convert pub_key bytes to S256Point
    // 2. Convert signature bytes to Signature
    // 3. Verify signature using point.verify(z, signature)
    
    // Push result to stack (1 for valid, 0 for invalid)
    stack.push(vec![1]);
    true
}
*/

type StackOpFunc = fn(&mut Vec<Vec<u8>>, &mut Vec<Vec<u8>>) -> bool;

#[derive(Clone)]
pub enum OpFunction {
    StackOp(fn(&mut Vec<Vec<u8>>) -> bool),
    StackItemsOp(fn(&mut Vec<Vec<u8>>, &mut Vec<u8>) -> bool),
    StackAltStackOp(StackOpFunc),
    StackHashOp(fn(&mut Vec<Vec<u8>>) -> bool),
    StackLocktimeSequenceOp(fn(&mut Vec<Vec<u8>>, u32, u32) -> bool),
    StackSigOp(fn(&mut Vec<Vec<u8>>, i64) -> bool),
}

pub fn create_op_code_functions() -> HashMap<u8, OpFunction> {
    let mut op_code_functions = HashMap::new();
    op_code_functions.insert(118, OpFunction::StackOp(op_dup));
    op_code_functions.insert(169, OpFunction::StackOp(op_hash160));
    op_code_functions.insert(170, OpFunction::StackOp(op_hash256));
    op_code_functions.insert(172, OpFunction::StackSigOp(op_checksig));
    op_code_functions
}

// create a names stack also?
pub fn create_op_code_names() -> HashMap<u8, &'static str> {
    let mut op_code_names = HashMap::new();
    // 99, 100
    op_code_names.insert(99, "OP_IF");
    op_code_names.insert(100, "OP_NOTIF");
    // 107, 108
    op_code_names.insert(107, "OP_TOALTSTACK");
    op_code_names.insert(108, "OP_FROMALTSTACK");
    // 172, 173, 174, 175
    op_code_names.insert(172, "OP_CHECKSIG");
    op_code_names.insert(173, "OP_CHECKSIGVERIFY");
    op_code_names.insert(174, "OP_CHECKMULTISIG");
    op_code_names.insert(175, "OP_CHECKMULTISIGVERIFY");
    op_code_names
}
