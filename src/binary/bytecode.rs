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
use super::{constant::Constant, TypeTags};
use std::collections::BTreeMap;
pub struct LemonVMByteCodeConstantVariable{
    type_tag: TypeTags,
    name_index: u16,
}

pub struct LemonVMByteCode{
    signature: [u8;5],
    version: u32,
    // format
    // u32 len
    // u16 value(tag data)
    // ...
    constant_pool:Box<BTreeMap<u16,Constant>>,

    // to start main function by uuid
    entry: u64,
}


pub struct LabeledBytecode{
    pub label: u16,
    pub label_len:u16,
    pub label_start_pc:u16,
    pub instructions: Vec<u64>,
}

// impl FunctionBytecode{
//     fn generate_default_bytecode()->Vec<u8>{
//         let mut ret:Vec<u8> = vec![];
//         // uuid
//         ret.append(&mut vec![0x00;8]);
//         // name
//         ret.append(&mut vec![0x00]);
//         // function_type
//         ret.append(&mut vec![0x00]);
//         // args_count
//         ret.append(&mut vec![0x00]);
//         // bytecodes
//         ret.append(&mut vec![0x00]);
//         ret
//     }
// }