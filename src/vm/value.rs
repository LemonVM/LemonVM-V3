use super::{VMClosure, gc::*, VMState, VMClosureStatus};
use std::{ptr::NonNull, collections::BTreeMap};
use crate::binary::constant::Constant;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    // on stack
    Null,
    Undef,
    Boolean(bool),

    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    // on gc heap
    GCValue(GCValue),
}
impl Value{
    pub fn from_constant(c: Constant, constant_pool_ptr:NonNull<BTreeMap<u16,Constant>>, state:&mut VMState) -> Self {
        match c {
            Constant::U8(v) => {Value::U8(v)}
            Constant::I8(v) => {Value::I8(v)}
            Constant::U16(v) => {Value::U16(v)}
            Constant::I16(v) => {Value::I16(v)}
            Constant::U32(v) => {Value::U32(v)}
            Constant::I32(v) => {Value::I32(v)}
            Constant::U64(v) => {Value::U64(v)}
            Constant::I64(v) => {Value::I64(v)}
            Constant::F32(v) => {Value::F32(v)}
            Constant::F64(v) => {Value::F64(v)}

            Constant::Function(v) => {
                let closure = VMClosure{
                    function_bytecode: v.clone(),
                    vargs: vec![],
                    rets: vec![],
                    registers: vec![Value::Undef;v.args_count as usize],
                    pc: 0,
                    status: VMClosureStatus::None,
                    constant_pool_ptr: constant_pool_ptr,
                };
                let block = GCValue::Closure(state.gc.add_block(GCInnerValue::Closure(closure)));
                Value::GCValue(block)
            }
            Constant::Map(v) => {
                let nm:BTreeMap<String,Value> = v.iter().map(|(k,v)|{
                    //TODO: mut引用问题
                    (k.clone(),Value::from_constant(v.clone(),constant_pool_ptr,state))
                }).collect();
                let block = GCValue::Map(state.gc.add_block(GCInnerValue::Map(nm)));
                Value::GCValue(block)
            }
            Constant::Vector(v) => {
                let nv = v.iter().map(|f| Value::from_constant(f.clone(),constant_pool_ptr,state)).collect::<Vec<_>>();
                let block = GCValue::Map(state.gc.add_block(GCInnerValue::Vector(nv)));
                Value::GCValue(block)
            }
            Constant::String(v) => {
                let ns = GCValue::String(state.gc.add_block(GCInnerValue::String(v.clone())));
                Value::GCValue(ns)
            }
            Constant::Opaque(v) => {
                let ns = GCValue::Opaque(state.gc.add_block(GCInnerValue::Opaque(v.clone())));
                Value::GCValue(ns)
            }
            _ => todo!()
        }
    }
    
}

impl Value {
    fn new_null() -> Self {
        Value::Null
    }
    fn new_undef() -> Self {
        Value::Undef
    }
    fn new_boolean(boolean: bool) -> Self {
        Value::Boolean(boolean)
    }
    fn new_f64(f: f64) -> Self {
        Value::F64(f)
    }

    fn new_string(string: String, gc: &mut dyn GC) {
        // TODO: gc implement
    }
    fn new_object(object: BTreeMap<String, Value>, gc: &mut dyn GC) {
        // TODO: gc implement
    }
    fn new_opaque(opaque: Vec<u8>, gc: &mut dyn GC) {
        // TODO: gc implement
    }
}
