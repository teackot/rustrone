use std::cmp::Ordering;
use phf::phf_ordered_map;

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

#[allow(non_upper_case_globals)]
static opcodes: phf::OrderedMap<u8, (Instr, &'static str, u8)> = phf_ordered_map! {
    0u8 => (Instr::Nop, "nop", 0),

    1u8 => (Instr::Add, "add", 2),
    2u8 => (Instr::Sub, "sub", 2),
    3u8 => (Instr::Mul, "mul", 2),
    4u8 => (Instr::Div, "div", 2),

    5u8 => (Instr::Inc, "inc", 1),
    6u8 => (Instr::Dec, "dec", 1),

    7u8 => (Instr::Ldr, "ldr", 2),
    8u8 => (Instr::Str, "str", 2),
    9u8 => (Instr::Put, "put", 2),
    10u8 => (Instr::Mov, "mov", 2),

    11u8 => (Instr::Push, "push", 1),
    12u8 => (Instr::Pop, "pop", 1),

    13u8 => (Instr::Halt, "halt", 0),

    14u8 => (Instr::Cmp, "cmp", 2),
    15u8 => (Instr::Jmp, "jmp", 1),
    16u8 => (Instr::Jcond(Ordering::Less), "jl", 1),
    17u8 => (Instr::Jncond(Ordering::Less), "jge", 1),
    18u8 => (Instr::Jcond(Ordering::Greater), "jg", 1),
    19u8 => (Instr::Jncond(Ordering::Greater), "jle", 1),
    20u8 => (Instr::Jcond(Ordering::Equal), "je", 1),
    21u8 => (Instr::Jncond(Ordering::Equal), "jne", 1),
};

impl Instr {
    pub fn from(instr: u8) -> Self {
        match opcodes.get(&instr) {
            Some(i) => i.0.clone(),
            None => Instr::Halt
        }
    }

    pub fn from_str(instr: &str) -> Option<Instr> {
        for entry in opcodes.entries() {
            if entry.1.1 == instr {
                return Some(entry.1.0.clone());
            }
        }

        None
    }

    pub fn to_u8(&self) -> u8 {
        let mut ret = 13u8;
        for entry in opcodes.entries() {
            if entry.1.0 == *self {
                ret = entry.0.clone();
                break;
            }
        }

        ret
    }

    pub fn get_operand_count(&self) -> u8 {
        let mut ret = 0u8;
        for entry in opcodes.entries() {
            if entry.1.0 == *self {
                ret = entry.1.2;
                break;
            }
        }

        ret
    }
}
