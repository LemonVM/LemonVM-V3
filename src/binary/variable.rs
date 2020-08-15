use super::{
    io::{BinaryRW, Reader, Writer},
    TypeTags,
};
use std::collections::BTreeMap;
// strong type extension
#[derive(Debug,Clone)]
enum Type {
    Mono(TypeTags),
    Record(BTreeMap<String, TypeTags>),
    // last in function is the return value type
    Function(Vec<Type>),
}

#[derive(Debug,Clone)]
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

    fn write(&self, writer: &mut Writer) {
        writer.write_u16(self.name);
        writer.write_u8(self.access);
        writer.write_option(self.init_value, |write, u| write.write_u16(u));
    }

    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>>{
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10{
            ret.push(Box::new(Variable{
                name: random(),
                access: random(),
                init_value: if random(){Some(random())}else{None},
            }));
        }
        ret
    }
}
