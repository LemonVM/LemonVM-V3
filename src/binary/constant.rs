use std::collections::HashMap;
pub enum Constant{
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(u64),
    Object(HashMap<String,Constant>),
    Opaque(Vec<u8>),
    // TODO: finish
    #[cfg(BIG_INT)]
    BigInt,
    #[cfg(VALUE_TYPE_REFERENCE)]
    Ref
}