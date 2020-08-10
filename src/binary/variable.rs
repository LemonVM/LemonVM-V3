use super::{io::{Reader, BinaryRW}, TypeTags};
use std::collections::BTreeMap;
// strong type extension
enum Type{
    Mono(TypeTags),
    Record(BTreeMap<String,TypeTags>),
    // last in function is the return value type
    Function(Vec<Type>)
}

pub struct Variable{
    name:u16,
    access: u8,
    init_value: Option<u16>,
    // strong type extension
    // type_info: Type,
}

impl BinaryRW for Variable{
    fn read(reader:&mut Reader) -> Self {
        let name = reader.read_u16();
        let access = reader.read_u8();
        let init_value = reader.read_option(|reader|reader.read_u16());
        Variable{
            name,access,init_value
        }
    }
}