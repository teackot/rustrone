use std::collections::HashMap;

use crate::instructions::{Instr, self, OperandType};

enum InstructionWord {
    Instruction(Instr),
    Label(String),
    None,
}

pub struct FullInstruction {
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

        if words.len() == 0 { // blank line / comment
            return Self {
                instruction: InstructionWord::None,
                operands: Vec::new(),
                size: 0,
            }
        }

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
                OperandType::Value => 3, // 1 byte + 1 reg + u8 value
                OperandType::Depends => panic!("Error, operand types must be definite"),
            }
        } else if operands.len() == 0 {
            0
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
            _ => None,
        }
    }

    pub fn build(&self, labels: &HashMap<String, usize>) -> Vec<u8> {
        let mut ret = Vec::<u8>::new();

        if let InstructionWord::Instruction(instr) = & self.instruction {
            ret.reserve_exact(self.size);
            let mut byte: u8 = instr.to_u8() << 2;

            match instr {
                Instr::Nop |
                Instr::Halt => {
                    ret.push(byte);
                },

                Instr::Add |
                Instr::Sub |
                Instr::Mul |
                Instr::Div |
                Instr::Ldr |
                Instr::Str |
                Instr::Mov |
                Instr::Cmp => {
                    ret.push(byte);

                    let mut registers_byte = 0u8;
                    let mut value_byte = 0u8;
                    for op in (&self.operands).into_iter().enumerate() {
                        let (i, (op, op_type)) = op;
                        match op_type {
                            OperandType::Register => {
                                let reg_id: u8 = op[1..].parse().expect("Invalid register id");
                                if ! (0..=7).contains(& reg_id) {
                                    panic!("Register id is out of range");
                                }

                                registers_byte += reg_id << (4 - 4 * i);
                            }
                            OperandType::Value => {
                                let value: u8 = op[1..].parse().expect("Invalid value");
                                value_byte = value;
                            }
                            OperandType::Depends => panic!("Error, operand types must be definite"),                            
                        }
                    }

                    ret.push(registers_byte);
                    ret.push(value_byte);
                },

                _ => { todo!() }

                // Instr::Put => {
                //     let reg_id = self.get_reg_operand(&mut word, &line, 1);
                //     byte += reg_id;

                //     self.assembled.push(byte);
                //     byte = 0;

                //     // TODO: remove duplicated code
                //     // Get the value (second operand)
                //     word = self.get_word(& line, 3);
                //     self.assembled.push(
                //         word.s
                //             .parse::<u8>()
                //             .expect("Second operand should be an 8 bit unsigned integer!")
                //     );
                // },

                // Instr::Jmp |
                // Instr::Jcond(_) |
                // Instr::Jncond(_) => {
                //     let is_reg = self.get_word(& line, 2).s.starts_with('r');
                //     if is_reg { // Jump by register
                //         byte += 0b001;
                //         self.assembled.push(byte);

                //         let reg_id = self.get_reg_operand(&mut word, &line, 1);
                //         self.assembled.push(reg_id);
                //         byte = 0;
                //     } else {// Jump by value    
                //         byte += 0b000;
                //         self.assembled.push(byte);

                //         // TODO: remove duplicated code
                //         // Get the value (second operand)
                //         word = self.get_word(& line, 3);
                //         self.assembled.push(
                //             word.s
                //                 .parse::<u8>()
                //                 .expect("Second operand should be an 8 bit unsigned integer!")
                //         );
                //     }
                // },

                // Instr::Inc  |
                // Instr::Dec  |
                // Instr::Push |
                // Instr::Pop  => {
                //     let reg_id = self.get_reg_operand(&mut word, &line, 1);
                //     byte += reg_id;

                //     self.assembled.push(byte);
                // },
            }
        }

        ret
    }
}
