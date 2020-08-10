use super::{function::Function, TypeTags, io::{Reader, BinaryRW}};
use std::collections::BTreeMap;
// type tag + data
pub enum Constant{
    String(String),
    Object(BTreeMap<String,Constant>),
    Opaque(Vec<u8>),
    Function(Function),
    // TODO: finish
    #[cfg(BIG_INT)]
    BigInt,
}

impl BinaryRW for Constant{
    fn read(reader:&mut Reader) -> Self {
        let tag = reader.read_u8();
        match tag{
            t if TypeTags::String as u8 == tag => {
                Constant::String(reader.read_string())
            }
            t if TypeTags::Object as u8 == tag => {
                Constant::Object(reader.read_map(|reader|{
                    (reader.read_string(),Constant::read(reader))
                }))
            }
            t if TypeTags::Opaque as u8 == tag => {
                Constant::Opaque(reader.read_vec(|reader|reader.read_u8()))
            }
            t if TypeTags::Function as u8 == tag => {
                Constant::Function(Function::read(reader))
            }
            _ => unimplemented!()
        }
    }
}