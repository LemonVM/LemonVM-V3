#[repr(u16)]
#[derive(Debug)]
pub enum OpCode {
    // use as debug
    // in debug mode will print the VM runtime info
    NOP = 0x0000,

    // ===== LOAD =====
    // load constant
    // LOADK dst [index] [module_name_index or 0xFFFF(local)]
    LOADK,
    // IMMBOOL dst 0x0100(true) 0x0000
    IMMBOOL,
    // IMMI16 dst [0x01 0x00 0x00 0x00](1)
    IMMU8,
    IMMI8,
    IMMU16,
    IMMI16,
    IMMU32,
    IMMI32,
    // IMMX64 dst [32bit
    // EXTRA  32bit] 16bit
    IMMU64,
    IMMI64,
    IMMF32,
    IMMF64,

    // IMMSTR only used to load short string for saving time of indexing constant pool
    // similar to IMMF64
    IMMSTR,
    // EXTRA [48bit data]
    EXTRA,

    // ===== FUNCTION? =====
    // load a function into register
    CLOSURE,
    // make dst an args object, if already is , then add another arg
    // ARGS src dst
    ARGS,
    // use to set multiple return value
    RETS,

    // ===== OBJECT and MODULE =====
    // An object is an runtime defined anonymous object
    // or loaded a dynamic(which means has state) module(lmvmb file loaded) instance
    
    // LOADMODULE will load module into the vmstate
    // LOADMODULE [module_name_index or 0xFFFF(local)]
    LOADMODULE,
    // NEWOBJM dst [module_name_index or 0xFFFF(local)]
    NEWOBJM,
    SETV,
    ADDKV,
    INDEXV,
    FINDKBYV,
    // NEWOBJ dst
    NEWOBJ,

    // ===== CONTROL FLOW =====
    // normally won't generate
    // JMP [16bit label]
    JMP,
    // if true
    // JPE src [16bit label]
    JPE,
    // if false
    // JPN src [16bit label]
    JPN,
    // note all functions in vm ONLY has one param called arguments
    // CALL clos-reg args-reg ret-reg
    CALL,
    // this ins is unstable, so normally will not generate
    // TAILCALL args-reg
    // TAILCALL,
    CALLCONS,
    CALLMETHOD,
    // RET no return value
    RET,
    // RETURN src dst(from father)
    RETURN,
    // RETURNM src(RETS) dst(from father)
    RETURNM, 
    // raise an error into VM status
    // check next exception table in current function
    // if not then goto last function call satck with last pc and check recursively
    ERROR,
    // ===== ARITH =====
    // enable operator overload will influence the performance
    // due to it looks table rather than directly doing the arith
    NEG,
    PLUS,
    DEC,
    INC,
    POSTDEC,
    POSTINC,

    ADD,
    SUB,
    MUL,
    MOD,
    DIV,

    // safe arith
    ADDS,
    SUBS,
    MULS,
    MODS,
    DIVS,

    BNOT,
    BAND,
    BOR,
    BXOR,
    SHL,
    SHR,

    NOT,
    LNOT,

    EQ,
    SEQ,
    NEQ,
    NSEQ,

    LT,
    GT,
    LTEQ,
}
