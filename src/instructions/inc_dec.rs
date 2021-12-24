use crate::computer::Computer;

use super::{Executable, operands::{get_next_reg_operand, OperandType}};

pub mod opcodes {
    pub const INC_OPCODE: u8 = 6;
    pub const DEC_OPCODE: u8 = 7;
}

pub mod structs {
    pub struct Inc;
    pub struct Dec;
}

use opcodes::*;
use structs::*;

fn assemble_inc_dec(opcode: u8, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
    vec![
        opcode << 2,
        operands[0][1..].parse::<u8>().expect("Invalid register id") << 4,
    ]
}

impl Executable for Inc {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.regs.common[reg] += 1;
    }

    fn mnemonic(&self) -> String { String::from("inc") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_inc_dec(INC_OPCODE, operands, operand_types)
    }
}

impl Executable for Dec {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.regs.common[reg] -= 1;
    }

    fn mnemonic(&self) -> String { String::from("dec") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_inc_dec(DEC_OPCODE, operands, operand_types)
    }
}
