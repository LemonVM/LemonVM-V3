/*
    an instruction is normally 64 bit long
    
    ins          rega      regb      regc
    0x00 0x00    0xAA 0xAA 0xBB 0xBB 0xCC 0xCC

    but abnormally some ins uses more than 48 bit data
    ins          data-start:
    0x00 0x00    0x00 0x00 0x00 0x00 0x00 0x00
    ins EXTRA
    0x00 0xAA    0x00 0x00 0x00 0x00 0x00 0x00

    until the next instruction is not start with 0xAA
    the data is fully loaded
*/

pub struct LemonVMByteCodeHeader{
    signature: [u8;5],
    version: u32,
}

pub struct LabeledBytecode{
    pub label: u16,
    pub label_len:u16,
    pub instructions: Vec<u64>
}

use super::constant::*;
use std::collections::HashMap;
pub struct DebugInfo();
pub enum FunctionType{
    Function,
    Generator,
    // currently disabled
    AsyncFunction,
    // currently disabled
    AsyncGenerator
}
pub struct FunctionBytecode{
    name: Option<String>,
    function_type:FunctionType,
    bytecodes: Vec<LabeledBytecode>,
    constant_pool: HashMap<u16,Constant>,
    // just used to build arguments object
    args_count: u8,
    closure_vars_count:u8,
    debug_info:Option<DebugInfo>
}