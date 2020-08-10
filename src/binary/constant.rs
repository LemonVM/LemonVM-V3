use super::{function::Function, TypeTags};
use std::collections::BTreeMap;
pub enum Constant{
    String(String),
    Object(BTreeMap<String,Constant>),
    Opaque(Vec<u8>),
    Function(Function),
    // TODO: finish
    #[cfg(BIG_INT)]
    BigInt,
}