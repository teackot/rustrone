use std::{cmp::Ordering, collections::{HashMap}};
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
}

//------------------
// Nop, Halt

struct Nop;
impl Executable for Nop {
    fn execute(&self, _computer: &mut Computer, _first_byte: u8) {}
}

struct Halt;
impl Executable for Halt {
    fn execute(&self, computer: &mut Computer, _first_byte: u8) { computer.should_halt = true }
}

//------------------
// Basic math

struct Add;
impl Executable for Add {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] += computer.regs.common[reg2];
    }
}

struct Sub;
impl Executable for Sub {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] -= computer.regs.common[reg2];
    }
}

struct Mul;
impl Executable for Mul {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] *= computer.regs.common[reg2];
    }
}

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
}

struct Inc;
impl Executable for Inc {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.regs.common[reg] += 1;
    }
}

struct Dec;
impl Executable for Dec {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.regs.common[reg] -= 1;
    }
}

//------------------
// Data movement

struct Ldr;
impl Executable for Ldr {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] = computer.memory[computer.regs.common[reg2] as usize];
    }
}

struct Str;
impl Executable for Str {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.memory[computer.regs.common[reg2] as usize] = computer.regs.common[reg1];
    }
}

struct Mov;
impl Executable for Mov {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] = computer.regs.common[reg2];
    }
}

struct Put;
impl Executable for Put {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.regs.common[reg] = computer.next_byte();
    }
}

//----------------
// Branching

struct Cmp;
impl Executable for Cmp {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.flags = computer.regs.common[reg1].cmp(&computer.regs.common[reg2]);
    }
}

struct Jmp;
impl Executable for Jmp {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.ip = computer.regs.common[reg];
    }
}

// true - cond, false - ncond
struct Jcond(bool, Ordering);
impl Executable for Jcond {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        if (computer.regs.flags == self.1) == self.0 {
            computer.ip = computer.regs.common[reg];
        }
    }
}

//----------------

lazy_static! {
    pub static ref INSTRUCTIONS: HashMap <u8, Box<dyn Executable + Sync>> = {
        let mut instrs = HashMap::<u8, Box<dyn Executable + Sync>>::new();

        instrs.insert(0u8, Box::from(Nop));

        instrs.insert(1u8, Box::from(Add));
        instrs.insert(2u8, Box::from(Sub));
        instrs.insert(3u8, Box::from(Mul));
        instrs.insert(4u8, Box::from(Div));

        instrs.insert(5u8, Box::from(Inc));
        instrs.insert(6u8, Box::from(Dec));

        instrs.insert(7u8, Box::from(Ldr));
        instrs.insert(8u8, Box::from(Str));
        instrs.insert(9u8, Box::from(Put));
        instrs.insert(10u8, Box::from(Mov));

        instrs.insert(13u8, Box::from(Halt));

        instrs.insert(14u8, Box::from(Cmp));
        instrs.insert(15u8, Box::from(Jmp));
        instrs.insert(16u8, Box::from(Jcond(true,  Ordering::Less)));     // jl
        instrs.insert(17u8, Box::from(Jcond(false, Ordering::Less)));    // jge
        instrs.insert(18u8, Box::from(Jcond(true,  Ordering::Greater)));  // jg
        instrs.insert(19u8, Box::from(Jcond(false, Ordering::Greater))); // jle
        instrs.insert(20u8, Box::from(Jcond(true,  Ordering::Equal)));    // je
        instrs.insert(21u8, Box::from(Jcond(false, Ordering::Equal)));   // jne

        instrs
    };
}

pub enum OperandType {
    Register,
    Value,
    Depends,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Instr {
    Nop,

    Add,
    Sub,
    Mul,
    Div,

    Inc,
    Dec,

    Ldr,
    Str,
    Put,
    Mov,

    Push,
    Pop,

    Halt,

    Cmp,
    Jmp,
    Jcond(Ordering),
    Jncond(Ordering),

    // Jl, Jge,
    // Jg, Jle,
    // Je, Jne,
}

lazy_static! {
    static ref OPCODES: HashMap <u8, (Instr, &'static str, u8, Vec<OperandType>)> = {
        let mut opcodes = HashMap::new();

        let opcodes_arr = [
            (0u8,  (Instr::Nop, "nop", 0, Vec::new())),
            
            (1u8,  (Instr::Add, "add", 2, vec![OperandType::Register, OperandType::Register])),
            (2u8,  (Instr::Sub, "sub", 2, vec![OperandType::Register, OperandType::Register])),
            (3u8,  (Instr::Mul, "mul", 2, vec![OperandType::Register, OperandType::Register])),
            (4u8,  (Instr::Div, "div", 2, vec![OperandType::Register, OperandType::Register])),

            (5u8,  (Instr::Inc, "inc", 1, vec![OperandType::Register])),
            (6u8,  (Instr::Dec, "dec", 1, vec![OperandType::Register])),

            (7u8,  (Instr::Ldr, "ldr", 2, vec![OperandType::Register, OperandType::Register])),
            (8u8,  (Instr::Str, "str", 2, vec![OperandType::Register, OperandType::Register])),
            (9u8,  (Instr::Put, "put", 2, vec![OperandType::Register, OperandType::Value])),
            (10u8, (Instr::Mov, "mov", 2, vec![OperandType::Register, OperandType::Register])),

            (11u8, (Instr::Push, "push", 1, vec![OperandType::Register])),
            (12u8, (Instr::Pop, "pop", 1, vec![OperandType::Register])),

            (13u8, (Instr::Halt, "halt", 0, Vec::new())),

            (14u8, (Instr::Cmp, "cmp", 2, vec![OperandType::Register, OperandType::Register])),
            (15u8, (Instr::Jmp, "jmp", 1, vec![OperandType::Depends])),
            (16u8, (Instr::Jcond(Ordering::Less), "jl", 1, vec![OperandType::Depends])),
            (17u8, (Instr::Jncond(Ordering::Less), "jge", 1, vec![OperandType::Depends])),
            (18u8, (Instr::Jcond(Ordering::Greater), "jg", 1, vec![OperandType::Depends])),
            (19u8, (Instr::Jncond(Ordering::Greater), "jle", 1, vec![OperandType::Depends])),
            (20u8, (Instr::Jcond(Ordering::Equal), "je", 1, vec![OperandType::Depends])),
            (21u8, (Instr::Jncond(Ordering::Equal), "jne", 1, vec![OperandType::Depends])),
        ];

        for op in opcodes_arr {
            opcodes.insert(op.0, op.1);
        }

        opcodes
    };
}

impl Instr {
    pub fn from(instr: u8) -> Self {
        match OPCODES.get(&instr) {
            Some(i) => i.0.clone(),
            None => Instr::Halt
        }
    }

    pub fn from_str(instr: &str) -> Option<Instr> {
        for entry in OPCODES.iter() {
            if entry.1.1 == instr {
                return Some(entry.1.0.clone());
            }
        }

        None
    }

    pub fn to_u8(&self) -> u8 {
        let mut ret = 13u8; // Default is halt

        for entry in OPCODES.iter() {
            if entry.1.0 == *self {
                ret = entry.0.clone();
                break;
            }
        }

        ret
    }

    pub fn get_operand_count(&self) -> u8 {
        let mut ret = 0u8;

        for entry in OPCODES.iter() {
            if entry.1.0 == *self {
                ret = entry.1.2;
                break;
            }
        }
        ret
    }
}
