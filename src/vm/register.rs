use crate::config::*;

pub enum VMFunctionCallStatus{
    Error,
    Yield,
}

pub struct VMFunctionCallState{
    pub uuid: u64,
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
}