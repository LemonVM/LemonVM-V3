/*
    an instruction is normally 64 bit long
    
    ins          rega      regb      regc
    0x00 0x00    0xAA 0xAA 0xBB 0xBB 0xCC 0xCC

    but abnormally some ins uses more than 48 bit data
    ins          data-start:
    0x00 0x00    0x00 0x00 0x00 0x00 0x00 0x00
    ins EXTRA
    0x00 0xAA    0x00 0x00 0x00 0x00 0x00 0x00

    until the next instruction is not start with 0xAA
    the data is fully loaded
*/
use super::{constant::Constant, TypeTags,variable::Variable, io::{Reader, BinaryRW}};
use std::collections::BTreeMap;

pub struct LemonVMByteCode{
    signature: [u8;5],
    version: u32,
    enabled_extensions: Vec<u8>,
    // format
    // u32 len
    // u16 value(tag data)
    // ...
    constant_pool:BTreeMap<u16,Constant>,

    // to index the main function from constant_pool by uuid
    entry: u16,
    variables: Vec<Variable>
}

impl BinaryRW for LemonVMByteCode {
    fn read(reader:&mut Reader) -> Self {
        let s1 = reader.read_u8();
        let s2 = reader.read_u8();
        let s3 = reader.read_u8();
        let s4 = reader.read_u8();
        let s5 = reader.read_u8();
        let signature = [s1,s2,s3,s4,s5];
        let version = reader.read_u32();
        let enabled_extensions = reader.read_vec(|reader|reader.read_u8());
        let constant_pool = reader.read_map(|reader|{
            (reader.read_u16(), Constant::read(reader))
        });
        let entry = reader.read_u16();
        let variables = reader.read_vec(|reader| Variable::read(reader));
        LemonVMByteCode{
            signature,
            version,
            enabled_extensions,
            constant_pool,
            entry,
            variables
        }
    }
}