/*
    Function is just a thing storage
    in constant pool, is not the function
    actually runs in vm, that is called closure
*/

use super::constant::*;
use super::{debug::*};
use std::collections::BTreeMap;

#[repr(u8)]
pub enum FunctionType{
    Function = 0x00,
    Generator,
    // currently disabled
    AsyncFunction,
    // currently disabled
    AsyncGenerator
}

// if pc in range from to pc and state is
// error then use jump to handler pc
struct Exception{
    from_pc: u16,
    end_pc: u16,
    handler_pc: u16,
}

struct ExceptionTable{
    table:Vec<Exception>
}

pub struct Function{
    function_type:FunctionType,
    // indexed by uuid
    // TODO: seperate constant_pool
    constant_pool: BTreeMap<u16,Constant>,
    // just used to build arguments object
    args_count: u8,
    max_registers: u16,
    exception_table:ExceptionTable,
    code: Vec<u64>,

    debug_info:Option<DebugInfo>,
}