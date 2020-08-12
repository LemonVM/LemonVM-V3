use super::register;
use crate::config::*;

use crate::{binary::function::Function, config::*};

pub enum VMFunctionCallStatus{
    Error,
    Yield,
}


pub struct VMFunctionCallState{
    pub function_bytecode: Function,
    pub registers: Vec<u64>,
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
    let mut regs_stack:[u64;MAX_REGISTER_ON_STACK] = [0;64];
    if state.current_function_call_state.registers.len() > MAX_REGISTER_ON_STACK{
        use_regs_stack = false;
    }
    if use_regs_stack{
        for r in 0..state.current_function_call_state.registers.len(){
            regs_stack[r] = state.current_function_call_state.registers[r];
        }
    }
    let mut pc = state.current_function_call_state.pc;

    


}