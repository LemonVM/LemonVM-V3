use super::io::{Reader, BinaryRW};
pub struct DebugPrecompiledLineNumber{
    start_pc: u16,
    end_pc: u16,
    precompiled_line_number:u16,
}

impl BinaryRW for DebugPrecompiledLineNumber{
    fn read(reader:&mut Reader) -> Self {
        let start_pc = reader.read_u16();
        let end_pc = reader.read_u16();
        let precompiled_line_number = reader.read_u16();
        DebugPrecompiledLineNumber{
            start_pc,end_pc,precompiled_line_number
        }
    }
}

pub struct DebugPrecompiledLineNumberTable{
    table:Vec<DebugPrecompiledLineNumber>
}

impl BinaryRW for DebugPrecompiledLineNumberTable{
    fn read(reader:&mut Reader) -> Self {
        let table = reader.read_vec(|reader|{
            DebugPrecompiledLineNumber::read(reader)
        });
        DebugPrecompiledLineNumberTable{
            table
        }
    }
}

// such as your code is compiled to bytecode
// JPE 0x000000  // if xxx == true
pub struct DebugCompiledByteCodeComment{
    start_pc: u16,
    end_pc: u16,
    comment: u16,
}

impl BinaryRW for DebugCompiledByteCodeComment{
    fn read(reader:&mut Reader) -> Self {
        let start_pc = reader.read_u16();
        let end_pc = reader.read_u16();
        let comment = reader.read_u16();
        DebugCompiledByteCodeComment{
            start_pc,end_pc,comment
        }
    }
}

pub struct DebugCompiledByteCodeCommentTable{
    table:Vec<DebugCompiledByteCodeComment>
}

impl BinaryRW for DebugCompiledByteCodeCommentTable{
    fn read(reader:&mut Reader) -> Self {
        let table = reader.read_vec(|reader|{
            DebugCompiledByteCodeComment::read(reader)
        });
        DebugCompiledByteCodeCommentTable{
            table
        }
    }
}

pub struct DebugVariable{
    name:u16,
    start_pc:u16,
    end_pc:u16,
}

impl BinaryRW for DebugVariable{
    fn read(reader:&mut Reader) -> Self {
        let name = reader.read_u16();
        let start_pc = reader.read_u16();
        let end_pc = reader.read_u16();
        DebugVariable{
            name,start_pc,end_pc
        }
    }
}

pub struct DebugVariableTable{
    table:Vec<DebugVariable>,
}

impl BinaryRW for DebugVariableTable{
    fn read(reader:&mut Reader) -> Self {
        let table = reader.read_vec(|reader|{
            DebugVariable::read(reader)
        });
        DebugVariableTable{
            table
        }
    }
}

pub struct DebugSourceInfo{
    // string source of original programming language
    source: u16,
    source_file_name: u16,
}

impl BinaryRW for DebugSourceInfo{
    fn read(reader:&mut Reader) -> Self {
        let source = reader.read_u16();
        let source_file_name = reader.read_u16();
        DebugSourceInfo{
            source,source_file_name
        }
    }
}

pub struct DebugInfo{
    source_info: Option<DebugSourceInfo>,
    variable_table: Option<DebugVariableTable>,
    precompiled_line_number_table: Option<DebugPrecompiledLineNumberTable>,
    comment_table: Option<DebugCompiledByteCodeCommentTable>,
}

impl BinaryRW for DebugInfo{
    fn read(reader:&mut Reader) -> Self {
        let source_info = reader.read_option(|reader|{
            DebugSourceInfo::read(reader)
        });
        let variable_table = reader.read_option(|reader|{
            DebugVariableTable::read(reader)
        });
        let precompiled_line_number_table = reader.read_option(|reader|{
            DebugPrecompiledLineNumberTable::read(reader)
        });
        let comment_table = reader.read_option(|reader|{
            DebugCompiledByteCodeCommentTable::read(reader)
        });
        DebugInfo{
            source_info,
            variable_table,
            precompiled_line_number_table,
            comment_table,
        }
    }
}