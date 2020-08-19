use super::value::Value;
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
            GCValue::String(x) => unsafe{x.as_ref().get_data_size()},
            GCValue::Opaque(x) => unsafe{x.as_ref().get_data_size()},
            GCValue::Vector(x) => unsafe{x.as_ref().get_data_size()},
            GCValue::Map(x) => unsafe{x.as_ref().get_data_size()},
            _ => unimplemented!()
        }
    }
}

pub enum GCInnerValue{
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

    // // following methods only being used in built in container structure
    // // value's lifetime follows the data structure
    // fn add_value(&mut self, v:Value);
    // // reference's lifetime follows all it's referee
    // fn add_reference(&mut self, ref_: GCValue);
    // // map
    // fn insert(&mut self, k:String,v:Value);

}

pub trait GC {
    // the runtime creates first gc( well only one gc i suppose )
    fn on_create(&mut self);

    fn add_block(&mut self, data:GCInnerValue);
    // young generation
    fn trigger_on_close_function_call(&mut self);
    // old generation
    fn trigger_on_increse_size(&mut self);
    // imm generation
    fn trigger_on_massive_increse_size(&mut self);
    // the runtime destroys it's gc
    // TODO: it hooks drop?
    fn on_destroy(&mut self);
}
