pub mod operands;
use self::operands::OperandType;

pub mod misc;
pub use misc::{opcodes::*, structs::*};

pub mod basic_math;
pub use basic_math::{opcodes::*, structs::*};

pub mod inc_dec;
pub use inc_dec::{opcodes::*, structs::*};

pub mod data_movement;
pub use data_movement::{opcodes::*, structs::*};

pub mod branching;
pub use branching::{opcodes::*, structs::*};


use std::{cmp::Ordering, collections::{HashMap}, vec};
use lazy_static::lazy_static;

use crate::computer::Computer;


pub trait Executable {
    fn execute(&self, computer: &mut Computer, first_byte: u8);
    fn mnemonic(&self) -> String;
    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8>;
}

lazy_static! {
    pub static ref INSTRUCTIONS: HashMap <u8, Box<dyn Executable + Sync>> = {
        let mut instrs = HashMap::<u8, Box<dyn Executable + Sync>>::new();

        instrs.insert(NOP_OPCODE, Box::from(Nop));
        instrs.insert(HALT_OPCODE, Box::from(Halt));

        instrs.insert(ADD_OPCODE, Box::from(Add));
        instrs.insert(SUB_OPCODE, Box::from(Sub));
        instrs.insert(MUL_OPCODE, Box::from(Mul));
        instrs.insert(DIV_OPCODE, Box::from(Div));

        instrs.insert(INC_OPCODE, Box::from(Inc));
        instrs.insert(DEC_OPCODE, Box::from(Dec));

        instrs.insert(LDR_OPCODE, Box::from(Ldr));
        instrs.insert(STR_OPCODE, Box::from(Str));
        instrs.insert(PUT_OPCODE, Box::from(Put));
        instrs.insert(MOV_OPCODE, Box::from(Mov));

        instrs.insert(CMP_OPCODE, Box::from(Cmp));
        instrs.insert(JMP_OPCODE, Box::from(Jmp));
        instrs.insert(JL_OPCODE, Box::from(Jcond(true,  Ordering::Less)));     // jl
        instrs.insert(JGE_OPCODE, Box::from(Jcond(false, Ordering::Less)));    // jge
        instrs.insert(JG_OPCODE, Box::from(Jcond(true,  Ordering::Greater)));  // jg
        instrs.insert(JLE_OPCODE, Box::from(Jcond(false, Ordering::Greater))); // jle
        instrs.insert(JE_OPCODE, Box::from(Jcond(true,  Ordering::Equal)));    // je
        instrs.insert(JNE_OPCODE, Box::from(Jcond(false, Ordering::Equal)));   // jne

        instrs
    };

    pub static ref INSTRUCTION_SIZE: Vec <(Vec<OperandType>, u8)> = {
        let mut sizes: Vec <(Vec<OperandType>, u8)> = Vec::new();

        sizes.push((vec![OperandType::Register, OperandType::Register], 2));
        sizes.push((vec![OperandType::Register, OperandType::Value], 3));
        sizes.push((vec![OperandType::Register], 2));
        sizes.push((vec![OperandType::Value], 2));
        sizes.push((Vec::new(), 1));

        sizes
    };
}

pub fn get_instruction_size(operand_types: &Vec<OperandType>) -> u8 {
    let mut equality: bool;

    for entry in (& INSTRUCTION_SIZE).iter() {
        equality = true;
        if operand_types.len() == entry.0.len() {
            for op in operand_types.iter().enumerate() {
                if * op.1 != entry.0[op.0] {
                    equality = false;
                    break;
                }
            }
        } else {
            equality = false;
        }

        if equality {
            return entry.1;
        }
    }

    return 0;
}

pub fn instr_from_str(s: &str) -> Option<&'static Box<dyn Executable + Sync + 'static>> {
    for instr in INSTRUCTIONS.values() {
        if instr.mnemonic() == s {
            return Some(&instr);
        }
    }

    return None
}
