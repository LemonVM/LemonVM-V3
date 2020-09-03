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

    // ===== ARITH =====
    // enable operator overload will influence the performance
    // due to it looks table rather than directly doing the arith

    // Ix will neg imm
    // Ux will cast to Ix then neg
    // NEG dst
    NEG,
    // Ix will neg imm
    // Ux and other will panic
    // TODO: throw exception
    SNEG,
    // INT will -1
    // FLOAT will -1.0
    // DEC dst
    DEC,
    // INT will +1
    // FLOAT will +1.0
    // DEC dst
    INC,
    // DEC but checking for overflow and
    // non int value type not allowed
    SDEC,
    // similar but inc
    SINC,

    // format INS src1 src2 dst
    ADD8,
    SUB8,
    MUL8,
    REM8,
    DIV8,

    ADD16,
    SUB16,
    MUL16,
    REM16,
    DIV16,

    ADD32,
    SUB32,
    MUL32,
    REM32,
    DIV32,

    ADD64,
    SUB64,
    MUL64,
    REM64,
    DIV64,

    // safe arith
    // check value that make sure is not overflow
    // check for minus unsigned value
    // check for div 0
    SADD8,
    SSUB8,
    SMUL8,
    SREM8,
    SDIV8,

    SADD16,
    SSUB16,
    SMUL16,
    SREM16,
    SDIV16,

    SADD32,
    SSUB32,
    SMUL32,
    SREM32,
    SDIV32,

    SADD64,
    SSUB64,
    SMUL64,
    SREM64,
    SDIV64,

    // FLOATING ARITH
    ADDF32,
    SUBF32,
    MULF32,
    REMF32,
    DIVF32,

    ADDF64,
    SUBF64,
    MULF64,
    REMF64,
    DIVF64,

    BNOT,
    BAND,
    BXOR,
    BOR,
    SHL,
    SHR,

    NOT,
    LNOT,
    AND,
    LAND,
    OR,
    LOR,

    EQ,
    SEQ,

    LT,
    GT,
    LTEQ,
    GTEQ,

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
    // ===== FUNCTION? =====
    // note: all functions in vm ONLY has one param called argument
    // and that is storaged in VMState
    // add src to args from VMState
    // ARGS src
    ARGS,
    // NARGS src(name) src(value)
    // TODO: implement
    NARGS,
    // after call args will be cleaned
    // CALL clos-reg
    CALL,
    // this ins is unstable, so normally will not generate
    // TAILCALL args-reg
    // TAILCALL,
    // TODO: implement
    CALLCONS,
    // TODO: implement
    CALLMETHOD,
    GETRET,
    // RET 0xFFFF => return void
    // RET src
    // set return value to the VMState
    RET,
    // RETN src(RETS)
    // append return value into the VMState
    // usage: RETN 0x0100; RETN 0x0200; RET 0xFFFF;
    RETN,
    // raise an error into VM status
    // check next exception table in current function
    // if not then goto last function call satck with last pc and check recursively
    ERROR,

    // ===== TEMP CONTAINER =====
    // container object managed by rust instead of managed by gc
    // make dst a vec object, if already is , then push src to dst
    // VEC dst src
    VEC,

    TOGC,

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
}
