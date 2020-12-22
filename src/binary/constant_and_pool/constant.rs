use super::{
    function::Function,
    io::{BinaryRW, Reader, Writer},
};
use crate::gen_test_reader_writer_for_type;
use super::tags::*;
// type tag + data
pub union ConstantValue{
    u_8:u8,
    i_8:i8,
    u_16:u16,
    i_16:i16,
    u_32:u32,
    i_32:i32,
    u_64:u64,
    i_64:i64,
    f_32:f32,
    f_64:f64,
    
    string: Vec<u8>,
}
#[derive(Copy, PartialEq)]
pub struct Constant {
    pub tag: u8,
    pub value: ConstantValue
}

impl BinaryRW for Constant {
    fn read(reader: &mut Reader) -> Self {
        let tag = reader.read_u8();
        let value = match tag {
            _ if TypeTags::String as u8 == tag => Constant::String(reader.read_string()),

            _ if U8Type as u8 == tag => Constant::U8(reader.read_u8()),
            _ if I8Type as u8 == tag => Constant::I8(reader.read_i8()),
            _ if U16_Type as u8 == tag => Constant::U16(reader.read_u16()),
            _ if I16_Type as u8 == tag => Constant::I16(reader.read_i16()),
            _ if U32_Type as u8 == tag => Constant::U32(reader.read_u32()),
            _ if I32_Type as u8 == tag => Constant::I32(reader.read_i32()),
            _ if U64_Type as u8 == tag => Constant::U64(reader.read_u64()),
            _ if I64_Type as u8 == tag => Constant::I64(reader.read_i64()),
            _ if F32_Type as u8 == tag => Constant::F32(reader.read_f32()),
            _ if F64_Type as u8 == tag => Constant::F64(reader.read_f64()),

            _ if TypeTags::Function as u8 == tag => Constant::Function(Function::read(reader)),
            _ => unimplemented!(),
        };
        Constant{
            tag,
            value
        }
    }

    fn write(&self, writer: &mut Writer) {
        match self {
            Constant::String(v) => {
                writer.write_u8(TypeTags::String as u8);
                writer.write_string(v.clone());
            }
            Constant::Map(v) => {
                writer.write_u8(TypeTags::Map as u8);
                writer.write_map(v.clone(), |write, (k, v): (String, Constant)| {
                    write.write_string(k.clone());
                    v.write(write);
                });
            }
            Constant::U8(v) => {
                writer.write_u8(TypeTags::U8 as u8);
                writer.write_u8(*v);
            }
            Constant::I8(v) => {
                writer.write_u8(TypeTags::I8 as u8);
                writer.write_u8(*v as u8);
            }
            Constant::U16(v) => {
                writer.write_u8(TypeTags::U16 as u8);
                writer.write_u16(*v as u16);
            }
            Constant::I16(v) => {
                writer.write_u8(TypeTags::I16 as u8);
                writer.write_u16(*v as u16);
            }
            Constant::U32(v) => {
                writer.write_u8(TypeTags::U32 as u8);
                writer.write_u32(*v as u32);
            }
            Constant::I32(v) => {
                writer.write_u8(TypeTags::I32 as u8);
                writer.write_u32(*v as u32);
            }
            Constant::U64(v) => {
                writer.write_u8(TypeTags::U64 as u8);
                writer.write_u64(*v as u64);
            }
            Constant::I64(v) => {
                writer.write_u8(TypeTags::I64 as u8);
                writer.write_u64(*v as u64);
            }
            Constant::F32(v) => {
                writer.write_u8(TypeTags::F32 as u8);
                writer.write_f32(*v);
            }
            Constant::F64(v) => {
                writer.write_u8(TypeTags::F64 as u8);
                writer.write_f64(*v);
            }
            Constant::Vector(v) => {
                writer.write_u8(TypeTags::Vector as u8);
                writer.write_vec(v.clone(), |writer, v| v.write(writer));
            }
            Constant::Function(v) => {
                writer.write_u8(TypeTags::Function as u8);
                v.write(writer);
            }
            Constant::Opaque(v) => {
                writer.write_u8(TypeTags::Opaque as u8);
                writer.write_vec(v.clone(), |r, v| r.write_u8(v));
            }
        }
    }

    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>> {
        use crate::binary::io::*;
        use rand::*;
        vec![
            Box::new(Constant::String(mock_string())),
            Box::new(Constant::U8(random())),
            Box::new(Constant::I8(random())),
            Box::new(Constant::U16(random())),
            Box::new(Constant::I16(random())),
            Box::new(Constant::U32(random())),
            Box::new(Constant::I32(random())),
            Box::new(Constant::U64(random())),
            Box::new(Constant::I64(random())),
            Box::new(Constant::F32(random())),
            Box::new(Constant::F64(random())),
            Box::new(Constant::Map(mock_object(&|| mock_string(), &|| {
                Constant::U8(random())
            }))),
            Box::new(Constant::Function((&*Function::mock_data()[0]).clone())),
            Box::new(Constant::Vector(vec![
                Constant::U8(random()),
                Constant::U64(random()),
                Constant::Map(mock_object(&|| mock_string(), &|| Constant::U8(random()))),
                Constant::Function((&*Function::mock_data()[0]).clone()),
            ])),
        ]
    }
}
gen_test_reader_writer_for_type!(test_rw_mock_Constant, Constant);
