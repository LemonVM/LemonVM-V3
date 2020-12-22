macro_rules! ins {
    ($id:ident,$value:expr) => {
        const $id:u16 = $value;
    };
}

// 0000 NOP
// 0001-0010 LOAD
// 0010-0040 ARITH
// 0040-0060 CONTROL FLOW

// 0060-F000 do what ever

// F000-FFFF FLAG FOR NEXT INSTRUCTION


// ===== FLAGS =====
// safe arith
// check value that make sure is not overflow
// check for minus unsigned value
// check for div 0
ins!(SA,0xF000);
// the next call instruction will call a closure
// CLOSCALL src
// stop current vm thread, store it, then modifies it to the vmstate from closure
// call the closure, then restore the info saved in the last vm thread.
ins!(CLOSCALL,0xF001);

ins!(LIMEINSB,0xFF00);
ins!(LIMEINSE,0xFF01);



// use as debug
// in debug mode will print the VM runtime info
ins!(NOP,0x0000);

// ===== LOAD =====
// load constant
// LOADK dst [index] [module_name_index or 0xFFFF(local)]
ins!(LOADK,0x0001);
// ===== IMMLOAD =====
// used for performance
// IMMBOOL dst 0x0100(true) 0x0000
ins!(IMMBOOL,0x0002);
// IMMI16 dst [0x01 0x00 0x00 0x00](1)
ins!(IMMU8,0x0003);
ins!(IMMI8,0x0004);
ins!(IMMU16,0x0005);
ins!(IMMI16,0x0006);
ins!(IMMU32,0x0007);
ins!(IMMI32,0x0008);
ins!(IMMF32,0x0009);
ins!(IMMNULL,0x000A);

// ===== ARITH =====
// enable operator overload will influence the performance
// due to it looks table rather than directly doing the arith

// Ix will neg imm
// Ux will force cast to Ix then neg
// NEG dst src
ins!(NEG,0x0010);
// format INS dst src1 src2
ins!(ADD,0x0011);
ins!(SUB,0x0012);
ins!(MUL,0x0013);
ins!(REM,0x0014);
ins!(DIV,0x0015);

ins!(BNOT,0x002F);
ins!(BAND,0x0030);
ins!(BXOR,0x0031);
ins!(BOR,0x0032);

ins!(SHL,0x0033);
ins!(SHR,0x0034);

ins!(NOT,0x0035);
ins!(AND,0x0036);
ins!(OR,0x0037);
ins!(EQ,0x0038);
ins!(LT,0x0039);
ins!(GT,0x003A);
ins!(LTEQ,0x003B);
ins!(GTEQ,0x003C);

ins!(SWAP,0x003D);

// ===== CONTROL FLOW =====
// normally won't generate
// JMP [16bit label]
ins!(JMP,0x0040);
// if true
// JPE src [16bit label]
ins!(JPE,0x0041);
// if false
// JPN src [16bit label]
ins!(JPN,0x0042);
// ===== FUNCTION? =====
// pushing a new call frame in local thread
// PUSHF
ins!(PUSHF,0x0043);
// pushing a new var into the tail of call frame
// PUSHARG src
ins!(PUSHARG,0x0044);
// pusing a named var into the tail of call frame
// PUSHNARG src
ins!(PUSHNARG,0x0045);
// get a arg from the vmstate
// PUSHARG src
ins!(GETARG,0x0044);
// get a names arg from the vmstate
// PUSHNARG src
ins!(GETNARG,0x0045);

// CALL
// enter the tail of call frame
ins!(CALL,0x0046);

// INVOKEDYNAMIC src
// enter the module and use the predefined invoke function to find the function to invoke
ins!(INVOKEDYNAMIC,0x0047);
// INVOKEVIRTUAL src
// enter the module and directly look up the function from the virtual table then invoke
ins!(INVOKEVIRTUAL,0x0048);

// RET src
ins!(RET,0x0049);


// diff between getret|pushret and getretn|pushretn
// pushret will push a single value and only accessable with getret
// pushretn will save the return values into a vec in vmstate and getretn will get a vec

// this ins is unstable, so normally will not generate
// TAILCALL args-reg
//ins!(TAILCALL,
// GETRET will get a return value from VMState
ins!(GETRET,0x004A);
// GETRETN will get a vec of return values from VMState
ins!(GETRETN,0x004B);

// PUSHRET src
// push the single return value into the vmstate
ins!(PUSHRET,0x4C);
// PUSHRETN src
// push the multiple return values into the vmstate
ins!(PUSHRETN,0x4D);

// raise an error into VM status
// check next exception table in current function
// if not then goto last function call satck with last pc and check recursively
ins!(ERROR,0x004E);


// ===== REFLECTIONS =====
// load module into vm
ins!(LOADMODULE,0x0060);
// get a member return to dst register
// GETMEMBER dst 
ins!(GETMEMBER,0x0061);
// insert a new member/modifies a member in module
ins!(SETMEMBER,0x0062);
// get a static member return to dst register
// GETMEMBER dst 
ins!(GETSTATICMEMBER,0x0063);
// insert a static new member/modifies a member in module
ins!(SETSTATICMEMBER,0x0064);
// check the type of first input is equal to second input type
// returns a bool
// ISINSTANCEOF dst src src
ins!(ISINSTANCEOF,0x0065);

// ISMULTIRETURN dst src
// need to input 
ins!(ISMULTIRETURN,0x0066);

// ===== OBJECT and MODULE =====
//TODO: internal array operations
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
