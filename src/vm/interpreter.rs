use super::register;
use crate::config::*;

use super::{VMState, value::Value};
use crate::{
    binary::{function::Function, opcode::OpCode, constant::Constant},
    config::*,
};
use std::{ptr::NonNull, collections::BTreeMap};

fn interpreter(state: VMState) {
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
                let constant_pool_ref = unsafe{current_function_state.constant_pool_ptr.as_ref()};
                let constant = constant_pool_ref[&e2].clone();

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


            _ if ins == OpCode::CLOSURE as u16 => {

            }

            _ => unimplemented!(),
        }

        pc += 1;
    }

    // restore status
    state.current_function_call_state = current_function_state;
}
