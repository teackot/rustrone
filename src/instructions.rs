use std::{cmp::Ordering, collections::{HashMap}, vec};
use lazy_static::lazy_static;

use crate::computer::Computer;

fn get_next_reg_reg_operands(computer: &mut Computer) -> (usize, usize) {
    let regs_byte = computer.next_byte();
    let reg1 = ((regs_byte & 0b11110000) >> 4) as usize;
    let reg2 = (regs_byte & 0b00001111) as usize;

    (reg1, reg2)
}

fn get_next_reg_operand(computer: &mut Computer) -> usize {
    let regs_byte = computer.next_byte();
    let reg = ((regs_byte & 0b11110000) >> 4) as usize;

    reg
}

pub trait Executable {
    fn execute(&self, computer: &mut Computer, first_byte: u8);
    fn mnemonic(&self) -> String;
    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8>;
}

//------------------
// Nop, Halt

const NOP_OPCODE: u8 = 0;
struct Nop;
impl Executable for Nop {
    fn execute(&self, _computer: &mut Computer, _first_byte: u8) {}
    fn mnemonic(&self) -> String { String::from("nop") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![NOP_OPCODE << 2]
    }
    
}

const HALT_OPCODE: u8 = 1;
struct Halt;
impl Executable for Halt {
    fn execute(&self, computer: &mut Computer, _first_byte: u8) { computer.should_halt = true }
    fn mnemonic(&self) -> String { String::from("halt") }
    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![HALT_OPCODE << 2]
    }
    
}

//------------------
// Basic math

fn assemble_basic_math(opcode: u8, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
    vec![
        opcode << 2,
        (operands[0][1..].parse::<u8>().expect("Invalid register id") << 4) +
        operands[1][1..].parse::<u8>().expect("Invalid register id"),
    ]
}

const ADD_OPCODE: u8 = 2;
struct Add;
impl Executable for Add {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] += computer.regs.common[reg2];
    }

    fn mnemonic(&self) -> String { String::from("add") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(ADD_OPCODE, operands, operand_types)
    }
}

const SUB_OPCODE: u8 = 3;
struct Sub;
impl Executable for Sub {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] -= computer.regs.common[reg2];
    }

    fn mnemonic(&self) -> String { String::from("sub") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(SUB_OPCODE, operands, operand_types)
    }
}

const MUL_OPCODE: u8 = 4;
struct Mul;
impl Executable for Mul {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] *= computer.regs.common[reg2];
    }

    fn mnemonic(&self) -> String { String::from("mul") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(MUL_OPCODE, operands, operand_types)
    }
}

const DIV_OPCODE: u8 = 5;
struct Div;
impl Executable for Div {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        
        let value1 = computer.regs.common[reg1];
        let value2 = computer.regs.common[reg2];

        let div = value1 / value2;
        let rem = value1 % value2;

        computer.regs.common[0] = div;
        computer.regs.common[1] = rem;
    }

    fn mnemonic(&self) -> String { String::from("div") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(DIV_OPCODE, operands, operand_types)
    }
}

//----------------
// Inc and dec

fn assemble_inc_dec(opcode: u8, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
    vec![
        opcode << 2,
        operands[0][1..].parse::<u8>().expect("Invalid register id") << 4,
    ]
}

const INC_OPCODE: u8 = 6;
struct Inc;
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

const DEC_OPCODE: u8 = 7;
struct Dec;
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

//------------------
// Data movement

fn assemble_ldr_str(opcode: u8, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
    vec![
        opcode << 2,
        (operands[0][1..].parse::<u8>().expect("Invalid register id") << 4) +
        operands[1][1..].parse::<u8>().expect("Invalid value"),
    ]
}

const LDR_OPCODE: u8 = 8;
struct Ldr;
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

const STR_OPCODE: u8 = 9;
struct Str;
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

const MOV_OPCODE: u8 = 10;
struct Mov;
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

const PUT_OPCODE: u8 = 11;
struct Put;
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

//----------------
// Branching

const CMP_OPCODE: u8 = 12;
struct Cmp;
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

const JMP_OPCODE: u8 = 13;
struct Jmp;
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

const JL_OPCODE: u8 = 14;
const JGE_OPCODE: u8 = 16;
const JG_OPCODE: u8 = 17;
const JLE_OPCODE: u8 = 18;
const JE_OPCODE: u8 = 19;
const JNE_OPCODE: u8 = 20;
// true - cond, false - ncond
struct Jcond(bool, Ordering);
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

//----------------

#[derive(PartialEq, Debug)]
pub enum OperandType {
    Register,
    Value,
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
