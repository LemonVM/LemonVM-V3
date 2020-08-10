use std::collections::BTreeMap;
use super::gc::*;
enum Value{
    // on stack
    Null,
    Undef,
    Boolean(bool),
    Number(f64),

    // on gc heap
    String(*mut dyn GCObject),
    Object(*mut dyn GCObject),
    Opaque(*mut dyn GCObject),
    // TODO: finish
    #[cfg(BIG_INT)]
    BigInt,
}
impl Value{
    fn new_null() -> Self{
        Value::Null
    }
    fn new_undef() -> Self{
        Value::Undef
    }
    fn new_boolean(boolean:bool) -> Self{
        Value::Boolean(boolean)
    }
    fn new_number(number:f64) -> Self{
        Value::Number(number)
    }

    fn new_string(string:String,gc:&mut dyn GC){
        // TODO: gc implement
    }
    fn new_object(object:BTreeMap<String,Value>,gc:&mut dyn GC){
        // TODO: gc implement
    }
    fn new_opaque(opaque:Vec<u8>,gc:&mut dyn GC){
        // TODO: gc implement
    }
}