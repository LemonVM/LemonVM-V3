pub struct DebugPrecompiledLineNumber{
    start_pc: u16,
    end_pc: u16,
    precompiled_line_number:u16,
}

pub struct DebugPrecompiledLineNumberTable{
    table:Vec<DebugPrecompiledLineNumber>
}

// such as your code is compiled to bytecode
// JPE 0x000000  // if xxx == true
pub struct DebugCompiledByteCodeComment{
    start_pc: u16,
    end_pc: u16,
    comment: u16,
}

pub struct DebugCompiledByteCodeCommentTable{
    table:Vec<DebugCompiledByteCodeComment>
}

pub struct DebugVariable{
    name:u16,
    start_pc:u16,
    end_pc:u16,
}

pub struct DebugVariableTable{
    table:Vec<DebugVariable>,
}

pub struct DebugSourceInfo{
    // string source of original programming language
    source: u16,
    source_file_name: u16,
}

pub struct DebugInfo{
    source_info: Option<DebugSourceInfo>,
    variable_table: Option<DebugVariableTable>,
    comment_table: Option<DebugCompiledByteCodeCommentTable>,
    precompiled_line_number_table: Option<DebugPrecompiledLineNumberTable>,
}