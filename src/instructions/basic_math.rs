use crate::computer::Computer;

use super::{Executable, operands::{get_next_reg_reg_operands, OperandType}};

pub mod opcodes {
    pub const ADD_OPCODE: u8 = 2;
    pub const SUB_OPCODE: u8 = 3;
    pub const MUL_OPCODE: u8 = 4;
    pub const DIV_OPCODE: u8 = 5;
}

pub mod structs {
    pub struct Add;
    pub struct Sub;
    pub struct Mul;
    pub struct Div;
}

use opcodes::*;
use structs::*;

fn assemble_basic_math(opcode: u8, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
    vec![
        opcode << 2,
        (operands[0][1..].parse::<u8>().expect("Invalid register id") << 4) +
        operands[1][1..].parse::<u8>().expect("Invalid register id"),
    ]
}

impl Executable for Add {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.common_registers[reg1] += computer.common_registers[reg2];
    }

    fn mnemonic(&self) -> String { String::from("add") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(ADD_OPCODE, operands, operand_types)
    }
}

impl Executable for Sub {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.common_registers[reg1] -= computer.common_registers[reg2];
    }

    fn mnemonic(&self) -> String { String::from("sub") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(SUB_OPCODE, operands, operand_types)
    }
}

impl Executable for Mul {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.common_registers[reg1] *= computer.common_registers[reg2];
    }

    fn mnemonic(&self) -> String { String::from("mul") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(MUL_OPCODE, operands, operand_types)
    }
}

impl Executable for Div {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        
        let value1 = computer.common_registers[reg1];
        let value2 = computer.common_registers[reg2];

        let div = value1 / value2;
        let rem = value1 % value2;

        computer.common_registers[0] = div;
        computer.common_registers[1] = rem;
    }

    fn mnemonic(&self) -> String { String::from("div") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(DIV_OPCODE, operands, operand_types)
    }
}
