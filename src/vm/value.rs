use super::gc::*;
use std::collections::BTreeMap;
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
    String(*mut dyn GCObject),
    Object(*mut dyn GCObject),
    Opaque(*mut dyn GCObject),
    // TODO: finish
    #[cfg(BIG_INT)]
    BigInt,
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
