use super::register;
use crate::config::*;

use super::{VMState, value::Value};
use crate::{
    binary::{function::Function, opcode::OpCode, constant::Constant},
    config::*,
};
use std::{ptr::NonNull, collections::BTreeMap};

macro_rules! expr {
    ($e:expr) => {
        $e
    }
}
macro_rules! TRI_INS_X {
    ($regs:ident,$e1:ident,$e2:ident,$e3:ident,$t:ty,$t2:ty,$vc1:ident,$vc2:ident,$op:tt) => {
        let mut sized = false;
        let v1 = match $regs[$e1 as usize]{
            Value::U8(v) => {v as $t},
            Value::I8(v) => {sized = true;v as $t},
            Value::U16(v) => {v as $t},
            Value::I16(v) => {sized = true;v as $t},
            Value::U32(v) => {v as $t},
            Value::I32(v) => {sized = true;v as $t},
            Value::U64(v) => {v as $t},
            Value::I64(v) => {sized = true;v as $t},
            Value::F32(v) => {sized = true;v as $t},
            Value::F64(v) => {sized = true;v as $t},
            _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
        };
        let v2 = match $regs[$e2 as usize]{
            Value::U8(v) => {v as $t},
            Value::I8(v) => {sized = true;v as $t},
            Value::U16(v) => {v as $t},
            Value::I16(v) => {sized = true;v as $t},
            Value::U32(v) => {v as $t},
            Value::I32(v) => {sized = true;v as $t},
            Value::U64(v) => {v as $t},
            Value::I64(v) => {sized = true;v as $t},
            Value::F32(v) => {sized = true;v as $t},
            Value::F64(v) => {sized = true;v as $t},
            _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
        };
        if sized{
            $regs[$e3 as usize] = Value::$vc1(expr!(v1 as $t2 $op v2 as $t2));
        }else{
            $regs[$e3 as usize] = Value::$vc2(expr!(v1 $op v2));
        }
    };
}

fn interpreter(state: &mut VMState) {
    // init
    let current_function_state = state.current_function_call_state.clone();
    if current_function_state.registers.len() > MAX_REGISTER_ON_STACK {
        panic!("ERROR! PLATFORM DOES NOT SUPPORT THAT MUCH REGISTER")
    }
    let codes = current_function_state.function_bytecode.code;

    // load registers
    let mut stack_regs: [Value; MAX_REGISTER_ON_STACK] = [Value::Undef; 64];
    //  copy register to stack
    for r in 0..current_function_state.registers.len() {
        stack_regs[r] = current_function_state.registers[r].clone();
    }
    let mut pc = current_function_state.pc;
    loop {
        // on exception
        // on breakpoint
        // on whatever

        // on end function scope
        if pc >= codes.len() as u16 {
            // TODO: I think that is an exception because a normal function at least has one return value or uses ret to return a void
        }
        let ci = codes[pc as usize];

        // register status
        let ins = ci as u16;
        let e1 = (ci << 16) as u16;
        let e1x = (ci << 16) as u32;
        let e2 = (ci << 32) as u16;
        let e2x = (ci << 32) as u32;
        let e3 = (ci << 48) as u16;
        //                          u48
        let ed = (ci << 16) as u64;
        // you can ask me why i do it this way
        // no silver bullet
        match ins {
            _ if ins == OpCode::NOP as u16 => {},
            
            // ===== LOAD =====
            
            _ if ins == OpCode::LOADK as u16 => {
                if e3 == 0xffff{
                    let constant_pool_ref = unsafe{current_function_state.constant_pool_ptr.as_ref()};
                    let constant = constant_pool_ref[&e2].clone();
                    let value = Value::from_constant(constant,current_function_state.constant_pool_ptr,state);
                    stack_regs[e1 as usize] = value;
                }
                // loadk from other module
                else{
                    // let from_constant_pool = thispool.find e3(string)
                    // state.pools.find(from_constant_pool)
                    // do load
                    unimplemented!()
                }
            },
            _ if ins == OpCode::IMMBOOL as u16 => {
                stack_regs[e1 as usize] = Value::Boolean(e2 != 0x00);
            }
            _ if ins == OpCode::IMMU8 as u16 => {
                stack_regs[e1 as usize] = Value::U8(e2 as u8);
            }
            _ if ins == OpCode::IMMU16 as u16 => {
                stack_regs[e1 as usize] = Value::U16(e2);
            }
            _ if ins == OpCode::IMMU32 as u16 => {
                stack_regs[e1 as usize] = Value::U32(e2x);
            }
            _ if ins == OpCode::IMMU64 as u16 => {
                let dp1 = e2x;
                pc += 1;
                let ci = codes[pc as usize];
                let ins = ci as u16;
                //                          u48
                let ed = (ci << 16) as u64;
                if ins != OpCode::EXTRA as u16 {
                    panic!("ERROR! EXCEPT EXTRA FIND {:02X}", ins);
                } else {
                    let v = ed << 16 + dp1;
                    stack_regs[e1 as usize] = Value::U64(v);
                }
            }
            
            _ if ins == OpCode::IMMI8 as u16 => {
                stack_regs[e1 as usize] = Value::I8(e2 as i8);
            }
            _ if ins == OpCode::IMMI16 as u16 => {
                stack_regs[e1 as usize] = Value::I16(e2 as i16);
            }
            _ if ins == OpCode::IMMI32 as u16 => {
                stack_regs[e1 as usize] = Value::I32(e2x as i32);
            }
            _ if ins == OpCode::IMMI64 as u16 => {
                let dp1 = e2x;
                pc += 1;
                let ci = codes[pc as usize];
                let ins = ci as u16;
                //                          u48
                let ed = (ci << 16) as u64;
                if ins != OpCode::EXTRA as u16 {
                    panic!("ERROR! EXCEPT EXTRA FIND {:02X}", ins);
                } else {
                    let v = ed << 16 + dp1;
                    stack_regs[e1 as usize] = Value::I64(v as i64);
                }
            }
            
            _ if ins == OpCode::IMMF32 as u16 => {
                let f = {
                    let i = e2x;
                    let bt = [
                        i as u8,
                        (i >> 8) as u8,
                        (i >> 16) as u8,
                        (i >> 24) as u8,
                    ];
                    f32::from_be_bytes(bt)
                };
                stack_regs[e1 as usize] = Value::F32(f);
            }
            _ if ins == OpCode::IMMF64 as u16 => {
                let dp1 = e2x;
                pc += 1;
                let ci = codes[pc as usize];
                let ins = ci as u16;
                //                          u48
                let ed = (ci << 16) as u64;
                if ins != OpCode::EXTRA as u16 {
                    panic!("ERROR! EXCEPT EXTRA FIND {:02X}", ins);
                } else {
                    let bt = [
                        dp1 as u8,
                        (dp1 >> 8) as u8,
                        (dp1 >> 16) as u8,
                        (dp1 >> 24) as u8,
                        ed as u8,
                        (ed >> 8) as u8,
                        (ed >> 16) as u8,
                        (ed >> 24) as u8,
                    ];
                    let f = f64::from_be_bytes(bt);
                    stack_regs[e1 as usize] = Value::F64(f);
                }
            }
            
            _ if ins == OpCode::IMMSTR as u16 => {
                // TODO:
                // load len
                // load vec
                // vec u8 to string
            }

            _ if ins == OpCode::NEG as u16 => {
                match stack_regs[e1 as usize]{
                    Value::U8(v) => {stack_regs[e1 as usize] = Value::U8(-(v as i8) as u8)},
                    Value::I8(v) => {stack_regs[e1 as usize] = Value::I8(-v)},
                    Value::U16(v) => {stack_regs[e1 as usize] = Value::U16(-(v as i16) as u16)},
                    Value::I16(v) => {stack_regs[e1 as usize] = Value::I16(-v)},
                    Value::U32(v) => {stack_regs[e1 as usize] = Value::U32(-(v as i32) as u32)},
                    Value::I32(v) => {stack_regs[e1 as usize] = Value::I32(-v)},
                    Value::U64(v) => {stack_regs[e1 as usize] = Value::U64(-(v as i64) as u64)},
                    Value::I64(v) => {stack_regs[e1 as usize] = Value::I64(-v)},
                    Value::F32(v) => {stack_regs[e1 as usize] = Value::F32(-v)},
                    Value::F64(v) => {stack_regs[e1 as usize] = Value::F64(-v)},
                    _ => {}
                }
            }
            _ if ins == OpCode::SNEG as u16 => {
                match stack_regs[e1 as usize]{
                    Value::U8(v) => {panic!("ERROR! NEG A U8")},
                    Value::I8(v) => {stack_regs[e1 as usize] = Value::I8(-v)},
                    Value::U16(v) => {panic!("ERROR! NEG A U16")},
                    Value::I16(v) => {stack_regs[e1 as usize] = Value::I16(-v)},
                    Value::U32(v) => {panic!("ERROR! NEG A U32")},
                    Value::I32(v) => {stack_regs[e1 as usize] = Value::I32(-v)},
                    Value::U64(v) => {panic!("ERROR! NEG A U64")},
                    Value::I64(v) => {stack_regs[e1 as usize] = Value::I64(-v)},
                    Value::F32(v) => {stack_regs[e1 as usize] = Value::F32(-v)},
                    Value::F64(v) => {stack_regs[e1 as usize] = Value::F64(-v)},
                    _ => {panic!("ERROR! SNEG COULD NOT APPLY TO NONVALUE TYPES")}
                }
            }

            
            _ if ins == OpCode::DEC as u16 => {
                match stack_regs[e1 as usize]{
                    Value::U8(v) => {stack_regs[e1 as usize] = Value::U8(v-1)},
                    Value::I8(v) => {stack_regs[e1 as usize] = Value::I8(v-1)},
                    Value::U16(v) => {stack_regs[e1 as usize] = Value::U16(v-1)},
                    Value::I16(v) => {stack_regs[e1 as usize] = Value::I16(v-1)},
                    Value::U32(v) => {stack_regs[e1 as usize] = Value::U32(v-1)},
                    Value::I32(v) => {stack_regs[e1 as usize] = Value::I32(v-1)},
                    Value::U64(v) => {stack_regs[e1 as usize] = Value::U64(v-1)},
                    Value::I64(v) => {stack_regs[e1 as usize] = Value::I64(v-1)},
                    Value::F32(v) => {stack_regs[e1 as usize] = Value::F32(v-1.0)},
                    Value::F64(v) => {stack_regs[e1 as usize] = Value::F64(v-1.0)},
                    _ => {}
                }
            }
            _ if ins == OpCode::INC as u16 => {
                match stack_regs[e1 as usize]{
                    Value::U8(v) => {stack_regs[e1 as usize] = Value::U8(v+1)},
                    Value::I8(v) => {stack_regs[e1 as usize] = Value::I8(v+1)},
                    Value::U16(v) => {stack_regs[e1 as usize] = Value::U16(v+1)},
                    Value::I16(v) => {stack_regs[e1 as usize] = Value::I16(v+1)},
                    Value::U32(v) => {stack_regs[e1 as usize] = Value::U32(v+1)},
                    Value::I32(v) => {stack_regs[e1 as usize] = Value::I32(v+1)},
                    Value::U64(v) => {stack_regs[e1 as usize] = Value::U64(v+1)},
                    Value::I64(v) => {stack_regs[e1 as usize] = Value::I64(v+1)},
                    Value::F32(v) => {stack_regs[e1 as usize] = Value::F32(v+1.0)},
                    Value::F64(v) => {stack_regs[e1 as usize] = Value::F64(v+1.0)},
                    _ => {}
                }
            }
            _ if ins == OpCode::SDEC as u16 => {
                match stack_regs[e1 as usize]{
                    Value::U8(v) => {stack_regs[e1 as usize] = if v > 0{Value::U8(v-1)}else{panic!("ERROR! OVERFLOWING U8")}},
                    Value::I8(v) => {stack_regs[e1 as usize] = if v > std::i8::MIN{Value::I8(v-1)}else{panic!("ERROR! OVERFLOWING I8")}},
                    Value::U16(v) => {stack_regs[e1 as usize] = if v > 0{Value::U16(v-1)}else{panic!("ERROR! OVERFLOWING U16")}},
                    Value::I16(v) => {stack_regs[e1 as usize] = if v > std::i16::MIN{Value::I16(v-1)}else{panic!("ERROR! OVERFLOWING I16")}},
                    Value::U32(v) => {stack_regs[e1 as usize] = if v > 0{Value::U32(v-1)}else{panic!("ERROR! OVERFLOWING U32")}},
                    Value::I32(v) => {stack_regs[e1 as usize] = if v > std::i32::MIN{Value::I32(v-1)}else{panic!("ERROR! OVERFLOWING I32")}},
                    Value::U64(v) => {stack_regs[e1 as usize] = if v > 0{Value::U64(v-1)}else{panic!("ERROR! OVERFLOWING U64")}},
                    Value::I64(v) => {stack_regs[e1 as usize] = if v > std::i64::MIN{Value::I64(v-1)}else{panic!("ERROR! OVERFLOWING I64")}},
                    _ => {panic!("ERROR! SDEC COULD NOT APPLY TO NONVALUE TYPES")}
                }
            }
            _ if ins == OpCode::SINC as u16 => {
                match stack_regs[e1 as usize]{
                    Value::U8(v) => {stack_regs[e1 as usize] = if v > 0{Value::U8(v+1)}else{panic!("ERROR! OVERFLOWING U8")}},
                    Value::I8(v) => {stack_regs[e1 as usize] = if v > std::i8::MAX{Value::I8(v+1)}else{panic!("ERROR! OVERFLOWING I8")}},
                    Value::U16(v) => {stack_regs[e1 as usize] = if v > 0{Value::U16(v+1)}else{panic!("ERROR! OVERFLOWING U16")}},
                    Value::I16(v) => {stack_regs[e1 as usize] = if v > std::i16::MAX{Value::I16(v+1)}else{panic!("ERROR! OVERFLOWING I16")}},
                    Value::U32(v) => {stack_regs[e1 as usize] = if v > 0{Value::U32(v+1)}else{panic!("ERROR! OVERFLOWING U32")}},
                    Value::I32(v) => {stack_regs[e1 as usize] = if v > std::i32::MAX{Value::I32(v+1)}else{panic!("ERROR! OVERFLOWING I32")}},
                    Value::U64(v) => {stack_regs[e1 as usize] = if v > 0{Value::U64(v+1)}else{panic!("ERROR! OVERFLOWING U64")}},
                    Value::I64(v) => {stack_regs[e1 as usize] = if v > std::i64::MAX{Value::I64(v+1)}else{panic!("ERROR! OVERFLOWING I64")}},
                    _ => {panic!("ERROR! SINC COULD NOT APPLY TO NONVALUE TYPES")}
                }
            }
            

            _ if ins == OpCode::ADD8 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u8},
                    Value::I8(v) => {sized = true;v as u8},
                    Value::U16(v) => {v as u8},
                    Value::I16(v) => {sized = true;v as u8},
                    Value::U32(v) => {v as u8},
                    Value::I32(v) => {sized = true;v as u8},
                    Value::U64(v) => {v as u8},
                    Value::I64(v) => {sized = true;v as u8},
                    Value::F32(v) => {sized = true;v as u8},
                    Value::F64(v) => {sized = true;v as u8},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u8},
                    Value::I8(v) => {sized = true;v as u8},
                    Value::U16(v) => {v as u8},
                    Value::I16(v) => {sized = true;v as u8},
                    Value::U32(v) => {v as u8},
                    Value::I32(v) => {sized = true;v as u8},
                    Value::U64(v) => {v as u8},
                    Value::I64(v) => {sized = true;v as u8},
                    Value::F32(v) => {sized = true;v as u8},
                    Value::F64(v) => {sized = true;v as u8},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I8(v1 as i8 + v2 as i8);
                }else{
                    stack_regs[e3 as usize] = Value::U8(v1+v2);
                }
            }
            _ if ins == OpCode::ADD16 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,u16,i16,I16,U16,+);
            }
            _ if ins == OpCode::ADD32 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,u32,i32,I32,U32,+);
            }
            _ if ins == OpCode::ADD64 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u64},
                    Value::I8(v) => {sized = true;v as u64},
                    Value::U16(v) => {v as u64},
                    Value::I16(v) => {sized = true;v as u64},
                    Value::U32(v) => {v as u64},
                    Value::I32(v) => {sized = true;v as u64},
                    Value::U64(v) => {v as u64},
                    Value::I64(v) => {sized = true;v as u64},
                    Value::F32(v) => {sized = true;v as u64},
                    Value::F64(v) => {sized = true;v as u64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u64},
                    Value::I8(v) => {sized = true;v as u64},
                    Value::U16(v) => {v as u64},
                    Value::I16(v) => {sized = true;v as u64},
                    Value::U32(v) => {v as u64},
                    Value::I32(v) => {sized = true;v as u64},
                    Value::U64(v) => {v as u64},
                    Value::I64(v) => {sized = true;v as u64},
                    Value::F32(v) => {sized = true;v as u64},
                    Value::F64(v) => {sized = true;v as u64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I64(v1 as i64 + v2 as i64);
                }else{
                    stack_regs[e3 as usize] = Value::U64(v1+v2);
                }
            }
            _ if ins == OpCode::ADDF32 as u16 => {
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as f32},
                    Value::I8(v) => {v as f32},
                    Value::U16(v) => {v as f32},
                    Value::I16(v) => {v as f32},
                    Value::U32(v) => {v as f32},
                    Value::I32(v) => {v as f32},
                    Value::U64(v) => {v as f32},
                    Value::I64(v) => {v as f32},
                    Value::F32(v) => {v as f32},
                    Value::F64(v) => {v as f32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as f32},
                    Value::I8(v) => {v as f32},
                    Value::U16(v) => {v as f32},
                    Value::I16(v) => {v as f32},
                    Value::U32(v) => {v as f32},
                    Value::I32(v) => {v as f32},
                    Value::U64(v) => {v as f32},
                    Value::I64(v) => {v as f32},
                    Value::F32(v) => {v as f32},
                    Value::F64(v) => {v as f32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                stack_regs[e3 as usize] = Value::F32(v1+v2);
            }
            _ if ins == OpCode::ADDF64 as u16 => {
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as f64},
                    Value::I8(v) => {v as f64},
                    Value::U16(v) => {v as f64},
                    Value::I16(v) => {v as f64},
                    Value::U32(v) => {v as f64},
                    Value::I32(v) => {v as f64},
                    Value::U64(v) => {v as f64},
                    Value::I64(v) => {v as f64},
                    Value::F32(v) => {v as f64},
                    Value::F64(v) => {v as f64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as f64},
                    Value::I8(v) => {v as f64},
                    Value::U16(v) => {v as f64},
                    Value::I16(v) => {v as f64},
                    Value::U32(v) => {v as f64},
                    Value::I32(v) => {v as f64},
                    Value::U64(v) => {v as f64},
                    Value::I64(v) => {v as f64},
                    Value::F32(v) => {v as f64},
                    Value::F64(v) => {v as f64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                stack_regs[e3 as usize] = Value::F64(v1+v2);
            }
            
            _ if ins == OpCode::SUB8 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u8},
                    Value::I8(v) => {sized = true;v as u8},
                    Value::U16(v) => {v as u8},
                    Value::I16(v) => {sized = true;v as u8},
                    Value::U32(v) => {v as u8},
                    Value::I32(v) => {sized = true;v as u8},
                    Value::U64(v) => {v as u8},
                    Value::I64(v) => {sized = true;v as u8},
                    Value::F32(v) => {sized = true;v as u8},
                    Value::F64(v) => {sized = true;v as u8},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u8},
                    Value::I8(v) => {sized = true;v as u8},
                    Value::U16(v) => {v as u8},
                    Value::I16(v) => {sized = true;v as u8},
                    Value::U32(v) => {v as u8},
                    Value::I32(v) => {sized = true;v as u8},
                    Value::U64(v) => {v as u8},
                    Value::I64(v) => {sized = true;v as u8},
                    Value::F32(v) => {sized = true;v as u8},
                    Value::F64(v) => {sized = true;v as u8},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I8(v1 as i8 - v2 as i8);
                }else{
                    stack_regs[e3 as usize] = Value::U8(v1-v2);
                }
            }
            _ if ins == OpCode::SUB16 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u16},
                    Value::I8(v) => {sized = true;v as u16},
                    Value::U16(v) => {v as u16},
                    Value::I16(v) => {sized = true;v as u16},
                    Value::U32(v) => {v as u16},
                    Value::I32(v) => {sized = true;v as u16},
                    Value::U64(v) => {v as u16},
                    Value::I64(v) => {sized = true;v as u16},
                    Value::F32(v) => {sized = true;v as u16},
                    Value::F64(v) => {sized = true;v as u16},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u16},
                    Value::I8(v) => {sized = true;v as u16},
                    Value::U16(v) => {v as u16},
                    Value::I16(v) => {sized = true;v as u16},
                    Value::U32(v) => {v as u16},
                    Value::I32(v) => {sized = true;v as u16},
                    Value::U64(v) => {v as u16},
                    Value::I64(v) => {sized = true;v as u16},
                    Value::F32(v) => {sized = true;v as u16},
                    Value::F64(v) => {sized = true;v as u16},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I16(v1 as i16 - v2 as i16);
                }else{
                    stack_regs[e3 as usize] = Value::U16(v1-v2);
                }
            }
            _ if ins == OpCode::SUB32 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u32},
                    Value::I8(v) => {sized = true;v as u32},
                    Value::U16(v) => {v as u32},
                    Value::I16(v) => {sized = true;v as u32},
                    Value::U32(v) => {v as u32},
                    Value::I32(v) => {sized = true;v as u32},
                    Value::U64(v) => {v as u32},
                    Value::I64(v) => {sized = true;v as u32},
                    Value::F32(v) => {sized = true;v as u32},
                    Value::F64(v) => {sized = true;v as u32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u32},
                    Value::I8(v) => {sized = true;v as u32},
                    Value::U16(v) => {v as u32},
                    Value::I16(v) => {sized = true;v as u32},
                    Value::U32(v) => {v as u32},
                    Value::I32(v) => {sized = true;v as u32},
                    Value::U64(v) => {v as u32},
                    Value::I64(v) => {sized = true;v as u32},
                    Value::F32(v) => {sized = true;v as u32},
                    Value::F64(v) => {sized = true;v as u32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I32(v1 as i32 - v2 as i32);
                }else{
                    stack_regs[e3 as usize] = Value::U32(v1-v2);
                }
            }
            _ if ins == OpCode::SUB64 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u64},
                    Value::I8(v) => {sized = true;v as u64},
                    Value::U16(v) => {v as u64},
                    Value::I16(v) => {sized = true;v as u64},
                    Value::U32(v) => {v as u64},
                    Value::I32(v) => {sized = true;v as u64},
                    Value::U64(v) => {v as u64},
                    Value::I64(v) => {sized = true;v as u64},
                    Value::F32(v) => {sized = true;v as u64},
                    Value::F64(v) => {sized = true;v as u64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u64},
                    Value::I8(v) => {sized = true;v as u64},
                    Value::U16(v) => {v as u64},
                    Value::I16(v) => {sized = true;v as u64},
                    Value::U32(v) => {v as u64},
                    Value::I32(v) => {sized = true;v as u64},
                    Value::U64(v) => {v as u64},
                    Value::I64(v) => {sized = true;v as u64},
                    Value::F32(v) => {sized = true;v as u64},
                    Value::F64(v) => {sized = true;v as u64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I64(v1 as i64 - v2 as i64);
                }else{
                    stack_regs[e3 as usize] = Value::U64(v1-v2);
                }
            }
            _ if ins == OpCode::SUBF32 as u16 => {
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as f32},
                    Value::I8(v) => {v as f32},
                    Value::U16(v) => {v as f32},
                    Value::I16(v) => {v as f32},
                    Value::U32(v) => {v as f32},
                    Value::I32(v) => {v as f32},
                    Value::U64(v) => {v as f32},
                    Value::I64(v) => {v as f32},
                    Value::F32(v) => {v as f32},
                    Value::F64(v) => {v as f32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as f32},
                    Value::I8(v) => {v as f32},
                    Value::U16(v) => {v as f32},
                    Value::I16(v) => {v as f32},
                    Value::U32(v) => {v as f32},
                    Value::I32(v) => {v as f32},
                    Value::U64(v) => {v as f32},
                    Value::I64(v) => {v as f32},
                    Value::F32(v) => {v as f32},
                    Value::F64(v) => {v as f32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                stack_regs[e3 as usize] = Value::F32(v1-v2);
            }
            _ if ins == OpCode::SUBF64 as u16 => {
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as f64},
                    Value::I8(v) => {v as f64},
                    Value::U16(v) => {v as f64},
                    Value::I16(v) => {v as f64},
                    Value::U32(v) => {v as f64},
                    Value::I32(v) => {v as f64},
                    Value::U64(v) => {v as f64},
                    Value::I64(v) => {v as f64},
                    Value::F32(v) => {v as f64},
                    Value::F64(v) => {v as f64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as f64},
                    Value::I8(v) => {v as f64},
                    Value::U16(v) => {v as f64},
                    Value::I16(v) => {v as f64},
                    Value::U32(v) => {v as f64},
                    Value::I32(v) => {v as f64},
                    Value::U64(v) => {v as f64},
                    Value::I64(v) => {v as f64},
                    Value::F32(v) => {v as f64},
                    Value::F64(v) => {v as f64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                stack_regs[e3 as usize] = Value::F64(v1-v2);
            }
            
            _ if ins == OpCode::MUL8 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u8},
                    Value::I8(v) => {sized = true;v as u8},
                    Value::U16(v) => {v as u8},
                    Value::I16(v) => {sized = true;v as u8},
                    Value::U32(v) => {v as u8},
                    Value::I32(v) => {sized = true;v as u8},
                    Value::U64(v) => {v as u8},
                    Value::I64(v) => {sized = true;v as u8},
                    Value::F32(v) => {sized = true;v as u8},
                    Value::F64(v) => {sized = true;v as u8},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u8},
                    Value::I8(v) => {sized = true;v as u8},
                    Value::U16(v) => {v as u8},
                    Value::I16(v) => {sized = true;v as u8},
                    Value::U32(v) => {v as u8},
                    Value::I32(v) => {sized = true;v as u8},
                    Value::U64(v) => {v as u8},
                    Value::I64(v) => {sized = true;v as u8},
                    Value::F32(v) => {sized = true;v as u8},
                    Value::F64(v) => {sized = true;v as u8},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I8(v1 as i8 + v2 as i8);
                }else{
                    stack_regs[e3 as usize] = Value::U8(v1+v2);
                }
            }
            _ if ins == OpCode::MUL16 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u16},
                    Value::I8(v) => {sized = true;v as u16},
                    Value::U16(v) => {v as u16},
                    Value::I16(v) => {sized = true;v as u16},
                    Value::U32(v) => {v as u16},
                    Value::I32(v) => {sized = true;v as u16},
                    Value::U64(v) => {v as u16},
                    Value::I64(v) => {sized = true;v as u16},
                    Value::F32(v) => {sized = true;v as u16},
                    Value::F64(v) => {sized = true;v as u16},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u16},
                    Value::I8(v) => {sized = true;v as u16},
                    Value::U16(v) => {v as u16},
                    Value::I16(v) => {sized = true;v as u16},
                    Value::U32(v) => {v as u16},
                    Value::I32(v) => {sized = true;v as u16},
                    Value::U64(v) => {v as u16},
                    Value::I64(v) => {sized = true;v as u16},
                    Value::F32(v) => {sized = true;v as u16},
                    Value::F64(v) => {sized = true;v as u16},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I16(v1 as i16 + v2 as i16);
                }else{
                    stack_regs[e3 as usize] = Value::U16(v1+v2);
                }
            }
            _ if ins == OpCode::MUL32 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u32},
                    Value::I8(v) => {sized = true;v as u32},
                    Value::U16(v) => {v as u32},
                    Value::I16(v) => {sized = true;v as u32},
                    Value::U32(v) => {v as u32},
                    Value::I32(v) => {sized = true;v as u32},
                    Value::U64(v) => {v as u32},
                    Value::I64(v) => {sized = true;v as u32},
                    Value::F32(v) => {sized = true;v as u32},
                    Value::F64(v) => {sized = true;v as u32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u32},
                    Value::I8(v) => {sized = true;v as u32},
                    Value::U16(v) => {v as u32},
                    Value::I16(v) => {sized = true;v as u32},
                    Value::U32(v) => {v as u32},
                    Value::I32(v) => {sized = true;v as u32},
                    Value::U64(v) => {v as u32},
                    Value::I64(v) => {sized = true;v as u32},
                    Value::F32(v) => {sized = true;v as u32},
                    Value::F64(v) => {sized = true;v as u32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I32(v1 as i32 + v2 as i32);
                }else{
                    stack_regs[e3 as usize] = Value::U32(v1+v2);
                }
            }
            _ if ins == OpCode::MUL64 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as u64},
                    Value::I8(v) => {sized = true;v as u64},
                    Value::U16(v) => {v as u64},
                    Value::I16(v) => {sized = true;v as u64},
                    Value::U32(v) => {v as u64},
                    Value::I32(v) => {sized = true;v as u64},
                    Value::U64(v) => {v as u64},
                    Value::I64(v) => {sized = true;v as u64},
                    Value::F32(v) => {sized = true;v as u64},
                    Value::F64(v) => {sized = true;v as u64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as u64},
                    Value::I8(v) => {sized = true;v as u64},
                    Value::U16(v) => {v as u64},
                    Value::I16(v) => {sized = true;v as u64},
                    Value::U32(v) => {v as u64},
                    Value::I32(v) => {sized = true;v as u64},
                    Value::U64(v) => {v as u64},
                    Value::I64(v) => {sized = true;v as u64},
                    Value::F32(v) => {sized = true;v as u64},
                    Value::F64(v) => {sized = true;v as u64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                if sized{
                    stack_regs[e3 as usize] = Value::I64(v1 as i64 + v2 as i64);
                }else{
                    stack_regs[e3 as usize] = Value::U64(v1+v2);
                }
            }
            _ if ins == OpCode::MULF32 as u16 => {
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as f32},
                    Value::I8(v) => {v as f32},
                    Value::U16(v) => {v as f32},
                    Value::I16(v) => {v as f32},
                    Value::U32(v) => {v as f32},
                    Value::I32(v) => {v as f32},
                    Value::U64(v) => {v as f32},
                    Value::I64(v) => {v as f32},
                    Value::F32(v) => {v as f32},
                    Value::F64(v) => {v as f32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as f32},
                    Value::I8(v) => {v as f32},
                    Value::U16(v) => {v as f32},
                    Value::I16(v) => {v as f32},
                    Value::U32(v) => {v as f32},
                    Value::I32(v) => {v as f32},
                    Value::U64(v) => {v as f32},
                    Value::I64(v) => {v as f32},
                    Value::F32(v) => {v as f32},
                    Value::F64(v) => {v as f32},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                stack_regs[e3 as usize] = Value::F32(v1+v2);
            }
            _ if ins == OpCode::MULF64 as u16 => {
                let v1 = match stack_regs[e1 as usize]{
                    Value::U8(v) => {v as f64},
                    Value::I8(v) => {v as f64},
                    Value::U16(v) => {v as f64},
                    Value::I16(v) => {v as f64},
                    Value::U32(v) => {v as f64},
                    Value::I32(v) => {v as f64},
                    Value::U64(v) => {v as f64},
                    Value::I64(v) => {v as f64},
                    Value::F32(v) => {v as f64},
                    Value::F64(v) => {v as f64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                let v2 = match stack_regs[e2 as usize]{
                    Value::U8(v) => {v as f64},
                    Value::I8(v) => {v as f64},
                    Value::U16(v) => {v as f64},
                    Value::I16(v) => {v as f64},
                    Value::U32(v) => {v as f64},
                    Value::I32(v) => {v as f64},
                    Value::U64(v) => {v as f64},
                    Value::I64(v) => {v as f64},
                    Value::F32(v) => {v as f64},
                    Value::F64(v) => {v as f64},
                    _ => {panic!("ERROR! ADD8 COULD ONLY APPLY TO NUMERICAL VALUES")}
                };
                stack_regs[e3 as usize] = Value::F64(v1+v2);
            }
            


            _ => unimplemented!(),
        }

        pc += 1;
    }

    // restore status
    state.current_function_call_state = current_function_state;
}
