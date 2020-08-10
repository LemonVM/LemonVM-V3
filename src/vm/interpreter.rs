use super::register;
use crate::config::*;
fn interpreter(state:register::VMState){
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