use super::io::{BinaryRW, Reader, Writer};
use crate::gen_test_reader_writer_for_type;


#[derive(Debug, Clone, PartialEq)]
pub struct DebugPrecompiledLineNumber {
    start_pc: u16,
    end_pc: u16,
    precompiled_line_number: u16,
}
impl BinaryRW for DebugPrecompiledLineNumber {
    fn read(reader: &mut Reader) -> Self {
        let start_pc = reader.read_u16();
        let end_pc = reader.read_u16();
        let precompiled_line_number = reader.read_u16();
        DebugPrecompiledLineNumber {
            start_pc,
            end_pc,
            precompiled_line_number,
        }
    }
    fn write(&self, writer: &mut Writer) {
        writer.write_u16(self.start_pc);
        writer.write_u16(self.end_pc);
        writer.write_u16(self.precompiled_line_number);
    }
    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>>{
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10{
            ret.push(Box::new(DebugPrecompiledLineNumber{
                start_pc: random(),
                end_pc: random(),
                precompiled_line_number: random(),
            }));
        }
        ret
    }
}
gen_test_reader_writer_for_type!(test_rw_mock_DebugPrecompiledLineNumber,DebugPrecompiledLineNumber);


#[derive(Debug, Clone, PartialEq)]
pub struct DebugPrecompiledLineNumberTable {
    table: Vec<DebugPrecompiledLineNumber>,
}

impl BinaryRW for DebugPrecompiledLineNumberTable {
    fn read(reader: &mut Reader) -> Self {
        let table = reader.read_vec(|reader| DebugPrecompiledLineNumber::read(reader));
        DebugPrecompiledLineNumberTable { table }
    }

    fn write(&self, writer: &mut Writer) {
        writer.write_vec(self.table.clone(), |writer, i| i.write(writer))
    }

    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>>{
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10{
            ret.push(Box::new(DebugPrecompiledLineNumberTable{
                table: DebugPrecompiledLineNumber::mock_data().iter().map(|d| (&**d).clone()).collect()
            }));
        }
        ret
    }
}
gen_test_reader_writer_for_type!(test_rw_mock_DebugPrecompiledLineNumberTable,DebugPrecompiledLineNumberTable);
// such as your code is compiled to bytecode
// JPE 0x000000  // if xxx == true
#[derive(Debug, Clone, PartialEq)]
pub struct DebugCompiledByteCodeComment {
    start_pc: u16,
    end_pc: u16,
    comment: u16,
}

impl BinaryRW for DebugCompiledByteCodeComment {
    fn read(reader: &mut Reader) -> Self {
        let start_pc = reader.read_u16();
        let end_pc = reader.read_u16();
        let comment = reader.read_u16();
        DebugCompiledByteCodeComment {
            start_pc,
            end_pc,
            comment,
        }
    }
    fn write(&self, writer: &mut Writer) {
        writer.write_u16(self.start_pc);
        writer.write_u16(self.end_pc);
        writer.write_u16(self.comment);
    }
    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>>{
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10{
            ret.push(Box::new(DebugCompiledByteCodeComment{
                start_pc: random(),
                end_pc: random(),
                comment: random(),
            }));
        }
        ret
    }
}

gen_test_reader_writer_for_type!(test_rw_mock_DebugCompiledByteCodeComment,DebugCompiledByteCodeComment);

#[derive(Debug, Clone, PartialEq)]
pub struct DebugCompiledByteCodeCommentTable {
    table: Vec<DebugCompiledByteCodeComment>,
}

impl BinaryRW for DebugCompiledByteCodeCommentTable {
    fn read(reader: &mut Reader) -> Self {
        let table = reader.read_vec(|reader| DebugCompiledByteCodeComment::read(reader));
        DebugCompiledByteCodeCommentTable { table }
    }
    fn write(&self, writer: &mut Writer) {
        writer.write_vec(self.table.clone(), |writer, i| i.write(writer))
    }
        // #[cfg(mock)]
        fn mock_data() -> Vec<Box<Self>>{
            use rand::*;
            let mut ret = vec![];
            for _ in 0..10{
                ret.push(Box::new(DebugCompiledByteCodeCommentTable{
                    table: DebugCompiledByteCodeComment::mock_data().iter().map(|d| (&**d).clone()).collect()
                }));
            }
            ret
        }
}
gen_test_reader_writer_for_type!(test_rw_mock_DebugCompiledByteCodeCommentTable,DebugCompiledByteCodeCommentTable);

#[derive(Debug, Clone, PartialEq)]
pub struct DebugVariable {
    pub name: u16,
    pub start_pc: u16,
    pub end_pc: u16,
    pub register: u16,
}
impl BinaryRW for DebugVariable {
    fn read(reader: &mut Reader) -> Self {
        let name = reader.read_u16();
        let start_pc = reader.read_u16();
        let end_pc = reader.read_u16();
        let register = reader.read_u16();
        DebugVariable {
            name,
            start_pc,
            end_pc,
            register,
        }
    }
    fn write(&self, writer: &mut Writer) {
        writer.write_u16(self.name);
        writer.write_u16(self.start_pc);
        writer.write_u16(self.end_pc);
        writer.write_u16(self.register);
    }
    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>>{
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10{
            ret.push(Box::new(DebugVariable{
                name: random(),
                start_pc: random(),
                end_pc: random(),
                register: random(),
            }));
        }
        ret
    }
}
gen_test_reader_writer_for_type!(test_rw_mock_DebugVariable,DebugVariable);
// break points: dynamically adding break points
pub struct DebugBreakPointTable {
    pub table: Vec<u16>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct DebugVariableTable {
    pub table: Vec<DebugVariable>,
}

impl BinaryRW for DebugVariableTable {
    fn read(reader: &mut Reader) -> Self {
        let table = reader.read_vec(|reader| DebugVariable::read(reader));
        DebugVariableTable { table }
    }
    fn write(&self, writer: &mut Writer) {
        writer.write_vec(self.table.clone(), |writer, i| i.write(writer))
    }
            // #[cfg(mock)]
            fn mock_data() -> Vec<Box<Self>>{
                use rand::*;
                let mut ret = vec![];
                for _ in 0..10{
                    ret.push(Box::new(DebugVariableTable{
                        table: DebugVariable::mock_data().iter().map(|d| (&**d).clone()).collect()
                    }));
                }
                ret
            }
}
gen_test_reader_writer_for_type!(test_rw_mock_DebugVariableTable,DebugVariableTable);


#[derive(Debug, Clone, PartialEq)]
pub struct DebugSourceInfo {
    // string source of original programming language
    source: u16,
    source_file_name: u16,
}

impl BinaryRW for DebugSourceInfo {
    fn read(reader: &mut Reader) -> Self {
        let source = reader.read_u16();
        let source_file_name = reader.read_u16();
        DebugSourceInfo {
            source,
            source_file_name,
        }
    }
    fn write(&self, writer: &mut Writer) {
        writer.write_u16(self.source);
        writer.write_u16(self.source_file_name);
    }
    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>>{
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10{
            ret.push(Box::new(DebugSourceInfo{
                source: random(),
                source_file_name: random(),
            }));
        }
        ret
    }
}

gen_test_reader_writer_for_type!(test_rw_mock_DebugSourceInfo,DebugSourceInfo);

#[derive(Debug, Clone, PartialEq)]
pub struct DebugInfo {
    source_info: Option<DebugSourceInfo>,
    variable_table: Option<DebugVariableTable>,
    precompiled_line_number_table: Option<DebugPrecompiledLineNumberTable>,
    comment_table: Option<DebugCompiledByteCodeCommentTable>,
}

impl BinaryRW for DebugInfo {
    fn read(reader: &mut Reader) -> Self {
        let source_info = reader.read_option(|reader| DebugSourceInfo::read(reader));
        let variable_table = reader.read_option(|reader| DebugVariableTable::read(reader));
        let precompiled_line_number_table =
            reader.read_option(|reader| DebugPrecompiledLineNumberTable::read(reader));
        let comment_table =
            reader.read_option(|reader| DebugCompiledByteCodeCommentTable::read(reader));
        DebugInfo {
            source_info,
            variable_table,
            precompiled_line_number_table,
            comment_table,
        }
    }
    fn write(&self, writer: &mut Writer) {
        writer.write_option(self.source_info.clone(), |writer, o| o.write(writer));
        writer.write_option(self.variable_table.clone(), |writer, o| o.write(writer));
        writer.write_option(self.precompiled_line_number_table.clone(), |writer, o| {
            o.write(writer)
        });
        writer.write_option(self.comment_table.clone(), |writer, o| o.write(writer));
    }
    // #[cfg(mock)]
    fn mock_data() -> Vec<Box<Self>>{
        use rand::*;
        let mut ret = vec![];
        for _ in 0..10{
            let source_info = if random(){Some((&*DebugSourceInfo::mock_data()[0]).clone())}else{None};
            let variable_table = if random(){Some((&*DebugVariableTable::mock_data()[0]).clone())}else{None};
            let precompiled_line_number_table = if random(){Some((&*DebugPrecompiledLineNumberTable::mock_data()[0]).clone())}else{None};
            let comment_table = if random(){Some((&*DebugCompiledByteCodeCommentTable::mock_data()[0]).clone())}else{None};
            ret.push(Box::new(DebugInfo{
                source_info,
                variable_table,
                precompiled_line_number_table,
                comment_table
            }));
        }
        ret
    }
}
gen_test_reader_writer_for_type!(test_rw_mock_DebugInfo,DebugInfo);
