/*
    Function is just a thing storage
    in constant pool, is not the function
    actually runs in vm, that is called closure
*/

use super::constant::*;
use super::{
    debug::*,
    io::{BinaryRW, Reader},
};
use std::collections::BTreeMap;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum FunctionType {
    Function = 0x00,
    // currently disabled
    Generator,
    // currently disabled
    AsyncFunction,
    // currently disabled
    AsyncGenerator,
}

impl BinaryRW for FunctionType {
    fn read(reader: &mut Reader) -> Self {
        let tag = reader.read_u8();
        match tag {
            t if tag == FunctionType::Function as u8 => FunctionType::Function,
            _ => unimplemented!(),
        }
    }
    fn write(&self, write: &mut super::io::Writer) {
        todo!()
    }
}

// if pc in range from to pc and state is
// error then use jump to handler pc
#[derive(Debug, Clone)]
struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
}

impl BinaryRW for Exception {
    fn read(reader: &mut Reader) -> Self {
        let start_pc = reader.read_u16();
        let end_pc = reader.read_u16();
        let handler_pc = reader.read_u16();
        Exception {
            start_pc,
            end_pc,
            handler_pc,
        }
    }
    fn write(&self, write: &mut super::io::Writer) {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct ExceptionTable {
    table: Vec<Exception>,
}

impl BinaryRW for ExceptionTable {
    fn read(reader: &mut Reader) -> Self {
        let table = reader.read_vec(|reader| Exception::read(reader));
        ExceptionTable { table }
    }
    fn write(&self, write: &mut super::io::Writer) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    function_type: FunctionType,
    // indexed by uuid
    // just used to build arguments object
    args_count: u8,
    max_registers: u16,
    pub code: Vec<u64>,

    exception_table: Option<ExceptionTable>,
    debug_info: Option<DebugInfo>,
}

impl BinaryRW for Function {
    fn read(reader: &mut Reader) -> Self {
        let function_type = FunctionType::read(reader);
        let args_count = reader.read_u8();
        let max_registers = reader.read_u16();
        let code = reader.read_vec(|reader| reader.read_u64());
        let exception_table = reader.read_option(|reader| ExceptionTable::read(reader));
        let debug_info = reader.read_option(|reader| DebugInfo::read(reader));
        Function {
            function_type,
            args_count,
            max_registers,
            code,
            exception_table,
            debug_info,
        }
    }
    fn write(&self, write: &mut super::io::Writer) {
        todo!()
    }
}
