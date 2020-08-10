/*
    A LMVMB file is either a executable lemonvm binary or
    a library
*/
pub mod constant;
pub mod bytecode;
pub mod function;
pub mod variable;
pub mod debug;
pub mod io;
// TODO: More premetive types
#[repr(u8)]
pub enum TypeTags{
    // Primitives
        Undefined = 0,
        Null,
        Boolean,
        Number,
        String,
        Symbol,
    // Object
        Object,
        // primitive function
        Function,
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