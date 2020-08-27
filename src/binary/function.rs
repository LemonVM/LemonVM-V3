/*
    Function is just a thing storage
    in constant pool, is not the function
    actually runs in vm, that is called closure
*/

use super::constant::*;
use super::{
    debug::*,
    io::{BinaryRW, Reader, Writer},
};
use crate::gen_test_reader_writer_for_type;
use std::collections::BTreeMap;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
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
    fn write(&self, writer: &mut super::io::Writer) {
        writer.write_u8(self.clone() as u8);
    }
    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>> {
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10 {
            //TODO: when all implemented change that
            ret.push(Box::new(FunctionType::Function));
        }
        ret
    }
}

gen_test_reader_writer_for_type!(test_rw_mock_FunctionType, FunctionType);

// if pc in range from to pc and state is
// error then use jump to handler pc
#[derive(Debug, Clone, PartialEq)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
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
    fn write(&self, writer: &mut super::io::Writer) {
        writer.write_u16(self.start_pc);
        writer.write_u16(self.end_pc);
        writer.write_u16(self.handler_pc);
    }

    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>> {
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10 {
            ret.push(Box::new(Exception {
                start_pc: random(),
                end_pc: random(),
                handler_pc: random(),
            }));
        }
        ret
    }
}

gen_test_reader_writer_for_type!(test_rw_mock_Exception, Exception);

#[derive(Debug, Clone, PartialEq)]
pub struct ExceptionTable {
    pub table: Vec<Exception>,
}

impl BinaryRW for ExceptionTable {
    fn read(reader: &mut Reader) -> Self {
        let table = reader.read_vec(|reader| Exception::read(reader));
        ExceptionTable { table }
    }
    fn write(&self, writer: &mut super::io::Writer) {
        writer.write_vec(self.table.clone(), |writer, o| Exception::write(&o, writer));
    }
    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>> {
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10 {
            ret.push(Box::new(ExceptionTable {
                table: Exception::mock_data()
                    .iter()
                    .map(|d| (&**d).clone())
                    .collect(),
            }));
        }
        ret
    }
}

gen_test_reader_writer_for_type!(test_rw_mock_ExceptionTable, ExceptionTable);

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    //TODO: prob add an option name
    is_multi_return_function: bool,
    function_type: FunctionType,
    // for example
    // args count is 3 f(a,b,c) will automatically use 4 register
    // the forth one is vargs
    pub args_count: u8,
    pub max_registers: u16,
    pub code: Vec<u64>,

    pub exception_table: Option<ExceptionTable>,
    pub debug_info: Option<DebugInfo>,
}

impl BinaryRW for Function {
    fn read(reader: &mut Reader) -> Self {
        let is_multi_return_function = reader.read_u8() != 0x00;
        let function_type = FunctionType::read(reader);
        let args_count = reader.read_u8();
        let max_registers = reader.read_u16();
        let code = reader.read_vec(|reader| reader.read_u64());
        let exception_table = reader.read_option(|reader| ExceptionTable::read(reader));
        let debug_info = reader.read_option(|reader| DebugInfo::read(reader));
        Function {
            is_multi_return_function,
            function_type,
            args_count,
            max_registers,
            code,
            exception_table,
            debug_info,
        }
    }
    fn write(&self, writer: &mut super::io::Writer) {
        writer.write_u8(if self.is_multi_return_function {
            0xFF
        } else {
            0x00
        });
        FunctionType::write(&self.function_type, writer);
        writer.write_u8(self.args_count);
        writer.write_u16(self.max_registers);
        writer.write_vec(self.code.clone(), |writer, v| writer.write_u64(v));
        writer.write_option(self.exception_table.clone(), |writer, o| {
            ExceptionTable::write(&o, writer)
        });
        writer.write_option(self.debug_info.clone(), |writer, o| {
            DebugInfo::write(&o, writer)
        });
    }

    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>> {
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10 {
            let mut vecu64 = vec![];
            for i in 0u8..random() {
                vecu64.push(random());
            }
            let is_multi_return_function = random();
            let function_type = (&*FunctionType::mock_data()[0]).clone();
            let args_count = random();
            let max_registers = random();
            let code = vecu64;
            let exception_table = None;
            let debug_info = if random() {
                Some((&*DebugInfo::mock_data()[0]).clone())
            } else {
                None
            };
            ret.push(Box::new(Function {
                is_multi_return_function,
                function_type,
                args_count,
                max_registers,
                code,
                exception_table,
                debug_info,
            }));
        }
        ret
    }
}

gen_test_reader_writer_for_type!(test_rw_mock_Function, Function);
