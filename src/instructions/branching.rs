use std::cmp::Ordering;

use crate::computer::Computer;

use super::{Executable, operands::{get_next_reg_reg_operands, OperandType, get_next_reg_operand}};

pub mod opcodes {
    pub const CMP_OPCODE: u8 = 12;

    pub const JMP_OPCODE: u8 = 13;

    pub const JL_OPCODE: u8 = 14;
    pub const JGE_OPCODE: u8 = 16;
    pub const JG_OPCODE: u8 = 17;
    pub const JLE_OPCODE: u8 = 18;
    pub const JE_OPCODE: u8 = 19;
    pub const JNE_OPCODE: u8 = 20;
}

pub mod structs {
    use std::cmp::Ordering;

    pub struct Cmp;
    pub struct Jmp;
    pub struct Jcond(pub bool, pub Ordering); // true - cond, false - ncond
}

use opcodes::*;
use structs::*;

impl Executable for Cmp {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.flags = computer.regs.common[reg1].cmp(&computer.regs.common[reg2]);
    }

    fn mnemonic(&self) -> String { String::from("cmp") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![
            CMP_OPCODE << 2,
            (operands[0][1..].parse::<u8>().expect("Invalid register id") << 4) +
            operands[1][1..].parse::<u8>().expect("Invalid register id"),
        ]
    }
}

impl Executable for Jmp {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.ip = computer.regs.common[reg];
    }

    fn mnemonic(&self) -> String { String::from("jmp") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![
            JMP_OPCODE << 2,
            operands[0][1..].parse::<u8>().expect("Invalid register id") << 4,
        ]
    }
}

impl Executable for Jcond {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        if (computer.regs.flags == self.1) == self.0 {
            computer.ip = computer.regs.common[reg];
        }
    }

    fn mnemonic(&self) -> String {
        let mut m = String::new();
        m.reserve(3);
        
        m.push('j');
        if ! self.0 {
            m.push('n');
        }
        m.push(match self.1 {
            Ordering::Less    => 'l',
            Ordering::Equal   => 'e',
            Ordering::Greater => 'g',
        });

        m
    }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        let opcode = if self.0 {
            match self.1 {
                Ordering::Less    => JL_OPCODE,
                Ordering::Equal   => JE_OPCODE,
                Ordering::Greater => JG_OPCODE,
            }
        } else {
            match self.1 {
                Ordering::Less    => JGE_OPCODE,
                Ordering::Equal   => JNE_OPCODE,
                Ordering::Greater => JLE_OPCODE,
            }
        };

        vec![
            opcode << 2,
            operands[0][1..].parse::<u8>().expect("Invalid register id") << 4,
        ]
    }
}
