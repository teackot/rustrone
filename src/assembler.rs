mod full_instruction;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::instructions::Instr;

struct Word {
    s: String,
    eol: bool,
    start_idx: usize,
    end_idx: usize,
}

pub struct Assembler {
    assembled: Vec<u8>,
    labels: HashMap<String, usize>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            assembled: Vec::new(),
            labels: HashMap::new(),
        }
    }

    fn get_word(&mut self, line: &str, depth: usize) -> Word {
        let mut word_started = false; // to skip indentations
        let mut start_idx = 0_usize;
        let mut end_idx = 1_usize;
        let mut eol = false;
        
        let mut word_cnt = 0;

        for ch in line.chars().enumerate() {
            end_idx = ch.0 + 1;
            if ch.1 == ' ' {
                if word_started {
                    word_cnt += 1;
                    if word_cnt == depth {
                        break;
                    } else {
                        word_started = false;
                    }
                }
            } else if ch.1 == ';' {
                eol = true;
                break;
            } else if ch.1 == ',' {
                break;
            } else {
                if !word_started {
                    word_started = true;
                    start_idx = ch.0;
                }
            }
        }

        let mut s = String::from(& line[start_idx..(end_idx-1)]);

        if s.starts_with('@') { // It's a label
            if self.labels.contains_key(& s) {
                s = self.labels.get(&s).unwrap().to_string();
                // println!("{}", s);
            } else {
                self.labels.insert(s, self.assembled.len());
                return Word {
                    s: String::from(""),
                    eol: true,
                    start_idx: 0,
                    end_idx: 0,
                };
            }
        }

        Word {
            s,
            eol,
            start_idx,
            end_idx
        }
    }

    fn get_reg_operand(&mut self, word: &mut Word, line: &str, depth: usize) -> u8 {
        *word = self.get_word(line, depth + 1); // Get operand

        if !word.s.starts_with('r') {
            panic!("Wrong operand type! Should be a register.");
        }

        let reg_id: u8 = word.s[1..]
            .parse()
            .expect("Register id should be an integer in range [0; 7]!");
        
        if ! (0..=7).contains(& reg_id) {
            panic!("Register id should be an integer in range [0; 7]!");
        }

        reg_id
    }

    pub fn assemble(mut self, filename: &str) -> Vec<u8> {
        let labels = HashMap::<String, usize>::new();

        self.assembled = Vec::<u8>::new();

        let path = Path::new(&filename);

        let file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let lines = io::BufReader::new(file).lines();
        'lines_loop: for line in lines {
            if let Ok(line) = line {
                let mut word = self.get_word(&line, 1);

                if word.s.len() == 0 {
                    continue; // Skip blank/comment line
                }

                let instr = Instr::from_str(&word.s.to_lowercase()).unwrap();
                let mut byte = instr.to_u8() << 3;

                match instr {
                    Instr::Nop |
                    Instr::Halt => {
                        self.assembled.push(byte);
                    },

                    Instr::Add |
                    Instr::Sub |
                    Instr::Mul |
                    Instr::Div |
                    Instr::Ldr |
                    Instr::Str |
                    Instr::Mov |
                    Instr::Cmp => {
                        for i in 0..2 {
                            let reg_id = self.get_reg_operand(&mut word, &line, i + 1);
                            byte += reg_id;

                            self.assembled.push(byte);
                            byte = 0;
                        }
                    },

                    Instr::Put => {
                        let reg_id = self.get_reg_operand(&mut word, &line, 1);
                        byte += reg_id;

                        self.assembled.push(byte);
                        byte = 0;

                        // TODO: remove duplicated code
                        // Get the value (second operand)
                        word = self.get_word(& line, 3);
                        self.assembled.push(
                            word.s
                                .parse::<u8>()
                                .expect("Second operand should be an 8 bit unsigned integer!")
                        );
                    },

                    Instr::Jmp |
                    Instr::Jcond(_) |
                    Instr::Jncond(_) => {
                        let is_reg = self.get_word(& line, 2).s.starts_with('r');
                        if is_reg { // Jump by register
                            byte += 0b001;
                            self.assembled.push(byte);

                            let reg_id = self.get_reg_operand(&mut word, &line, 1);
                            self.assembled.push(reg_id);
                            byte = 0;
                        } else {// Jump by value    
                            byte += 0b000;
                            self.assembled.push(byte);

                            // TODO: remove duplicated code
                            // Get the value (second operand)
                            word = self.get_word(& line, 3);
                            self.assembled.push(
                                word.s
                                    .parse::<u8>()
                                    .expect("Second operand should be an 8 bit unsigned integer!")
                            );
                        }
                    },

                    Instr::Inc  |
                    Instr::Dec  |
                    Instr::Push |
                    Instr::Pop  => {
                        let reg_id = self.get_reg_operand(&mut word, &line, 1);
                        byte += reg_id;

                        self.assembled.push(byte);
                    },
                }
            }
        }

        self.labels.clear();
        return self.assembled;
    }
}
