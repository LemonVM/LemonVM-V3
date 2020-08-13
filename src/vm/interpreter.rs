use super::register;
use crate::config::*;

use crate::{binary::{opcode::OpCode, function::Function}, config::*};
use super::value::Value;

pub enum VMFunctionCallStatus{
    Error,
    Yield,
}


pub struct VMFunctionCallState{
    pub function_bytecode: Function,
    pub registers: Vec<Value>,
    pub pc: u16,
    pub status:VMFunctionCallStatus,
}
// multiple instance sharing a same VMState? go on, tell me the bugs
pub struct VMState{
    // register protection mode: once calling new function, 
    // all old registers is saved in heap
    
    // when calling a function all the register is saved in heap
    // and the last one is last function
    // the first one should always be the main function
    // when current function returns, 
    //     pops the last element(registers) and copy them into the RegisterPool(normally on stack)
    //     the return value of the function also(
    pub function_call_chain_states:Vec<VMFunctionCallState>,
    // when resume a function the heap registers will be copied into stack(depends on size)
    pub current_function_call_state: VMFunctionCallState,
    
    // enable debug mode will able to see the bytecode executed
    pub debug_mode: bool,
    // enable profile mode will display the memory usage and gc status
    pub profile_mode: bool,

    // in runtime add break point(pc) or removing a break point in current function
    pub break_points: Vec<u16>
}

fn interpreter(state:VMState){
    // load registers
    let mut use_regs_stack = true;
    let mut regs_stack:[Value;MAX_REGISTER_ON_STACK] = [Value::Undef;64];
    if state.current_function_call_state.registers.len() > MAX_REGISTER_ON_STACK{
        use_regs_stack = false;
    }
    if use_regs_stack{
        for r in 0..state.current_function_call_state.registers.len(){
            regs_stack[r] = state.current_function_call_state.registers[r];
        }
    }
    let mut pc = state.current_function_call_state.pc;
    let codes = state.current_function_call_state.function_bytecode.code;
    loop{
        // finish
        if pc as usize == codes.len(){
            break;
        }
        // exception
        // break point


        let code = codes[pc as usize];
        let ins = code as u16;
        match ins{
            _ if ins == OpCode::NOP as u16 => {}
            _ if ins == OpCode::IMMBOOL as u16 => {
                let r = (code << 16) as u16;
                let b = (code << 32) as u8;
                // TODO: heap stack
                regs_stack[r as usize] = Value::Boolean(!(b == 0x00));
            }
            _ if ins == OpCode::IMMU8 as u16 => {
                let r = (code << 16) as u16;
                let b = (code << 32) as u8;
                // TODO: heap stack
                regs_stack[r as usize] = Value::U8(b);
            }
            _ if ins == OpCode::IMMU16 as u16 => {
                let r = (code << 16) as u16;
                let b = (code << 32) as u16;
                // TODO: heap stack
                regs_stack[r as usize] = Value::U16(b);
            }
            _ if ins == OpCode::IMMU32 as u16 => {
                let r = (code << 16) as u16;
                let b = (code << 32) as u32;
                // TODO: heap stack
                regs_stack[r as usize] = Value::U32(b);
            }
            // TODO: make sure that is working
            _ if ins == OpCode::IMMU64 as u16 => {
                let r = (code << 16) as u16;
                let b = (code << 32) as u64;
                let b = b << 32;
                pc += 1;
                let next_code = codes[pc as usize];
                let next_ins = code as u16;
                if next_ins != OpCode::EXTRA as u16{
                    panic!("EXCEPT INSTRUCTION EXTRA FOUND {:02X}",next_ins)
                }
                let b2 = (next_code << 16) as u64;
                let b = b + b2;
                // TODO: heap stack
                regs_stack[r as usize] = Value::U64(b);
            }
            
            _ => unimplemented!()
        }
        pc += 1;
    }

}