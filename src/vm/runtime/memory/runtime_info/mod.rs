use std::{collections::HashMap, sync::Arc, sync::Mutex};

type ConstantPoolRef = u16;
enum TypeInfoType{
    // for example generic module doesnt apply shit
    UNINIT_GENERIC,
    // just type name for example I32
    MONO,
    // typed by fields for example struct A { i32 a } -> {a:I32}
    STRUCTRUAL,
    // functions closures
    ARROR,
}
union TypeInfoVale{
    // uninit generic
    uninit_generic: ConstantPoolRef,
    // mono: tag
    mono: u8,
    // structrual
    structrual: Vec<(Option<ConstantPoolRef>,ConstantPoolRef,RuntimeTypeInfo)>,
    // arrow
    arrow: (Vec<(Option<ConstantPoolRef>,ConstantPoolRef,RuntimeTypeInfo)>,Vec<(ConstantPoolRef,RuntimeTypeInfo)>),
}

pub struct RuntimeTypeInfo{
    // constant pool index
    pub type_name: ConstantPoolRef,
    // type info type
    pub info: u8,
    pub type_info: TypeInfoValue,
}

pub struct RuntimeField{
    pub type_info: RuntimeTypeInfo,
    // if current module is value typed
    pub value_typed_index:u16,  
} 
pub struct RuntimeModule{
    pub constant_pool:Vec<Value>,
    pub field_index_table: HashMap<String,ConstantPoolRef>,
    pub fields : HashMap<ConstantPoolRef,RuntimeField>,
}

pub struct RuntimeThreadInfo {
    pub registers: [Value;64],
    pub pc: u16,
    pub status: VMClosureStatus,
    // after throwing an exception and in that scope there is no exception handler
    // in that case the exception will throw to super function call
    // for saving the exception status adding the closure into the exception_stack
    // when the exception is finally handled the exception stack is renewed
    pub exception_stack: Vec<VMClosure>,
    // in runtime add break point(pc) or removing a break point in current function
    pub break_points: Vec<u16>,

    pub closure: VMClosure,
    pub shared_data: Arc<RuntimeShareData>
}



pub struct RuntimeShareData{
    // thread pool info
    pub modules: Mutex<HashMap<String,RuntimeModule>>,
    pub global_vars: Mutex<HashMap<String,Value>>,
    pub memory_pool: MemoryPool,
    // jit mems

    // enable debug mode will able to see the bytecode executed
    pub debug_mode: bool,
    // enable profile mode will display the memory usage and gc status
    pub profile_mode: bool,
}