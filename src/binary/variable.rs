use super::{
    io::{BinaryRW, Reader, Writer},
    TypeTags,
};
use std::collections::BTreeMap;
// strong type extension
enum Type {
    Mono(TypeTags),
    Record(BTreeMap<String, TypeTags>),
    // last in function is the return value type
    Function(Vec<Type>),
}

pub struct Variable {
    name: u16,
    access: u8,
    init_value: Option<u16>,
    // strong type extension
    // type_info: Type,
}

impl BinaryRW for Variable {
    fn read(reader: &mut Reader) -> Self {
        let name = reader.read_u16();
        let access = reader.read_u8();
        let init_value = reader.read_option(|reader| reader.read_u16());
        Variable {
            name,
            access,
            init_value,
        }
    }

    fn write(&self, write: &mut Writer) {
        write.write_u16(self.name);
        write.write_u8(self.access);
        write.write_option(self.init_value, |write, u| write.write_u16(u));
    }
}
