use super::register;
use crate::config::*;

use super::{value::{NSValue, Value}, VMClosureStatus, VMState};
use crate::{
    binary::{constant::Constant, function::Function, opcode::OpCode},
    config::*,
};
use std::panic;
use std::{collections::BTreeMap, ptr::NonNull};

macro_rules! expr {
    ($e:expr) => {
        $e
    };
}
macro_rules! TRI_INS_X {
    ($regs:ident,$e1:ident,$e2:ident,$e3:ident,$t:ident,$t2:ident,$vc1:ident,$op:tt) => {
        let v1 = match $regs[$e1 as usize]{
            Value::U8(v) => {v as $t},
            Value::I8(v) => {v as $t},
            Value::U16(v) => {v as $t},
            Value::I16(v) => {v as $t},
            Value::U32(v) => {v as $t},
            Value::I32(v) => {v as $t},
            Value::U64(v) => {v as $t},
            Value::I64(v) => {v as $t},
            Value::F32(v) => {v as $t},
            Value::F64(v) => {v as $t},
            _ => {panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES")}
        };
        let v2 = match $regs[$e2 as usize]{
            Value::U8(v) => {v as $t},
            Value::I8(v) => {v as $t},
            Value::U16(v) => {v as $t},
            Value::I16(v) => {v as $t},
            Value::U32(v) => {v as $t},
            Value::I32(v) => {v as $t},
            Value::U64(v) => {v as $t},
            Value::I64(v) => {v as $t},
            Value::F32(v) => {v as $t},
            Value::F64(v) => {v as $t},
            _ => {panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES")}
        };
        unsafe {
            $regs[$e3 as usize] = Value::$vc1(
                expr!(v1 $op v2)
            );
        }
    };
    ($regs:ident,$e1:ident,$e2:ident,$e3:ident,$t:ident,$t2:ident,$vc1:ident,$vc2:ident,$op:ident) => {
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
            _ => {panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES")}
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
            _ => {panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES")}
        };
        unsafe {
            if sized{
                $regs[$e3 as usize] = Value::$vc1(
                    (v1 as $t2).$op(v2 as $t2)
                );
            }else{
                $regs[$e3 as usize] = Value::$vc2(
                    (v1).$op(v2)
                );
            }
        }
    };
}

#[inline(always)]
fn value_to_bool(value: &Value) -> Option<bool> {
    match value {
        Value::U8(v) => Some(*v != 0),
        Value::I8(v) => Some(*v != 0),
        Value::U16(v) => Some(*v != 0),
        Value::I16(v) => Some(*v != 0),
        Value::U32(v) => Some(*v != 0),
        Value::I32(v) => Some(*v != 0),
        Value::U64(v) => Some(*v != 0),
        Value::I64(v) => Some(*v != 0),
        Value::F32(v) => Some(*v != 0.0),
        Value::F64(v) => Some(*v != 0.0),
        Value::Boolean(v) => Some(*v),
        _ => None,
    }
}
pub fn interpreter(state: &mut VMState) {
    // init
    let mut current_function_state = state.current_function_call_state.clone();
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
        if current_function_state.status == VMClosureStatus::Error {
            let mut find_exception_handle = false;
            // reset to error pc
            pc -= 1;
            if let Some(etable) = &current_function_state.function_bytecode.exception_table {
                let handles = etable
                    .table
                    .iter()
                    .filter(|x| x.start_pc <= pc && x.end_pc > pc)
                    .next();
                if let Some(handle) = handles {
                    find_exception_handle = true;
                    let handler_pc = handle.handler_pc;
                    pc = handler_pc;
                }
            }
            if !find_exception_handle {
                // return to super function stack
                // set exception
                // save template stack variable to current function state
                state.current_function_call_state.registers = stack_regs.to_vec();
                state.current_function_call_state.pc = pc;
                state.exception_stack.push(state.current_function_call_state.clone());
                if let Some(cls) = state.function_call_chain_states.pop() {
                    state.current_function_call_state = cls;
                    state.current_function_call_state.pc += 1;
                    state.current_function_call_state.status = VMClosureStatus::Error;
                    return interpreter(state);
                }
                // the outer
                else {
                    panic!("VM TERMINATED WITH CALL STACK {:?}", state.exception_stack);
                }
            }
        }

        // on breakpoint
        // on whatever

        // on end function scope
        if pc >= codes.len() as u16 {
            return;
            // TODO: I think that is an exception because a normal function at least has one return value or uses ret to return a void
        }
        let ci = codes[pc as usize];

        // register status
        let ins = ci as u16;
        let e1 = (ci >> 16) as u16;
        let e1x = (ci >> 16) as u32;
        let e2 = (ci >> 32) as u16;
        let e2x = (ci >> 32) as u32;
        let e3 = (ci >> 48) as u16;
        //                          u48
        let ed = (ci >> 16) as u64;
        // you can ask me why i do it this way
        // no silver bullet
        match ins {
            _ if ins == OpCode::NOP as u16 => {}

            // ===== LOAD =====
            _ if ins == OpCode::LOADK as u16 => {
                if e3 == 0xFFFF {
                    let constant_pool_ref =
                        unsafe { current_function_state.constant_pool_ptr.as_ref() };
                    let constant = constant_pool_ref[&e2].clone();
                    let value = Value::from_constant(
                        constant,
                        current_function_state.constant_pool_ptr,
                        state,
                    );
                    stack_regs[e1 as usize] = value;
                }
                // loadk from other module
                else {
                    // let from_constant_pool = thispool.find e3(string)
                    // state.pools.find(from_constant_pool)
                    // do load
                    unimplemented!()
                }
            }
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
                    let bt = [i as u8, (i >> 8) as u8, (i >> 16) as u8, (i >> 24) as u8];
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

            _ if ins == OpCode::NEG as u16 => match stack_regs[e1 as usize] {
                Value::U8(v) => stack_regs[e1 as usize] = Value::U8(-(v as i8) as u8),
                Value::I8(v) => stack_regs[e1 as usize] = Value::I8(-v),
                Value::U16(v) => stack_regs[e1 as usize] = Value::U16(-(v as i16) as u16),
                Value::I16(v) => stack_regs[e1 as usize] = Value::I16(-v),
                Value::U32(v) => stack_regs[e1 as usize] = Value::U32(-(v as i32) as u32),
                Value::I32(v) => stack_regs[e1 as usize] = Value::I32(-v),
                Value::U64(v) => stack_regs[e1 as usize] = Value::U64(-(v as i64) as u64),
                Value::I64(v) => stack_regs[e1 as usize] = Value::I64(-v),
                Value::F32(v) => stack_regs[e1 as usize] = Value::F32(-v),
                Value::F64(v) => stack_regs[e1 as usize] = Value::F64(-v),
                _ => {}
            },
            _ if ins == OpCode::SNEG as u16 => match stack_regs[e1 as usize] {
                Value::U8(v) => panic!("ERROR! NEG A U8"),
                Value::I8(v) => stack_regs[e1 as usize] = Value::I8(-v),
                Value::U16(v) => panic!("ERROR! NEG A U16"),
                Value::I16(v) => stack_regs[e1 as usize] = Value::I16(-v),
                Value::U32(v) => panic!("ERROR! NEG A U32"),
                Value::I32(v) => stack_regs[e1 as usize] = Value::I32(-v),
                Value::U64(v) => panic!("ERROR! NEG A U64"),
                Value::I64(v) => stack_regs[e1 as usize] = Value::I64(-v),
                Value::F32(v) => stack_regs[e1 as usize] = Value::F32(-v),
                Value::F64(v) => stack_regs[e1 as usize] = Value::F64(-v),
                _ => panic!("ERROR! SNEG COULD NOT APPLY TO NONVALUE TYPES"),
            },

            _ if ins == OpCode::DEC as u16 => match stack_regs[e1 as usize] {
                Value::U8(v) => stack_regs[e1 as usize] = Value::U8(v - 1),
                Value::I8(v) => stack_regs[e1 as usize] = Value::I8(v - 1),
                Value::U16(v) => stack_regs[e1 as usize] = Value::U16(v - 1),
                Value::I16(v) => stack_regs[e1 as usize] = Value::I16(v - 1),
                Value::U32(v) => stack_regs[e1 as usize] = Value::U32(v - 1),
                Value::I32(v) => stack_regs[e1 as usize] = Value::I32(v - 1),
                Value::U64(v) => stack_regs[e1 as usize] = Value::U64(v - 1),
                Value::I64(v) => stack_regs[e1 as usize] = Value::I64(v - 1),
                Value::F32(v) => stack_regs[e1 as usize] = Value::F32(v - 1.0),
                Value::F64(v) => stack_regs[e1 as usize] = Value::F64(v - 1.0),
                _ => {}
            },
            _ if ins == OpCode::INC as u16 => match stack_regs[e1 as usize] {
                Value::U8(v) => stack_regs[e1 as usize] = Value::U8(v + 1),
                Value::I8(v) => stack_regs[e1 as usize] = Value::I8(v + 1),
                Value::U16(v) => stack_regs[e1 as usize] = Value::U16(v + 1),
                Value::I16(v) => stack_regs[e1 as usize] = Value::I16(v + 1),
                Value::U32(v) => stack_regs[e1 as usize] = Value::U32(v + 1),
                Value::I32(v) => stack_regs[e1 as usize] = Value::I32(v + 1),
                Value::U64(v) => stack_regs[e1 as usize] = Value::U64(v + 1),
                Value::I64(v) => stack_regs[e1 as usize] = Value::I64(v + 1),
                Value::F32(v) => stack_regs[e1 as usize] = Value::F32(v + 1.0),
                Value::F64(v) => stack_regs[e1 as usize] = Value::F64(v + 1.0),
                _ => {}
            },
            _ if ins == OpCode::SDEC as u16 => match stack_regs[e1 as usize] {
                Value::U8(v) => {
                    stack_regs[e1 as usize] = if v > 0 {
                        Value::U8(v - 1)
                    } else {
                        panic!("ERROR! OVERFLOWING U8")
                    }
                }
                Value::I8(v) => {
                    stack_regs[e1 as usize] = if v > std::i8::MIN {
                        Value::I8(v - 1)
                    } else {
                        panic!("ERROR! OVERFLOWING I8")
                    }
                }
                Value::U16(v) => {
                    stack_regs[e1 as usize] = if v > 0 {
                        Value::U16(v - 1)
                    } else {
                        panic!("ERROR! OVERFLOWING U16")
                    }
                }
                Value::I16(v) => {
                    stack_regs[e1 as usize] = if v > std::i16::MIN {
                        Value::I16(v - 1)
                    } else {
                        panic!("ERROR! OVERFLOWING I16")
                    }
                }
                Value::U32(v) => {
                    stack_regs[e1 as usize] = if v > 0 {
                        Value::U32(v - 1)
                    } else {
                        panic!("ERROR! OVERFLOWING U32")
                    }
                }
                Value::I32(v) => {
                    stack_regs[e1 as usize] = if v > std::i32::MIN {
                        Value::I32(v - 1)
                    } else {
                        panic!("ERROR! OVERFLOWING I32")
                    }
                }
                Value::U64(v) => {
                    stack_regs[e1 as usize] = if v > 0 {
                        Value::U64(v - 1)
                    } else {
                        panic!("ERROR! OVERFLOWING U64")
                    }
                }
                Value::I64(v) => {
                    stack_regs[e1 as usize] = if v > std::i64::MIN {
                        Value::I64(v - 1)
                    } else {
                        panic!("ERROR! OVERFLOWING I64")
                    }
                }
                _ => panic!("ERROR! SDEC COULD NOT APPLY TO NONVALUE TYPES"),
            },
            _ if ins == OpCode::SINC as u16 => match stack_regs[e1 as usize] {
                Value::U8(v) => {
                    stack_regs[e1 as usize] = if v > 0 {
                        Value::U8(v + 1)
                    } else {
                        panic!("ERROR! OVERFLOWING U8")
                    }
                }
                Value::I8(v) => {
                    stack_regs[e1 as usize] = if v > std::i8::MAX {
                        Value::I8(v + 1)
                    } else {
                        panic!("ERROR! OVERFLOWING I8")
                    }
                }
                Value::U16(v) => {
                    stack_regs[e1 as usize] = if v > 0 {
                        Value::U16(v + 1)
                    } else {
                        panic!("ERROR! OVERFLOWING U16")
                    }
                }
                Value::I16(v) => {
                    stack_regs[e1 as usize] = if v > std::i16::MAX {
                        Value::I16(v + 1)
                    } else {
                        panic!("ERROR! OVERFLOWING I16")
                    }
                }
                Value::U32(v) => {
                    stack_regs[e1 as usize] = if v > 0 {
                        Value::U32(v + 1)
                    } else {
                        panic!("ERROR! OVERFLOWING U32")
                    }
                }
                Value::I32(v) => {
                    stack_regs[e1 as usize] = if v > std::i32::MAX {
                        Value::I32(v + 1)
                    } else {
                        panic!("ERROR! OVERFLOWING I32")
                    }
                }
                Value::U64(v) => {
                    stack_regs[e1 as usize] = if v > 0 {
                        Value::U64(v + 1)
                    } else {
                        panic!("ERROR! OVERFLOWING U64")
                    }
                }
                Value::I64(v) => {
                    stack_regs[e1 as usize] = if v > std::i64::MAX {
                        Value::I64(v + 1)
                    } else {
                        panic!("ERROR! OVERFLOWING I64")
                    }
                }
                _ => panic!("ERROR! SINC COULD NOT APPLY TO NONVALUE TYPES"),
            },

            _ if ins == OpCode::ADD8 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u8, i8, I8, U8, wrapping_add);
            }
            _ if ins == OpCode::ADD16 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u16, i16, I16, U16, wrapping_add);
            }
            _ if ins == OpCode::ADD32 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u32, i32, I32, U32, wrapping_add);
            }
            _ if ins == OpCode::ADD64 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u64, i64, I64, U64, wrapping_add);
            }
            _ if ins == OpCode::ADDF32 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,+);
            }
            _ if ins == OpCode::ADDF64 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,+);
            }

            _ if ins == OpCode::SUB8 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u8, i8, I8, U8, wrapping_sub);
            }
            _ if ins == OpCode::SUB16 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u16, i16, I16, U16, wrapping_sub);
            }
            _ if ins == OpCode::SUB32 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u32, i32, I32, U32, wrapping_sub);
            }
            _ if ins == OpCode::SUB64 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u64, i64, I64, U64, wrapping_sub);
            }
            _ if ins == OpCode::SUBF32 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,-);
            }
            _ if ins == OpCode::SUBF64 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,-);
            }

            _ if ins == OpCode::MUL8 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u8, i8, I8, U8, wrapping_mul);
            }
            _ if ins == OpCode::MUL16 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u16, i16, I16, U16, wrapping_mul);
            }
            _ if ins == OpCode::MUL32 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u32, i32, I32, U32, wrapping_mul);
            }
            _ if ins == OpCode::MUL64 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u64, i64, I64, U64, wrapping_mul);
            }
            _ if ins == OpCode::MULF32 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,*);
            }
            _ if ins == OpCode::MULF64 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,*);
            }

            _ if ins == OpCode::DIV8 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u8, i8, I8, U8, wrapping_div);
            }
            _ if ins == OpCode::DIV16 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u16, i16, I16, U16, wrapping_div);
            }
            _ if ins == OpCode::DIV32 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u32, i32, I32, U32, wrapping_div);
            }
            _ if ins == OpCode::DIV64 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u64, i64, I64, U64, wrapping_div);
            }
            _ if ins == OpCode::DIVF32 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,/);
            }
            _ if ins == OpCode::DIVF64 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,/);
            }

            _ if ins == OpCode::REM8 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u8, i8, I8, U8, wrapping_rem);
            }
            _ if ins == OpCode::REM16 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u16, i16, I16, U16, wrapping_rem);
            }
            _ if ins == OpCode::REM32 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u32, i32, I32, U32, wrapping_rem);
            }
            _ if ins == OpCode::REM64 as u16 => {
                TRI_INS_X!(stack_regs, e1, e2, e3, u64, i64, I64, U64, wrapping_rem);
            }
            _ if ins == OpCode::REMF32 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,%);
            }
            _ if ins == OpCode::REMF64 as u16 => {
                TRI_INS_X!(stack_regs,e1,e2,e3,f32,f32,F32,%);
            }

            _ if ins == OpCode::SADD8 as u16 => {
                let mut sized = false;
                let v1 = match stack_regs[e1 as usize] {
                    Value::U8(v) => v as u8,
                    Value::I8(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::U16(v) => v as u8,
                    Value::I16(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::U32(v) => v as u8,
                    Value::I32(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::U64(v) => v as u8,
                    Value::I64(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::F32(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::F64(v) => {
                        sized = true;
                        v as u8
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                let v2 = match stack_regs[e2 as usize] {
                    Value::U8(v) => v as u8,
                    Value::I8(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::U16(v) => v as u8,
                    Value::I16(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::U32(v) => v as u8,
                    Value::I32(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::U64(v) => v as u8,
                    Value::I64(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::F32(v) => {
                        sized = true;
                        v as u8
                    }
                    Value::F64(v) => {
                        sized = true;
                        v as u8
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                if sized {
                    match (v1 as i8).checked_add(v2 as i8) {
                        Some(v) => {
                            stack_regs[e3 as usize] = Value::I8(v);
                        }
                        None => panic!("ERROR! ARITH OVERFLOW"),
                    }
                } else {
                    match v1.checked_add(v2) {
                        Some(v) => {
                            stack_regs[e3 as usize] = Value::U8(v);
                        }
                        None => panic!("ERROR! ARITH OVERFLOW"),
                    }
                }
            }

            _ if ins == OpCode::BNOT as u16 => match stack_regs[e1 as usize] {
                Value::U8(v) => stack_regs[e2 as usize] = Value::U8(!v),
                Value::I8(v) => stack_regs[e2 as usize] = Value::I8(!v),
                Value::U16(v) => stack_regs[e2 as usize] = Value::U16(!v),
                Value::I16(v) => stack_regs[e2 as usize] = Value::I16(!v),
                Value::U32(v) => stack_regs[e2 as usize] = Value::U32(!v),
                Value::I32(v) => stack_regs[e2 as usize] = Value::I32(!v),
                Value::U64(v) => stack_regs[e2 as usize] = Value::U64(!v),
                Value::I64(v) => stack_regs[e2 as usize] = Value::I64(!v),
                Value::F32(v) => stack_regs[e2 as usize] = Value::I32(!(v as i32)),
                Value::F64(v) => stack_regs[e2 as usize] = Value::I64(!(v as i64)),
                _ => {}
            },
            _ if ins == OpCode::BAND as u16 => {
                let mut sized = false;
                let mut max: u8 = 0b0000;
                let v1 = match stack_regs[e1 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                let v2 = match stack_regs[e2 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                if max & 0b1000 == 0b1000 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I64(v1 as i64 & v2 as i64);
                    } else {
                        stack_regs[e3 as usize] = Value::U64(v1 as u64 & v2 as u64);
                    }
                } else if max & 0b0100 == 0b0100 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I32(v1 as i32 & v2 as i32);
                    } else {
                        stack_regs[e3 as usize] = Value::U32(v1 as u32 & v2 as u32);
                    }
                } else if max & 0b0010 == 0b0010 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I16(v1 as i16 & v2 as i16);
                    } else {
                        stack_regs[e3 as usize] = Value::U16(v1 as u16 & v2 as u16);
                    }
                } else if max & 0b0001 == 0b0001 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I8(v1 as i8 & v2 as i8);
                    } else {
                        stack_regs[e3 as usize] = Value::U8(v1 as u8 & v2 as u8);
                    }
                } else {
                    panic!("卧槽? 咋了?!");
                }
            }
            _ if ins == OpCode::BXOR as u16 => {
                let mut sized = false;
                let mut max: u8 = 0b0000;
                let v1 = match stack_regs[e1 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                let v2 = match stack_regs[e2 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                if max & 0b1000 == 0b1000 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I64(v1 as i64 ^ v2 as i64);
                    } else {
                        stack_regs[e3 as usize] = Value::U64(v1 as u64 ^ v2 as u64);
                    }
                } else if max & 0b0100 == 0b0100 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I32(v1 as i32 ^ v2 as i32);
                    } else {
                        stack_regs[e3 as usize] = Value::U32(v1 as u32 ^ v2 as u32);
                    }
                } else if max & 0b0010 == 0b0010 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I16(v1 as i16 ^ v2 as i16);
                    } else {
                        stack_regs[e3 as usize] = Value::U16(v1 as u16 ^ v2 as u16);
                    }
                } else if max & 0b0001 == 0b0001 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I8(v1 as i8 ^ v2 as i8);
                    } else {
                        stack_regs[e3 as usize] = Value::U8(v1 as u8 ^ v2 as u8);
                    }
                } else {
                    panic!("卧槽? 咋了?!");
                }
            }
            _ if ins == OpCode::BOR as u16 => {
                let mut sized = false;
                let mut max: u8 = 0b0000;
                let v1 = match stack_regs[e1 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                let v2 = match stack_regs[e2 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                if max & 0b1000 == 0b1000 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I64(v1 as i64 | v2 as i64);
                    } else {
                        stack_regs[e3 as usize] = Value::U64(v1 as u64 | v2 as u64);
                    }
                } else if max & 0b0100 == 0b0100 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I32(v1 as i32 | v2 as i32);
                    } else {
                        stack_regs[e3 as usize] = Value::U32(v1 as u32 | v2 as u32);
                    }
                } else if max & 0b0010 == 0b0010 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I16(v1 as i16 | v2 as i16);
                    } else {
                        stack_regs[e3 as usize] = Value::U16(v1 as u16 | v2 as u16);
                    }
                } else if max & 0b0001 == 0b0001 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I8(v1 as i8 | v2 as i8);
                    } else {
                        stack_regs[e3 as usize] = Value::U8(v1 as u8 | v2 as u8);
                    }
                } else {
                    panic!("卧槽? 咋了?!");
                }
            }

            _ if ins == OpCode::SHL as u16 => {
                let mut sized = false;
                let mut max: u8 = 0b0000;
                let v1 = match stack_regs[e1 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                let v2 = match stack_regs[e2 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                if max & 0b1000 == 0b1000 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I64((v1 as i64) << v2 as i64);
                    } else {
                        stack_regs[e3 as usize] = Value::U64((v1 as u64) << v2 as u64);
                    }
                } else if max & 0b0100 == 0b0100 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I32((v1 as i32) << v2 as i32);
                    } else {
                        stack_regs[e3 as usize] = Value::U32((v1 as u32) << v2 as u32);
                    }
                } else if max & 0b0010 == 0b0010 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I16((v1 as i16) << v2 as i16);
                    } else {
                        stack_regs[e3 as usize] = Value::U16((v1 as u16) << v2 as u16);
                    }
                } else if max & 0b0001 == 0b0001 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I8((v1 as i8) << v2 as i8);
                    } else {
                        stack_regs[e3 as usize] = Value::U8((v1 as u8) << v2 as u8);
                    }
                } else {
                    panic!("卧槽? 咋了?!");
                }
            }
            _ if ins == OpCode::SHR as u16 => {
                let mut sized = false;
                let mut max: u8 = 0b0000;
                let v1 = match stack_regs[e1 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                let v2 = match stack_regs[e2 as usize] {
                    Value::U8(v) => {
                        max |= 0b0001;
                        v as u64
                    }
                    Value::I8(v) => {
                        max |= 0b0001;
                        sized = true;
                        v as u64
                    }
                    Value::U16(v) => {
                        max |= 0b0010;
                        v as u64
                    }
                    Value::I16(v) => {
                        max |= 0b0010;
                        sized = true;
                        v as u64
                    }
                    Value::U32(v) => {
                        max |= 0b0100;
                        v as u64
                    }
                    Value::I32(v) => {
                        max |= 0b0100;
                        sized = true;
                        v as u64
                    }
                    Value::U64(v) => {
                        max |= 0b1000;
                        v as u64
                    }
                    Value::I64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F32(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    Value::F64(v) => {
                        max |= 0b1000;
                        sized = true;
                        v as u64
                    }
                    _ => panic!("ERROR! ARITH INSTRUCTION COULD ONLY APPLY TO NUMERICAL VALUES"),
                };
                if max & 0b1000 == 0b1000 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I64((v1 as i64) >> v2 as i64);
                    } else {
                        stack_regs[e3 as usize] = Value::U64((v1 as u64) >> v2 as u64);
                    }
                } else if max & 0b0100 == 0b0100 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I32((v1 as i32) >> v2 as i32);
                    } else {
                        stack_regs[e3 as usize] = Value::U32((v1 as u32) >> v2 as u32);
                    }
                } else if max & 0b0010 == 0b0010 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I16((v1 as i16) >> v2 as i16);
                    } else {
                        stack_regs[e3 as usize] = Value::U16((v1 as u16) >> v2 as u16);
                    }
                } else if max & 0b0001 == 0b0001 {
                    if sized {
                        stack_regs[e3 as usize] = Value::I8((v1 as i8) >> v2 as i8);
                    } else {
                        stack_regs[e3 as usize] = Value::U8((v1 as u8) >> v2 as u8);
                    }
                } else {
                    panic!("卧槽? 咋了?!");
                }
            }

            _ if ins == OpCode::NOT as u16 => {
                if let Some(b) = value_to_bool(&stack_regs[e1 as usize]) {
                    stack_regs[e2 as usize] = Value::Boolean(!b);
                } else {
                    panic!("ERROR! NOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES")
                }
            }
            _ if ins == OpCode::LNOT as u16 => {
                match stack_regs[e1 as usize] {
                    Value::Boolean(v) => {
                        stack_regs[e2 as usize] = Value::Boolean(!v);
                    }
                    _ => panic!("ERROR! LNOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES"),
                };
            }
            _ if ins == OpCode::AND as u16 => {
                if let Some(b1) = value_to_bool(&stack_regs[e1 as usize]) {
                    if let Some(b2) = value_to_bool(&stack_regs[e2 as usize]) {
                        stack_regs[e3 as usize] = Value::Boolean(b1 && b2);
                    } else {
                        panic!("ERROR! LNOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES")
                    }
                } else {
                    panic!("ERROR! LNOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES")
                }
            }
            _ if ins == OpCode::LAND as u16 => {
                let b1 = match stack_regs[e1 as usize] {
                    Value::Boolean(v) => v,
                    _ => panic!("ERROR! LNOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES"),
                };
                let b2 = match stack_regs[e1 as usize] {
                    Value::Boolean(v) => v,
                    _ => panic!("ERROR! LNOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES"),
                };
                stack_regs[e3 as usize] = Value::Boolean(b1 && b2);
            }
            _ if ins == OpCode::OR as u16 => {
                if let Some(b1) = value_to_bool(&stack_regs[e1 as usize]) {
                    if let Some(b2) = value_to_bool(&stack_regs[e2 as usize]) {
                        stack_regs[e3 as usize] = Value::Boolean(b1 || b2);
                    } else {
                        panic!("ERROR! LNOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES")
                    }
                } else {
                    panic!("ERROR! LNOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES")
                }
            }
            _ if ins == OpCode::LOR as u16 => {
                let b1 = match stack_regs[e1 as usize] {
                    Value::Boolean(v) => v,
                    _ => panic!("ERROR! LNOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES"),
                };
                let b2 = match stack_regs[e1 as usize] {
                    Value::Boolean(v) => v,
                    _ => panic!("ERROR! LNOT INSTRUCTION COULD ONLY APPLY TO BOOLEAN VALUES"),
                };
                stack_regs[e3 as usize] = Value::Boolean(b1 || b2);
            }
            _ if ins == OpCode::EQ as u16 => {
                let mut cu1: u64 = 0;
                let mut cf1: f64 = 0.0;
                let mut to_f1 = false;
                let mut scomp1 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu1 = v as u64,
                    Value::I8(v) => cu1 = v as u64,
                    Value::U16(v) => cu1 = v as u64,
                    Value::I16(v) => cu1 = v as u64,
                    Value::U32(v) => cu1 = v as u64,
                    Value::I32(v) => cu1 = v as u64,
                    Value::U64(v) => cu1 = v as u64,
                    Value::I64(v) => cu1 = v as u64,
                    Value::F32(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    Value::F64(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    _ => scomp1 = true,
                };
                let mut cu2: u64 = 0;
                let mut cf2: f64 = 0.0;
                let mut to_f2 = false;
                let mut scomp2 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu2 = v as u64,
                    Value::I8(v) => cu2 = v as u64,
                    Value::U16(v) => cu2 = v as u64,
                    Value::I16(v) => cu2 = v as u64,
                    Value::U32(v) => cu2 = v as u64,
                    Value::I32(v) => cu2 = v as u64,
                    Value::U64(v) => cu2 = v as u64,
                    Value::I64(v) => cu2 = v as u64,
                    Value::F32(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    Value::F64(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    _ => scomp2 = true,
                };
                if scomp1 || scomp2 {
                    stack_regs[e3 as usize] =
                        Value::Boolean(stack_regs[e1 as usize] == stack_regs[e2 as usize]);
                } else if to_f1 || to_f2 {
                    if to_f1 && !to_f2 {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 == cu2 as f64);
                    } else if to_f2 && !to_f1 {
                        stack_regs[e3 as usize] = Value::Boolean(cf2 == cu1 as f64);
                    } else {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 == cf2);
                    }
                } else {
                    stack_regs[e3 as usize] = Value::Boolean(cu1 == cu2);
                }
            }
            _ if ins == OpCode::SEQ as u16 => {
                stack_regs[e3 as usize] =
                    Value::Boolean(stack_regs[e1 as usize] == stack_regs[e2 as usize]);
            }
            _ if ins == OpCode::LT as u16 => {
                let mut cu1: u64 = 0;
                let mut cf1: f64 = 0.0;
                let mut to_f1 = false;
                let mut scomp1 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu1 = v as u64,
                    Value::I8(v) => cu1 = v as u64,
                    Value::U16(v) => cu1 = v as u64,
                    Value::I16(v) => cu1 = v as u64,
                    Value::U32(v) => cu1 = v as u64,
                    Value::I32(v) => cu1 = v as u64,
                    Value::U64(v) => cu1 = v as u64,
                    Value::I64(v) => cu1 = v as u64,
                    Value::F32(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    Value::F64(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    _ => scomp1 = true,
                };
                let mut cu2: u64 = 0;
                let mut cf2: f64 = 0.0;
                let mut to_f2 = false;
                let mut scomp2 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu2 = v as u64,
                    Value::I8(v) => cu2 = v as u64,
                    Value::U16(v) => cu2 = v as u64,
                    Value::I16(v) => cu2 = v as u64,
                    Value::U32(v) => cu2 = v as u64,
                    Value::I32(v) => cu2 = v as u64,
                    Value::U64(v) => cu2 = v as u64,
                    Value::I64(v) => cu2 = v as u64,
                    Value::F32(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    Value::F64(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    _ => scomp2 = true,
                };
                if scomp1 || scomp2 {
                    stack_regs[e3 as usize] =
                        Value::Boolean(stack_regs[e1 as usize] == stack_regs[e2 as usize]);
                } else if to_f1 || to_f2 {
                    if to_f1 && !to_f2 {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 == cu2 as f64);
                    } else if to_f2 && !to_f1 {
                        stack_regs[e3 as usize] = Value::Boolean(cf2 == cu1 as f64);
                    } else {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 == cf2);
                    }
                } else {
                    stack_regs[e3 as usize] = Value::Boolean(cu1 == cu2);
                }
            }
            _ if ins == OpCode::LT as u16 => {
                let mut cu1: u64 = 0;
                let mut cf1: f64 = 0.0;
                let mut to_f1 = false;
                let mut scomp1 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu1 = v as u64,
                    Value::I8(v) => cu1 = v as u64,
                    Value::U16(v) => cu1 = v as u64,
                    Value::I16(v) => cu1 = v as u64,
                    Value::U32(v) => cu1 = v as u64,
                    Value::I32(v) => cu1 = v as u64,
                    Value::U64(v) => cu1 = v as u64,
                    Value::I64(v) => cu1 = v as u64,
                    Value::F32(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    Value::F64(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    _ => scomp1 = true,
                };
                let mut cu2: u64 = 0;
                let mut cf2: f64 = 0.0;
                let mut to_f2 = false;
                let mut scomp2 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu2 = v as u64,
                    Value::I8(v) => cu2 = v as u64,
                    Value::U16(v) => cu2 = v as u64,
                    Value::I16(v) => cu2 = v as u64,
                    Value::U32(v) => cu2 = v as u64,
                    Value::I32(v) => cu2 = v as u64,
                    Value::U64(v) => cu2 = v as u64,
                    Value::I64(v) => cu2 = v as u64,
                    Value::F32(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    Value::F64(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    _ => scomp2 = true,
                };
                if scomp1 || scomp2 {
                    panic!("ERROR LT COULD NOT COMPARE NON VALUE TYPE")
                } else if to_f1 || to_f2 {
                    if to_f1 && !to_f2 {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 < cu2 as f64);
                    } else if to_f2 && !to_f1 {
                        stack_regs[e3 as usize] = Value::Boolean(cf2 < cu1 as f64);
                    } else {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 < cf2);
                    }
                } else {
                    stack_regs[e3 as usize] = Value::Boolean(cu1 < cu2);
                }
            }
            _ if ins == OpCode::GT as u16 => {
                let mut cu1: u64 = 0;
                let mut cf1: f64 = 0.0;
                let mut to_f1 = false;
                let mut scomp1 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu1 = v as u64,
                    Value::I8(v) => cu1 = v as u64,
                    Value::U16(v) => cu1 = v as u64,
                    Value::I16(v) => cu1 = v as u64,
                    Value::U32(v) => cu1 = v as u64,
                    Value::I32(v) => cu1 = v as u64,
                    Value::U64(v) => cu1 = v as u64,
                    Value::I64(v) => cu1 = v as u64,
                    Value::F32(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    Value::F64(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    _ => scomp1 = true,
                };
                let mut cu2: u64 = 0;
                let mut cf2: f64 = 0.0;
                let mut to_f2 = false;
                let mut scomp2 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu2 = v as u64,
                    Value::I8(v) => cu2 = v as u64,
                    Value::U16(v) => cu2 = v as u64,
                    Value::I16(v) => cu2 = v as u64,
                    Value::U32(v) => cu2 = v as u64,
                    Value::I32(v) => cu2 = v as u64,
                    Value::U64(v) => cu2 = v as u64,
                    Value::I64(v) => cu2 = v as u64,
                    Value::F32(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    Value::F64(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    _ => scomp2 = true,
                };
                if scomp1 || scomp2 {
                    panic!("ERROR LT COULD NOT COMPARE NON VALUE TYPE")
                } else if to_f1 || to_f2 {
                    if to_f1 && !to_f2 {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 > cu2 as f64);
                    } else if to_f2 && !to_f1 {
                        stack_regs[e3 as usize] = Value::Boolean(cf2 > cu1 as f64);
                    } else {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 > cf2);
                    }
                } else {
                    stack_regs[e3 as usize] = Value::Boolean(cu1 > cu2);
                }
            }
            _ if ins == OpCode::LTEQ as u16 => {
                let mut cu1: u64 = 0;
                let mut cf1: f64 = 0.0;
                let mut to_f1 = false;
                let mut scomp1 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu1 = v as u64,
                    Value::I8(v) => cu1 = v as u64,
                    Value::U16(v) => cu1 = v as u64,
                    Value::I16(v) => cu1 = v as u64,
                    Value::U32(v) => cu1 = v as u64,
                    Value::I32(v) => cu1 = v as u64,
                    Value::U64(v) => cu1 = v as u64,
                    Value::I64(v) => cu1 = v as u64,
                    Value::F32(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    Value::F64(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    _ => scomp1 = true,
                };
                let mut cu2: u64 = 0;
                let mut cf2: f64 = 0.0;
                let mut to_f2 = false;
                let mut scomp2 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu2 = v as u64,
                    Value::I8(v) => cu2 = v as u64,
                    Value::U16(v) => cu2 = v as u64,
                    Value::I16(v) => cu2 = v as u64,
                    Value::U32(v) => cu2 = v as u64,
                    Value::I32(v) => cu2 = v as u64,
                    Value::U64(v) => cu2 = v as u64,
                    Value::I64(v) => cu2 = v as u64,
                    Value::F32(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    Value::F64(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    _ => scomp2 = true,
                };
                if scomp1 || scomp2 {
                    panic!("ERROR LT COULD NOT COMPARE NON VALUE TYPE")
                } else if to_f1 || to_f2 {
                    if to_f1 && !to_f2 {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 <= cu2 as f64);
                    } else if to_f2 && !to_f1 {
                        stack_regs[e3 as usize] = Value::Boolean(cf2 <= cu1 as f64);
                    } else {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 <= cf2);
                    }
                } else {
                    stack_regs[e3 as usize] = Value::Boolean(cu1 <= cu2);
                }
            }
            _ if ins == OpCode::GTEQ as u16 => {
                let mut cu1: u64 = 0;
                let mut cf1: f64 = 0.0;
                let mut to_f1 = false;
                let mut scomp1 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu1 = v as u64,
                    Value::I8(v) => cu1 = v as u64,
                    Value::U16(v) => cu1 = v as u64,
                    Value::I16(v) => cu1 = v as u64,
                    Value::U32(v) => cu1 = v as u64,
                    Value::I32(v) => cu1 = v as u64,
                    Value::U64(v) => cu1 = v as u64,
                    Value::I64(v) => cu1 = v as u64,
                    Value::F32(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    Value::F64(v) => {
                        to_f1 = true;
                        cf1 = v as f64
                    }
                    _ => scomp1 = true,
                };
                let mut cu2: u64 = 0;
                let mut cf2: f64 = 0.0;
                let mut to_f2 = false;
                let mut scomp2 = false;
                match stack_regs[e1 as usize] {
                    Value::U8(v) => cu2 = v as u64,
                    Value::I8(v) => cu2 = v as u64,
                    Value::U16(v) => cu2 = v as u64,
                    Value::I16(v) => cu2 = v as u64,
                    Value::U32(v) => cu2 = v as u64,
                    Value::I32(v) => cu2 = v as u64,
                    Value::U64(v) => cu2 = v as u64,
                    Value::I64(v) => cu2 = v as u64,
                    Value::F32(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    Value::F64(v) => {
                        to_f2 = true;
                        cf2 = v as f64
                    }
                    _ => scomp2 = true,
                };
                if scomp1 || scomp2 {
                    panic!("ERROR LT COULD NOT COMPARE NON VALUE TYPE")
                } else if to_f1 || to_f2 {
                    if to_f1 && !to_f2 {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 >= cu2 as f64);
                    } else if to_f2 && !to_f1 {
                        stack_regs[e3 as usize] = Value::Boolean(cf2 >= cu1 as f64);
                    } else {
                        stack_regs[e3 as usize] = Value::Boolean(cf1 >= cf2);
                    }
                } else {
                    stack_regs[e3 as usize] = Value::Boolean(cu1 >= cu2);
                }
            }

            _ if ins == OpCode::JMP as u16 => {
                pc = e1 - 1;
            }
            _ if ins == OpCode::JPE as u16 => {
                if let Value::Boolean(b) = stack_regs[e1 as usize] {
                    if b {
                        pc = e1 - 1;
                    }
                } else {
                    panic!("ERROR! JPE COULD NOT PASS NON BOOLEAN VALUE")
                }
            }
            _ if ins == OpCode::JPN as u16 => {
                if let Value::Boolean(b) = stack_regs[e1 as usize] {
                    if !b {
                        pc = e1 - 1;
                    }
                } else {
                    panic!("ERROR! JPN COULD NOT PASS NON BOOLEAN VALUE")
                }
            }
            _ if ins == OpCode::ARGS as u16 => {
                state.args.push(stack_regs[e1 as usize]);
            }
            _ if ins == OpCode::CALL as u16 => {
                if let Value::NSValue(n) = stack_regs[e1 as usize]{
                    if let NSValue::Closure(c) = n {
                        // save status
                        state.current_function_call_state.registers = stack_regs.to_vec();
                        state.current_function_call_state.pc = pc;
                        // push current function to function call stack
                        state.function_call_chain_states.push(state.current_function_call_state.clone());
                        // change current function
                        state.current_function_call_state = unsafe{c.as_ref()}.clone();
                        // copy args into function args
                        state.current_function_call_state.args = state.args.clone();
                        return interpreter(state);
                    }
                }
                // TODO: implement gc closure call
            }
            _ if ins == OpCode::RET as u16 => {
                if e1 == 0xFFFF{
                    state.return_value = None;
                }else{
                    state.return_value = Some(stack_regs[e1 as usize]);
                }
                if let Some(cls) = state.function_call_chain_states.pop() {
                    state.current_function_call_state = cls;
                    state.current_function_call_state.pc += 1;
                    return interpreter(state);
                }
                // the outer
                else {
                    return;
                }
            }
            _ if ins == OpCode::RETN as u16 => {
                state.return_values.push(stack_regs[e1 as usize]);
            }
            _ => unimplemented!(),
        }

        pc += 1;
    }

    // restore status
    state.current_function_call_state = current_function_state;
}
