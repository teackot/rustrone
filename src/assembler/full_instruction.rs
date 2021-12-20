use std::collections::HashMap;

use crate::instructions::{OperandType, instr_from_str, Executable, get_instruction_size};

enum InstructionWord {
    Instruction(&'static Box<dyn Executable + Sync + 'static>),
    Label(String),
    None,
}

pub struct FullInstruction {
    instruction: InstructionWord,
    operands: Vec<String>,
    operand_types: Vec<OperandType>,
    size: usize,
}

impl FullInstruction {
    fn get_words(line: &str) -> Vec<String> {
        let mut word_started = false; // to skip indentations
        let mut start_idx = 0_usize;
        let mut end_idx = 1_usize;

        let mut words: Vec<String> = Vec::new();
        words.reserve(3);

        for ch in line.chars().enumerate() {
            end_idx = ch.0 + 1;
            if (ch.1 == ' ') || (ch.1 == ';') {
                if word_started { // We found a word
                    word_started = false;
                    let s = String::from(& line[start_idx..(end_idx-1)]);
                    words.push(s);

                    if ch.1 == ';' {
                        break;
                    }
                }
            } else if !word_started {
                word_started = true;
                start_idx = ch.0;
            }
        }

        words
    }

    pub fn new(line: &str) -> Self {
        let words = Self::get_words(line);

        if words.len() == 0 { // blank line or comment
            return Self {
                instruction: InstructionWord::None,
                operands: Vec::new(),
                operand_types: Vec::new(),
                size: 0,
            }
        }

        let instruction = match instr_from_str(&words[0]) {
            Some(instr) => InstructionWord::Instruction(instr),
            None => InstructionWord::Label(words[0].clone()),
        };

        let operands = if words.len() > 1 {
            words[1..].to_vec()
        } else {
            Vec::new()
        };

        let mut operand_types: Vec<OperandType> = Vec::new();
        operand_types.reserve_exact(operands.len());
        for op in & operands {
            if op.starts_with('r') {
                operand_types.push(OperandType::Register);
            } else {
                operand_types.push(OperandType::Value);
            }
        }

        // Calculate total size in bytes
        let size: usize = if let InstructionWord::Instruction(instr) = instruction {
            get_instruction_size(& operand_types) as usize
        } else {
            0usize
        };

        Self {
            instruction,
            operands,
            operand_types,
            size,
        }
    }

    pub fn size(&self) -> usize { self.size }

    pub fn as_label(&self) -> Option<String> {
        match &self.instruction {
            InstructionWord::Label(lbl) => Some(lbl.clone()),
            _ => None,
        }
    }

    pub fn build(&mut self, labels: &HashMap<String, usize>) -> Vec<u8> {
        // TODO: compile labels
        for op in &mut self.operands {
            if op.starts_with('@') { // It is a label
                *op = labels.get(op).expect("There is no such label!").to_string();
            }
        }

        let mut ret = Vec::<u8>::new();

        if let InstructionWord::Instruction(instr) = & self.instruction {
            ret.reserve_exact(self.size);
            ret = instr.assemble(& self.operands, & self.operand_types);
        }

        ret
    }
}
