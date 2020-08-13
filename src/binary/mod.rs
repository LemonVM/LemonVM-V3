/*
    A LMVMB file is either a executable lemonvm binary or
    a library
*/
pub mod bytecode;
pub mod constant;
pub mod debug;
pub mod function;
pub mod io;
pub mod opcode;
pub mod variable;

mod test;

// TODO: More premetive types
#[repr(u8)]
pub enum TypeTags {
    // Primitives
    Undefined = 0,
    Null,
    String,
    Symbol,
    // Object
    Object,
    // primitive function
    Function,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    // use to make typed array
    Opaque,
    // With extension
    #[cfg(BIG_INT)]
    BigInt,
    #[cfg(VALUE_TYPE_REFERENCE)]
    REF,
    // strong type extension
    OBJStart,
    KeyName,
    ValueType,
    OBJEnd,
}
