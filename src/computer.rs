use std::{cmp::Ordering, usize, ops::Range};

use crate::instructions::{INSTRUCTIONS};

pub struct Regs {
    pub common: [u8; 4],
    pub flags: Ordering,
}

pub struct Computer {
    pub memory: Vec<u8>,
    pub regs: Regs,
    pub ip: u8,
    pub should_halt: bool,
}

// Other
impl Computer {
    pub fn new(mem_size: usize) -> Self {
        Self {
            memory: vec![0; mem_size],
            regs: Regs {
                common: [0, 0, 0, 0],
                flags: Ordering::Equal,
            },
            ip: 0,
            should_halt: false,
        }
    }

    pub fn load_program(&mut self, prg: Vec<u8>) {
        for i in 0..prg.len() {
            self.memory[i] = prg[i];
        }
    }

    pub fn dump(&self) {
        for reg in self.regs.common.iter().enumerate() {
            println!("r{}: {}", reg.0, reg.1);
        }

        println!("ip: {}\n", self.ip);
    }

    pub fn dump_memory(&self, r: Range<usize>) {
        for i in r {
            print!("{} | ", self.memory[i]);
        }
        println!();
    }

    pub fn next_byte(&mut self) -> u8 {
        let ret = self.memory[self.ip as usize];
        self.ip += 1;
        ret
    }

    pub fn tick(&mut self) -> bool {
        let byte = self.next_byte();
        let instr = (byte & 0b11111100) >> 2;
        INSTRUCTIONS.get(&instr).unwrap().execute(self, byte);

        self.dump();

        ! self.should_halt
    }
}
