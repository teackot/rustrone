mod full_instruction;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use self::full_instruction::FullInstruction;

pub struct Assembler {
    assembled: Vec<u8>,
    labels: HashMap<String, usize>,
    instrs: Vec<FullInstruction>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            assembled: Vec::new(),
            labels: HashMap::new(),
            instrs: Vec::new(),
        }
    }

    pub fn assemble(mut self, filename: &str) -> Vec<u8> {
        self.assembled = Vec::<u8>::new();

        let path = Path::new(&filename);

        let file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let lines = io::BufReader::new(file).lines();
        let mut current_byte: usize = 0;
        for line in lines {
            if let Ok(line) = line {
                let instr = FullInstruction::new(&line);

                match instr.as_label() {
                    Some(s) => {
                        self.labels.insert(s, current_byte);
                    },
                    None => {
                        current_byte += instr.size()
                    },
                }

                self.instrs.push(instr);
            }
        }
        
        self.assembled.reserve(current_byte);
        for mut instr in self.instrs {
            self.assembled.append(&mut instr.build(& self.labels));
        }

        self.labels.clear();
        return self.assembled;
    }
}
