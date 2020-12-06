use super::{gc::*, VMClosure, VMClosureStatus, VMState};
use crate::binary::constant::Constant;
use std::{collections::BTreeMap, ptr::NonNull};

use std::mem::ManuallyDrop;

pub union Value{
    _bool:bool,
    u_8:u8,
    i_8:i8,
    
}

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
    // on heap but not escaped (managed by rust)
    NSValue(NSValue),
    // on gc heap
    GCValue(GCValue),
}