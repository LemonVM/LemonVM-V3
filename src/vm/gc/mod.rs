use super::{VMClosure, value::Value};
use std::{ptr::NonNull, collections::BTreeMap};

// a good gc that targetting a big project
pub mod lemondger;
// a dum gc for a very small script no need gc pause
pub mod onclosegc;

// manage ptr by Box::into_raw
// and clean it by using
// ptr::drop_in_place(p);
// dealloc(p as *mut u8, Layout::new::<String>());
// !!! ATTENSION PARTIALEQ IS COMPARING POINTER !!!
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GCValue{
    Closure(NonNull<dyn GCBlock>),
    String(NonNull<dyn GCBlock>),
    Opaque(NonNull<dyn GCBlock>),
    Vector(NonNull<dyn GCBlock>),
    Map(NonNull<dyn GCBlock>),
    // TODO: FINISH
    // BigInt()
}
impl GCValue{
    // should only used in gc implementation
    fn get_block(&self) -> NonNull<dyn GCBlock>{
        match self{
            GCValue::Closure(x) => *x,
            GCValue::String(x) => *x,
            GCValue::Opaque(x) => *x,
            GCValue::Vector(x) => *x,
            GCValue::Map(x) => *x,
            _ => unimplemented!()
        }
    }
    // get size in byte
    fn get_data_size(&self) -> usize{
        match self{
            GCValue::Closure(x) => unsafe{x.as_ref().get_data_size()}
            GCValue::String(x) => unsafe{x.as_ref().get_data_size()},
            GCValue::Opaque(x) => unsafe{x.as_ref().get_data_size()},
            GCValue::Vector(x) => unsafe{x.as_ref().get_data_size()},
            GCValue::Map(x) => unsafe{x.as_ref().get_data_size()},
            _ => unimplemented!()
        }
    }
}

pub enum GCInnerValue{
    Closure(VMClosure),
    String(String),
    Opaque(Vec<u8>),
    Vector(Vec<Value>),
    Map(BTreeMap<String,Value>),
    // BigInt()
}

pub trait GCBlock {
    fn get_data(&self) -> NonNull<GCInnerValue>;
    // get size in byte
    fn get_data_size(&self) -> usize;
    // !! is not recursive
    fn get_references(&self) -> Vec<NonNull<dyn GCBlock>>;

    
    // !! if value is a reference type then it is not directly set
    // it will create a new gcblock and move the pointer to the new gcblock
    fn set_value(&mut self, v:Value) -> Value;
}

pub trait GC {
    // the runtime creates first gc( well only one gc i suppose )
    fn on_create(&mut self);

    fn add_block(&mut self, data:GCInnerValue)->NonNull<dyn GCBlock>;
    fn trigger_on_increse_size(&mut self);
    fn trigger_on_massive_increse_size(&mut self);
    // the runtime destroys it's gc
    // TODO: it hooks drop?
    fn on_destroy(&mut self);
}
