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

static nop_opcode: u8 = 0;
struct Nop;
impl Executable for Nop {
    fn execute(&self, _computer: &mut Computer, _first_byte: u8) {}
    fn mnemonic(&self) -> String { String::from("nop") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![nop_opcode << 2]
    }
    
}

static halt_opcode: u8 = 1;
struct Halt;
impl Executable for Halt {
    fn execute(&self, computer: &mut Computer, _first_byte: u8) { computer.should_halt = true }
    fn mnemonic(&self) -> String { String::from("halt") }
    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![halt_opcode << 2]
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

static add_opcode: u8 = 2;
struct Add;
impl Executable for Add {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] += computer.regs.common[reg2];
    }

    fn mnemonic(&self) -> String { String::from("add") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(add_opcode, operands, operand_types)
    }
}

static sub_opcode: u8 = 3;
struct Sub;
impl Executable for Sub {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] -= computer.regs.common[reg2];
    }

    fn mnemonic(&self) -> String { String::from("sub") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(sub_opcode, operands, operand_types)
    }
}

static mul_opcode: u8 = 4;
struct Mul;
impl Executable for Mul {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] *= computer.regs.common[reg2];
    }

    fn mnemonic(&self) -> String { String::from("mul") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_basic_math(mul_opcode, operands, operand_types)
    }
}

static div_opcode: u8 = 5;
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
        assemble_basic_math(div_opcode, operands, operand_types)
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

static inc_opcode: u8 = 6;
struct Inc;
impl Executable for Inc {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.regs.common[reg] += 1;
    }

    fn mnemonic(&self) -> String { String::from("inc") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_inc_dec(inc_opcode, operands, operand_types)
    }
}

static dec_opcode: u8 = 7;
struct Dec;
impl Executable for Dec {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.regs.common[reg] -= 1;
    }

    fn mnemonic(&self) -> String { String::from("dec") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_inc_dec(dec_opcode, operands, operand_types)
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

static ldr_opcode: u8 = 8;
struct Ldr;
impl Executable for Ldr {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] = computer.memory[computer.regs.common[reg2] as usize];
    }

    fn mnemonic(&self) -> String { String::from("ldr") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_ldr_str(ldr_opcode, operands, operand_types)
    }
}

static str_opcode: u8 = 9;
struct Str;
impl Executable for Str {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.memory[computer.regs.common[reg2] as usize] = computer.regs.common[reg1];
    }

    fn mnemonic(&self) -> String { String::from("str") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        assemble_ldr_str(str_opcode, operands, operand_types)
    }
}

static mov_opcode: u8 = 10;
struct Mov;
impl Executable for Mov {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.common[reg1] = computer.regs.common[reg2];
    }

    fn mnemonic(&self) -> String { String::from("mov") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![
            mov_opcode << 2,
            (operands[0][1..].parse::<u8>().expect("Invalid register id") << 4) +
            operands[1][1..].parse::<u8>().expect("Invalid register id"),
        ]
    }
}

static put_opcode: u8 = 11;
struct Put;
impl Executable for Put {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.regs.common[reg] = computer.next_byte();
    }

    fn mnemonic(&self) -> String { String::from("put") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![
            put_opcode << 2,
            operands[0][1..].parse::<u8>().expect("Invalid register id") << 4,
            operands[1].parse::<u8>().expect("Invalid value"),
        ]
    }
}

//----------------
// Branching

static cmp_opcode: u8 = 12;
struct Cmp;
impl Executable for Cmp {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let (reg1, reg2) = get_next_reg_reg_operands(computer);
        computer.regs.flags = computer.regs.common[reg1].cmp(&computer.regs.common[reg2]);
    }

    fn mnemonic(&self) -> String { String::from("cmp") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![
            cmp_opcode << 2,
            (operands[0][1..].parse::<u8>().expect("Invalid register id") << 4) +
            operands[1][1..].parse::<u8>().expect("Invalid register id"),
        ]
    }
}

static jmp_opcode: u8 = 13;
struct Jmp;
impl Executable for Jmp {
    fn execute(&self, computer: &mut Computer, first_byte: u8) {
        let reg = get_next_reg_operand(computer);
        computer.ip = computer.regs.common[reg];
    }

    fn mnemonic(&self) -> String { String::from("jmp") }

    fn assemble(&self, operands: & Vec<String>, operand_types: &Vec<OperandType>) -> Vec<u8> {
        vec![
            jmp_opcode << 2,
            operands[0][1..].parse::<u8>().expect("Invalid register id") << 4,
        ]
    }
}

static jl_opcode: u8 = 14;
static jge_opcode: u8 = 16;
static jg_opcode: u8 = 17;
static jle_opcode: u8 = 18;
static je_opcode: u8 = 19;
static jne_opcode: u8 = 20;
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
                Ordering::Less    => jl_opcode,
                Ordering::Equal   => je_opcode,
                Ordering::Greater => jg_opcode,
            }
        } else {
            match self.1 {
                Ordering::Less    => jge_opcode,
                Ordering::Equal   => jne_opcode,
                Ordering::Greater => jle_opcode,
            }
        };

        vec![
            opcode << 2,
            operands[0][1..].parse::<u8>().expect("Invalid register id") << 4,
        ]
    }
}

//----------------

#[derive(PartialEq)]
pub enum OperandType {
    Register,
    Value,
}

lazy_static! {
    pub static ref INSTRUCTIONS: HashMap <u8, Box<dyn Executable + Sync>> = {
        let mut instrs = HashMap::<u8, Box<dyn Executable + Sync>>::new();

        instrs.insert(nop_opcode, Box::from(Nop));
        instrs.insert(halt_opcode, Box::from(Halt));

        instrs.insert(add_opcode, Box::from(Add));
        instrs.insert(sub_opcode, Box::from(Sub));
        instrs.insert(mul_opcode, Box::from(Mul));
        instrs.insert(div_opcode, Box::from(Div));

        instrs.insert(inc_opcode, Box::from(Inc));
        instrs.insert(dec_opcode, Box::from(Dec));

        instrs.insert(ldr_opcode, Box::from(Ldr));
        instrs.insert(str_opcode, Box::from(Str));
        instrs.insert(put_opcode, Box::from(Put));
        instrs.insert(mov_opcode, Box::from(Mov));

        instrs.insert(cmp_opcode, Box::from(Cmp));
        instrs.insert(jmp_opcode, Box::from(Jmp));
        instrs.insert(jl_opcode, Box::from(Jcond(true,  Ordering::Less)));     // jl
        instrs.insert(jge_opcode, Box::from(Jcond(false, Ordering::Less)));    // jge
        instrs.insert(jg_opcode, Box::from(Jcond(true,  Ordering::Greater)));  // jg
        instrs.insert(jle_opcode, Box::from(Jcond(false, Ordering::Greater))); // jle
        instrs.insert(je_opcode, Box::from(Jcond(true,  Ordering::Equal)));    // je
        instrs.insert(jne_opcode, Box::from(Jcond(false, Ordering::Equal)));   // jne

        instrs
    };

    pub static ref INSTRUCTION_SIZE: Vec <(Vec<OperandType>, u8)> = {
        let mut sizes: Vec <(Vec<OperandType>, u8)> = Vec::new();

        sizes.push((vec![OperandType::Register, OperandType::Register], 2));

        sizes
    };
}

pub fn get_instruction_size(operand_types: &Vec<OperandType>) -> u8 {
    for entry in (& INSTRUCTION_SIZE).iter() {
        for op in operand_types.iter().enumerate() {
            if * op.1 == entry.0[op.0] {
                return entry.1;
            }
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

// #[derive(Clone, PartialEq, Debug)]
// pub enum Instr {
//     Nop,

//     Add,
//     Sub,
//     Mul,
//     Div,

//     Inc,
//     Dec,

//     Ldr,
//     Str,
//     Put,
//     Mov,

//     Push,
//     Pop,

//     Halt,

//     Cmp,
//     Jmp,
//     Jcond(Ordering),
//     Jncond(Ordering),

//     // Jl, Jge,
//     // Jg, Jle,
//     // Je, Jne,
// }

// lazy_static! {
//     static ref OPCODES: HashMap <u8, (Instr, &'static str, u8, Vec<OperandType>)> = {
//         let mut opcodes = HashMap::new();

//         let opcodes_arr = [
//             (0u8,  (Instr::Nop, "nop", 0, Vec::new())),
            
//             (1u8,  (Instr::Add, "add", 2, vec![OperandType::Register, OperandType::Register])),
//             (2u8,  (Instr::Sub, "sub", 2, vec![OperandType::Register, OperandType::Register])),
//             (3u8,  (Instr::Mul, "mul", 2, vec![OperandType::Register, OperandType::Register])),
//             (4u8,  (Instr::Div, "div", 2, vec![OperandType::Register, OperandType::Register])),

//             (5u8,  (Instr::Inc, "inc", 1, vec![OperandType::Register])),
//             (6u8,  (Instr::Dec, "dec", 1, vec![OperandType::Register])),

//             (7u8,  (Instr::Ldr, "ldr", 2, vec![OperandType::Register, OperandType::Register])),
//             (8u8,  (Instr::Str, "str", 2, vec![OperandType::Register, OperandType::Register])),
//             (9u8,  (Instr::Put, "put", 2, vec![OperandType::Register, OperandType::Value])),
//             (10u8, (Instr::Mov, "mov", 2, vec![OperandType::Register, OperandType::Register])),

//             (11u8, (Instr::Push, "push", 1, vec![OperandType::Register])),
//             (12u8, (Instr::Pop, "pop", 1, vec![OperandType::Register])),

//             (13u8, (Instr::Halt, "halt", 0, Vec::new())),

//             (14u8, (Instr::Cmp, "cmp", 2, vec![OperandType::Register, OperandType::Register])),
//             (15u8, (Instr::Jmp, "jmp", 1, vec![OperandType::Depends])),
//             (16u8, (Instr::Jcond(Ordering::Less), "jl", 1, vec![OperandType::Depends])),
//             (17u8, (Instr::Jncond(Ordering::Less), "jge", 1, vec![OperandType::Depends])),
//             (18u8, (Instr::Jcond(Ordering::Greater), "jg", 1, vec![OperandType::Depends])),
//             (19u8, (Instr::Jncond(Ordering::Greater), "jle", 1, vec![OperandType::Depends])),
//             (20u8, (Instr::Jcond(Ordering::Equal), "je", 1, vec![OperandType::Depends])),
//             (21u8, (Instr::Jncond(Ordering::Equal), "jne", 1, vec![OperandType::Depends])),
//         ];

//         for op in opcodes_arr {
//             opcodes.insert(op.0, op.1);
//         }

//         opcodes
//     };
// }

// impl Instr {
//     pub fn from(instr: u8) -> Self {
//         match OPCODES.get(&instr) {
//             Some(i) => i.0.clone(),
//             None => Instr::Halt
//         }
//     }

//     pub fn from_str(instr: &str) -> Option<Instr> {
//         for entry in OPCODES.iter() {
//             if entry.1.1 == instr {
//                 return Some(entry.1.0.clone());
//             }
//         }

//         None
//     }

//     pub fn to_u8(&self) -> u8 {
//         let mut ret = 13u8; // Default is halt

//         for entry in OPCODES.iter() {
//             if entry.1.0 == *self {
//                 ret = entry.0.clone();
//                 break;
//             }
//         }

//         ret
//     }

//     pub fn get_operand_count(&self) -> u8 {
//         let mut ret = 0u8;

//         for entry in OPCODES.iter() {
//             if entry.1.0 == *self {
//                 ret = entry.1.2;
//                 break;
//             }
//         }
//         ret
//     }
// }
