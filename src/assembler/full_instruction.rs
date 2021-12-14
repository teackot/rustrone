use std::collections::HashMap;

use crate::instructions::{Instr, self};

enum OperandType {
    Register,
    Value,
}

enum InstructionWord {
    Instruction(Instr),
    Label(String),
}

struct FullInstruction {
    instruction: InstructionWord,
    operands: Vec<(String, OperandType)>,
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
                    let mut s = String::from(& line[start_idx..(end_idx-1)]);
                    words.push(s);

                    if ch.1 == ';' {
                        break;
                    }

                    // if s.starts_with('@') { // It's a label
                    //     if labels.contains_key(& s) {
                    //         s = labels.get(&s).unwrap().to_string();
                    //         // println!("{}", s);
                    //     } else {
                    //         labels.insert(s, self.assembled.len());
                    //         return Word {
                    //             s: String::from(""),
                    //             eol: true,
                    //             start_idx: 0,
                    //             end_idx: 0,
                    //         };
                    //     }
                    // }
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

        let instruction = match Instr::from_str(&words[0]) {
            Some(instr) => InstructionWord::Instruction(instr),
            None => InstructionWord::Label(words[0].clone()),
        };

        let operands_strings = if words.len() > 1 {
            words[1..].to_vec()
        } else {
            Vec::new()
        };

        let mut operands: Vec<(String, OperandType)> = Vec::new();
        operands.reserve_exact(operands_strings.len());
        for op in operands_strings {
            if op.starts_with('r') {
                operands.push((op, OperandType::Register));
            } else {
                operands.push((op, OperandType::Value));
            }
        }

        // Calculate total size in bytes
        let size: usize = if operands.len() == 1 {
            2
        } else if operands.len() == 2 {
            match operands.last().unwrap().1 {
                OperandType::Register => 2, // 1 byte + 2 regs in 1 byte
                OperandType::Value => 3 // 1 byte + 1 reg + u8 value
            }
        } else {
            panic!("There is no instruction with this number of operands");
        };

        Self {
            instruction,
            operands,
            size,
        }
    }

    pub fn set_position() {

    }

    pub fn size(&self) -> usize { self.size }

    pub fn is_label(&self) -> bool {
        matches!(self.instruction, InstructionWord::Label(_))
    }

    pub fn as_label(&self) -> Option<String> {
        match &self.instruction {
            InstructionWord::Label(lbl) => Some(lbl.clone()),
            InstructionWord::Instruction(_) => None,
        }
    }
}
