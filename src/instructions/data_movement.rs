use crate::computer::Computer;

use super::{operands::{OperandType, get_next_reg_reg_operands, get_next_reg_operand}, Executable};

pub mod opcodes {
    pub const LDR_OPCODE: u8 = 8;
    pub const STR_OPCODE: u8 = 9;
    pub const MOV_OPCODE: u8 = 10;
    pub const PUT_OPCODE: u8 = 11;
}

pub mod structs {
    pub struct Ldr;
    pub struct Str;
    pub struct Mov;
    pub struct Put;
}

use opcodes::*;
use structs::*;

fn assemble_ldr_str(opcode: u8, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
    vec![
        opcode << 2,
        (operands[0][1..].parse::<u8>().expect("Invalid register id") << 4) +
        operands[1][1..].parse::<u8>().expect("Invalid value"),
    ]
}

impl Executable for Ldr {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] = computer.memory[computer.regs.common[reg2] as usize];
    }

    fn mnemonic(&self) -> String { String::from("ldr") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_ldr_str(LDR_OPCODE, operands, operand_types)
    }
}

impl Executable for Str {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.memory[computer.regs.common[reg2] as usize] = computer.regs.common[reg1];
    }

    fn mnemonic(&self) -> String { String::from("str") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_ldr_str(STR_OPCODE, operands, operand_types)
    }
}

impl Executable for Mov {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] = computer.regs.common[reg2];
    }

    fn mnemonic(&self) -> String { String::from("mov") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![
            MOV_OPCODE << 2,
            (operands[0][1..].parse::<u8>().expect("Invalid register id") << 4) +
            operands[1][1..].parse::<u8>().expect("Invalid register id"),
        ]
    }
}

impl Executable for Put {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.regs.common[reg] = computer.next_byte();
    }

    fn mnemonic(&self) -> String { String::from("put") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![
            PUT_OPCODE << 2,
            operands[0][1..].parse::<u8>().expect("Invalid register id") << 4,
            operands[1].parse::<u8>().expect("Invalid value"),
        ]
    }
}
