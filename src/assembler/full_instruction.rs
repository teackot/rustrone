use std::collections::HashMap;

use crate::instructions::{self, OperandType, instr_from_str, Executable, get_instruction_size};

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
                    let mut s = String::from(& line[start_idx..(end_idx-1)]);
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

        if words.len() == 0 { // blank line / comment
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

    pub fn set_position() {

    }

    pub fn size(&self) -> usize { self.size }

    pub fn is_label(&self) -> bool {
        matches!(self.instruction, InstructionWord::Label(_))
    }

    pub fn as_label(&self) -> Option<String> {
        match &self.instruction {
            InstructionWord::Label(lbl) => Some(lbl.clone()),
            _ => None,
        }
    }

    pub fn build(&self, labels: &HashMap<String, usize>) -> Vec<u8> {
        // TODO: compile labels
        let mut ret = Vec::<u8>::new();

        if let InstructionWord::Instruction(instr) = & self.instruction {
            ret.reserve_exact(self.size);
            ret = instr.assemble(& self.operands, & self.operand_types);

            // match instr {
            //     Instr::Nop |
            //     Instr::Halt => {
            //         ret.push(byte);
            //     },

            //     Instr::Add |
            //     Instr::Sub |
            //     Instr::Mul |
            //     Instr::Div |
            //     Instr::Ldr |
            //     Instr::Str |
            //     Instr::Mov |
            //     Instr::Cmp => {
            //         ret.push(byte);

            //         let mut registers_byte = 0u8;
            //         let mut value_byte = 0u8;
            //         for op in (&self.operands).into_iter().enumerate() {
            //             let (i, (op, op_type)) = op;
            //             match op_type {
            //                 OperandType::Register => {
            //                     let reg_id: u8 = op[1..].parse().expect("Invalid register id");
            //                     if ! (0..=7).contains(& reg_id) {
            //                         panic!("Register id is out of range");
            //                     }

            //                     registers_byte += reg_id << (4 - 4 * i);
            //                 }
            //                 OperandType::Value => {
            //                     let value: u8 = op[1..].parse().expect("Invalid value");
            //                     value_byte = value;
            //                 }
            //                 OperandType::Depends => panic!("Error, operand types must be definite"),                            
            //             }
            //         }

            //         ret.push(registers_byte);
            //         ret.push(value_byte);
            //     },

            //     _ => { todo!() }

            //     // Instr::Put => {
            //     //     let reg_id = self.get_reg_operand(&mut word, &line, 1);
            //     //     byte += reg_id;

            //     //     self.assembled.push(byte);
            //     //     byte = 0;

            //     //     // TODO: remove duplicated code
            //     //     // Get the value (second operand)
            //     //     word = self.get_word(& line, 3);
            //     //     self.assembled.push(
            //     //         word.s
            //     //             .parse::<u8>()
            //     //             .expect("Second operand should be an 8 bit unsigned integer!")
            //     //     );
            //     // },

            //     // Instr::Jmp |
            //     // Instr::Jcond(_) |
            //     // Instr::Jncond(_) => {
            //     //     let is_reg = self.get_word(& line, 2).s.starts_with('r');
            //     //     if is_reg { // Jump by register
            //     //         byte += 0b001;
            //     //         self.assembled.push(byte);

            //     //         let reg_id = self.get_reg_operand(&mut word, &line, 1);
            //     //         self.assembled.push(reg_id);
            //     //         byte = 0;
            //     //     } else {// Jump by value    
            //     //         byte += 0b000;
            //     //         self.assembled.push(byte);

            //     //         // TODO: remove duplicated code
            //     //         // Get the value (second operand)
            //     //         word = self.get_word(& line, 3);
            //     //         self.assembled.push(
            //     //             word.s
            //     //                 .parse::<u8>()
            //     //                 .expect("Second operand should be an 8 bit unsigned integer!")
            //     //         );
            //     //     }
            //     // },

            //     // Instr::Inc  |
            //     // Instr::Dec  |
            //     // Instr::Push |
            //     // Instr::Pop  => {
            //     //     let reg_id = self.get_reg_operand(&mut word, &line, 1);
            //     //     byte += reg_id;

            //     //     self.assembled.push(byte);
            //     // },
            // }
        }

        ret
    }
}
