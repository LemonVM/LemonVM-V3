use crate::binary::{constant::Constant, function::Function};
use gc::GC;
use std::{collections::BTreeMap, ptr::NonNull};
use value::Value;

pub mod gc;
pub mod interpreter;
pub mod register;
pub mod value;

#[derive(Debug, Clone, PartialEq)]
pub enum VMClosureStatus {
    None,
    Error,
    Yield,
}

// function call
// first use args to fix all the variable that calling this function
// to an args object
// then using the max call args to make the register mapping
// if arguments is far more over then think a way to... kind of vargs for example make an object
#[derive(Debug, Clone, PartialEq)]
pub struct VMClosure {
    pub function_bytecode: Function,
    pub vargs: Vec<Value>,
    pub rets: Vec<Value>,
    pub registers: Vec<Value>,
    pub pc: u16,
    pub status: VMClosureStatus,
    pub constant_pool_ptr: NonNull<BTreeMap<u16, Constant>>,
}
#[derive(Debug, Clone)]
pub struct VMFunctionCallArgsObject {
    // save value rather than using reference
    args: Vec<Value>,
}

impl VMClosure {
    fn call_with_args_obj(&mut self, args_obj: VMFunctionCallArgsObject) {
        let args = self.function_bytecode.args_count;
        let mut aobj = args_obj.clone();
        for i in 0..args as usize {
            // default value is undef
            self.registers[i] = aobj.args.pop().unwrap_or(Value::Undef);
        }
        if aobj.args.len() > 0 {
            self.vargs.append(&mut aobj.args);
        }
    }
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
    pub function_call_chain_states: Vec<VMClosure>,
    // when resume a function the heap registers will be copied into stack(depends on size)
    pub current_function_call_state: VMClosure,
    // after throwing an exception and in that scope there is no exception handler
    // in that case the exception will throw to super function call
    // for saving the exception status adding the closure into the exception_stack
    // when the exception is finally handled the exception stack is renewed
    pub exception_stack: Vec<VMClosure>,

    pub constant_pools: Vec<BTreeMap<u16, Constant>>,

    // enable debug mode will able to see the bytecode executed
    pub debug_mode: bool,
    // enable profile mode will display the memory usage and gc status
    pub profile_mode: bool,

    // in runtime add break point(pc) or removing a break point in current function
    pub break_points: Vec<u16>,

    pub gc: Box<dyn GC>,
}

impl VMState {
    // if the function loaded the module then return true
    // if the function is just doing some other work just return false
    fn module_load_hook(function: fn() -> bool) -> bool {
        false
    }
    fn load_module() {}
    // normally use in start up
    fn run_module() {}
}
