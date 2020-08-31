use liblemonvm::{binary::{constant::Constant, function::{Function, FunctionType}}, vm::*};
use std::{ptr::NonNull, collections::BTreeMap};
use gc::{onclosegc::OnCloseGC, GC};
use liblemonvm::binary::opcode::OpCode::*;
macro_rules! INS {
    ($code:ident,$e1:literal,$e2:literal,$e3:literal) => {
        {
            let mut i = $code as u64;
            i |= ($e1 as u64) << 16;
            i |= ($e2 as u64) << 32;
            i |= ($e3 as u64) << 48;
            i
        }
    };
}

fn main() {
    let bytecode = Function{
        is_multi_return_function: false,
        function_type: FunctionType::Function,
        args_count: 0,
        max_registers: 255,
        code: vec![
            INS!(LOADK,0x0000u16,0x0000u16,0xFFFFu16),
            INS!(LOADK,0x0001u16,0x0001u16,0xFFFFu16),
            INS!(ADD8 ,0x0000u16,0x0001u16,0x0002u16),
            INS!(ERROR,0x0000u16,0x0000u16,0x0000u16),
            INS!(RET  ,0x0002u16,0x0000u16,0x0000u16),
        ],
    
        exception_table: None,
        debug_info: None,
    };
    let closure = VMClosure{
        function_bytecode: bytecode,
        args: vec![],
        registers: vec![],
        pc: 0,
        status: VMClosureStatus::None,
        constant_pool_ptr: unsafe{NonNull::new_unchecked(std::ptr::null_mut())},
        stack_values: vec![],
    };
    let gc: Box<dyn GC> = Box::new(OnCloseGC{ blocks: vec![], pool: vec![]});
    let mut state = VMState{
        function_call_chain_states: vec![],
        current_function_call_state: closure,
        exception_stack: vec![],
        args: vec![],
        nargs: BTreeMap::new(),
        return_value: None,
        return_values: vec![],
        constant_pools: vec![],
        debug_mode: true,
        profile_mode: true,
        break_points: vec![],
        gc,  
    };
    let mut CP = BTreeMap::new();
    CP.insert(0x0000, Constant::U8(1));
    CP.insert(0x0001, Constant::U8(1));
    state.constant_pools.push(CP);
    state.current_function_call_state.constant_pool_ptr = NonNull::new(state.constant_pools.last_mut().unwrap()).unwrap();
    interpreter::interpreter(&mut state);
    println!("{:?}",state.return_value);
}
