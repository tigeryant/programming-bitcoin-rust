use std::collections::HashMap;
use crate::utils::hash256::hash256;
use crate::utils::hash160::hash160;

pub enum OpFunction {
    StackOp(fn(&mut Vec<Vec<u8>>) -> bool)
}

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

pub fn create_op_code_functions() -> HashMap<u32, OpFunction> {
    let mut op_code_functions = HashMap::new();
    op_code_functions.insert(118, OpFunction::StackOp(op_dup));
    op_code_functions.insert(169, OpFunction::StackOp(op_hash160));
    op_code_functions.insert(170, OpFunction::StackOp(op_hash256));
    op_code_functions
}

// create a names stack also?
