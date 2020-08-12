#[repr(u16)]
#[derive(Debug)]
pub enum OpCode{
    // use as debug
    // in debug mode will print the VM runtime info
    NOP = 0x0000,

    // ===== LOAD =====
    // load constant
    LOADK,
    // IMMBOOL 0x01(true) dst
    IMMBOOL,
    // IMMI16 [0x01 0x00 0x00 0x00](1) dst
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
    // RETURN src(from father)
    RETURN,
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