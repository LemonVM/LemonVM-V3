use crate::binary::{constant::Constant, function::Function};
use gc::GC;
use std::{collections::BTreeMap, ptr::NonNull};
use value::{NSValue, Value};

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
    pub args: Vec<Value>,
    // pub rets: Vec<Value>,
    pub registers: Vec<Value>,
    pub pc: u16,
    pub status: VMClosureStatus,
    pub constant_pool_ptr: NonNull<BTreeMap<u16, Constant>>,
    pub stack_values: Vec<NSValue>,
}

macro_rules! new_stack_value_method_impl {
    ($name:ident,$data_type:ty,$alias_type:ident) => {
        pub fn $name(&mut self, data:$data_type) -> NSValue{
            let b = Box::new(data);
            let ptr = Box::leak(b).into();
            let v = NSValue::$alias_type(ptr);
            self.stack_values.push(v);
            v
        }
    };
}

impl VMClosure {
    new_stack_value_method_impl!(new_closure,VMClosure,Closure);
    new_stack_value_method_impl!(new_string,String,String);
    new_stack_value_method_impl!(new_opaque,Vec<u8>,Opaque);
    new_stack_value_method_impl!(new_vec,Vec<Value>,Vector);
    new_stack_value_method_impl!(new_map,BTreeMap<String,Value>,Map);
    pub fn clean_stack_values(&mut self){
        use std::alloc::{dealloc, Layout};
        use std::ptr;
        for v in &mut self.stack_values{
            unsafe{
                match v {
                    NSValue::Closure(c) => {
                        ptr::drop_in_place(c.as_ptr());
                        dealloc(c.as_ptr() as *mut u8,Layout::new::<VMClosure>());
                    }
                    NSValue::String(s) => {
                        ptr::drop_in_place(s.as_ptr());
                        dealloc(s.as_ptr() as *mut u8,Layout::new::<String>());
                    }
                    NSValue::Opaque(o) => {
                        ptr::drop_in_place(o.as_ptr());
                        dealloc(o.as_ptr() as *mut u8,Layout::new::<Vec<u8>>());
                    }
                    NSValue::Vector(v) => {
                        ptr::drop_in_place(v.as_ptr());
                        dealloc(v.as_ptr() as *mut u8,Layout::new::<Vec<Value>>());
                    }
                    NSValue::Map(m)=> {
                        ptr::drop_in_place(m.as_ptr());
                        dealloc(m.as_ptr() as *mut u8,Layout::new::<BTreeMap<String,Value>>());
                    }
                }
            }
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
    
    pub args: Vec<Value>,
    pub nargs: BTreeMap<String,Value>,

    pub return_value: Option<Value>,
    pub return_values: Vec<Value>,

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
