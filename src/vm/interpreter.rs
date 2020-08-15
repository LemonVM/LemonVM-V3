use super::register;
use crate::config::*;

use super::value::Value;
use crate::{
    binary::{function::Function, opcode::OpCode},
    config::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum VMFunctionCallStatus {
    Error,
    Yield,
}
#[derive(Debug, Clone, PartialEq)]
pub struct VMFunctionCallState {
    pub function_bytecode: Function,
    pub registers: Vec<Value>,
    pub pc: u16,
    pub status: VMFunctionCallStatus,
}
// multiple instance sharing a same VMState? go on, tell me the bugs
pub struct VMState {
    // register protection mode: once calling new function,
    // all old registers is saved in heap

    // when calling a function all the register is saved in heap
    // and the last one is last function
    // the first one should always be the main function
    // when current function returns,
    //     pops the last element(registers) and copy them into the RegisterPool(normally on stack)
    //     the return value of the function also(
    pub function_call_chain_states: Vec<VMFunctionCallState>,
    // when resume a function the heap registers will be copied into stack(depends on size)
    pub current_function_call_state: VMFunctionCallState,

    // enable debug mode will able to see the bytecode executed
    pub debug_mode: bool,
    // enable profile mode will display the memory usage and gc status
    pub profile_mode: bool,

    // in runtime add break point(pc) or removing a break point in current function
    pub break_points: Vec<u16>,
}

//                             better enable in very large function
fn interpreter(state: VMState) {
    // init
    let current_function_state = state.current_function_call_state.clone();
    if current_function_state.registers.len() > MAX_REGISTER_ON_STACK {
        panic!("ERROR! PLATFORM DOES NOT SUPPORT THAT MUCH REGISTER")
    }
    let codes = current_function_state.function_bytecode.code;

    // load registers
    let mut stack_regs: [Value; MAX_REGISTER_ON_STACK] = [Value::Undef; 64];
    //  copy register to stack
    for r in 0..current_function_state.registers.len() {
        stack_regs[r] = current_function_state.registers[r];
    }
    let mut pc = current_function_state.pc;
    loop {
        // on exception
        // on breakpoint
        // on whatever

        // on end function scope
        if pc >= codes.len() as u16 {
            // TODO: I think that is an exception because a normal function at least has one return value or uses ret to return a void
        }
        let ci = codes[pc as usize];

        // register status
        let ins = ci as u16;
        let e1 = (ci << 16) as u16;
        let e1x = (ci << 16) as u32;
        let e2 = (ci << 32) as u16;
        let e2x = (ci << 32) as u32;
        let e3 = (ci << 48) as u16;
        //                          u48
        let ed = (ci << 16) as u64;

        match ins {
            _ if ins == OpCode::IMMU8 as u16 => {
                stack_regs[e1 as usize] = Value::U8(e2 as u8);
            }
            _ if ins == OpCode::IMMU16 as u16 => {
                stack_regs[e1 as usize] = Value::U16(e2);
            }
            _ if ins == OpCode::IMMU32 as u16 => {
                stack_regs[e1 as usize] = Value::U32(e2x);
            }
            _ if ins == OpCode::IMMU64 as u16 => {
                let dp1 = e2x;
                pc += 1;
                let ci = codes[pc as usize];
                let ins = ci as u16;
                //                          u48
                let ed = (ci << 16) as u64;
                if ins != OpCode::EXTRA as u16 {
                    panic!("ERROR! EXCEPT EXTRA FIND {:02X}", ins);
                } else {
                    let v = ed << 16 + dp1;
                    stack_regs[e1 as usize] = Value::U64(v);
                }
            }

            _ => unimplemented!(),
        }

        pc += 1;
    }

    // restore status
    state.current_function_call_state = current_function_state;
}
