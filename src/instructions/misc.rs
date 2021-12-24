use crate::computer::Computer;

use super::{Executable, operands::OperandType};

pub mod opcodes {
    pub const NOP_OPCODE: u8 = 0;
    pub const HALT_OPCODE: u8 = 1;
}

pub mod structs {
    pub struct Nop;
    pub struct Halt;
}

use opcodes::*;
use structs::*;

impl Executable for Nop {
    fn execute(&self, _computer: &mut Computer, _first_byte: u8) {}
    fn mnemonic(&self) -> String { String::from("nop") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![NOP_OPCODE << 2]
    }
    
}

impl Executable for Halt {
    fn execute(&self, computer: &mut Computer, _first_byte: u8) { computer.should_halt = true }
    fn mnemonic(&self) -> String { String::from("halt") }
    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![HALT_OPCODE << 2]
    }
    
}
