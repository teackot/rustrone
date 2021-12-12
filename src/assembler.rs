use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::instructions::Instr;

fn get_word(line: &str) -> (&str, bool) {
    let mut word_started = false; // to skip indentations
    let mut word_start_idx = 0_usize;
    let mut word_end_idx = 0_usize;
    let mut eol = false;

    for ch in line.chars().enumerate() {
        word_end_idx += 1;
        if ch.1 == ' ' {
            if word_started {
                break;
            }
        } else if ch.1 == ';' {
            eol = true;
            break;
        } else if ch.1 == ',' {
            break;
        } else {
            if !word_started {
                word_started = true;
                word_start_idx = word_end_idx - 1;
            }
        }
    }

   (& line[word_start_idx..word_end_idx], eol)
}

pub fn assemble(filename: &str) -> Vec<u8> {
    let mut ret = Vec::<u8>::new();

    let path = Path::new(&filename);

    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();
    'lines_loop: for line in lines {
        if let Ok(line) = line {
            let mut chars = line.chars();

            let mut word = String::new();
            
            // Indentation
            loop {
                let ch = if let Some(ch) = chars.next() { ch } else { continue 'lines_loop };
                if ch != ' ' {
                    word.push(ch);
                    break;
                }
            }

            // Instruction
            loop {
                let ch = if let Some(ch) = chars.next() { ch } else { break };
                if ch != ' ' {
                    word.push(ch);
                } else {
                    break;
                }
            }

            let instr = Instr::from_str(&word.to_lowercase());
            let mut byte = instr.to_u8() << 3;

            match instr {
                Instr::Nop |
                Instr::Halt => {
                    ret.push(byte);
                    break;
                },

                Instr::Add |
                Instr::Sub |
                Instr::Mul |
                Instr::Div |
                Instr::Ldr |
                Instr::Str |
                Instr::Mov |
                Instr::Cmp => {
                    for _ in 0..2 {
                        if chars.next() != Some('r') {
                            panic!("Wrong operand type! Should be a register.");
                        }
    
                        if let Some(reg) = chars.next() {
                            match reg.to_digit(10) {
                                Some(reg_id) => byte += reg_id as u8,
                                None => panic!("Register id should be a digit!"),
                            }
                        } else {
                            panic!("You should specify a register!");
                        }

                        ret.push(byte);
                        byte = 0;
                        chars.next(); // skip the space
                    }
                },

                Instr::Put => {
                    if chars.next() != Some('r') {
                        panic!("Wrong operand_L type! Should be a register.");
                    }

                    if let Some(reg) = chars.next() {
                        match reg.to_digit(10) {
                            Some(reg_id) => byte += reg_id as u8,
                            None => panic!("Register id should be a digit!"),
                        }
                    } else {
                        panic!("You should specify a register!");
                    }

                    ret.push(byte);
                    chars.next(); // skip the space

                    // Get the value (second operand)
                    word.clear();
                    loop {
                        let ch = if let Some(ch) = chars.next() { ch } else { break };
                        if ch != ';' {
                            word.push(ch);
                        } else {
                            break;
                        }
                    }
                    ret.push(word.parse::<u8>().expect("Second operand should be an 8 bit unsigned integer!"));
                },

                Instr::Jmp |
                Instr::Jcond(_) |
                Instr::Jncond(_) => {
                    if let Some(ch) = chars.next() {
                        if ch == 'r' { // Jump by register
                            byte += 0b001;
                            ret.push(byte);
                            if let Some(reg) = chars.next() {
                                match reg.to_digit(10) {
                                    Some(reg_id) => ret.push(reg_id as u8),
                                    None => panic!("Register id should be a digit!"),
                                }
                            } else {
                                panic!("You should specify a register!");
                            }
                        } else { // Jump by value
                            byte += 0b000;
                            ret.push(byte);

                            // Get the value (second operand)
                            word.clear();
                            word.push(ch); // Because we already got one char to check the operand type
                            loop {
                                let ch = if let Some(ch) = chars.next() { ch } else { break };
                                if ch != ';' {
                                    word.push(ch);
                                } else {
                                    break;
                                }
                            }
                            ret.push(word.parse::<u8>().expect("Second operand should be an 8 bit unsigned integer!"));
                            // println!("{}", ret.last().unwrap());
                        }
                    } else {
                        panic!();
                    }
                },

                Instr::Inc => todo!(),
                Instr::Dec => todo!(),

                Instr::Push => todo!(),
                Instr::Pop => todo!(),
            }
        }
    }

    return ret;
}
