use std::cmp::Ordering;

use crate::instructions::Instr;

struct Regs {
    pub common: [u8; 4],
    pub flags: Ordering,
}

pub struct Computer {
    memory: Vec<u8>,
    regs: Regs,
    pub ip: u8,
}

impl Computer {
    fn get_reg_reg_ops(&mut self, start_byte: u8) -> (usize, usize) {
        let reg1_id = (start_byte & 0b111) as usize;
        let reg2_id = self.next_byte() as usize;

        (reg1_id, reg2_id)
    }
}

// Instruction execution
impl Computer {
    fn execute_reg_reg(&mut self, instr: Instr, start_byte: u8) {
        let reg1 = (start_byte & 0b111) as usize;
        let reg2 = self.next_byte() as usize;

        match instr {
            Instr::Add => self.regs.common[reg1] += self.regs.common[reg2],
            Instr::Sub => self.regs.common[reg1] -= self.regs.common[reg2],
            Instr::Mul => self.regs.common[reg1] *= self.regs.common[reg2],
            Instr::Div => {
                let value1 = self.regs.common[reg1];
                let value2 = self.regs.common[reg2];

                let div = value1 / value2;
                let rem = value1 % value2;

                self.regs.common[0] = div;
                self.regs.common[1] = rem;
            },

            Instr::Ldr => {
                self.regs.common[reg1] = self.memory[self.regs.common[reg2] as usize];
            },
            Instr::Str => {
                self.memory[self.regs.common[reg2] as usize] = self.regs.common[reg1];
            },
            Instr::Mov => {
                self.regs.common[reg1] = self.regs.common[reg2];
            },

            Instr::Cmp => {
                self.regs.flags = self.regs.common[reg1].cmp(& self.regs.common[reg2]);
            },

            _ => panic!(),
        }
    }

    fn execute_reg_val(&mut self, instr: Instr, start_byte: u8) {
        let reg = (start_byte & 0b111) as usize;
        let val = self.next_byte();

        match instr {
            Instr::Put => self.regs.common[reg] = val,

            _ => panic!(),
        }
    }

    fn execute_jump(&mut self, instr: Instr, start_byte: u8) {
        // 0 - value
        // 1 - reg
        let operand_type = (start_byte & 0b111) != 0; // != 0 casts to bool
        let byte = self.next_byte();

        let destination = match operand_type {
            false => byte, // value
            true => self.regs.common[byte as usize], // reg
        };

        match instr {
            Instr::Jmp => {
                self.ip = destination;
            },
            Instr::Jcond(ord) => {
                if self.regs.flags == ord {
                    self.ip = destination;
                }
            },
            Instr::Jncond(ord) => {
                if self.regs.flags != ord {
                    self.ip = destination;
                }
            },

            _ => panic!("Invalid jump instruction")
        }
    }
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

        println!("ip: {}", self.ip);
    }

    fn next_byte(&mut self) -> u8 {
        let ret = self.memory[self.ip as usize];
        self.ip += 1;
        ret
    }

    pub fn tick(&mut self) -> bool {
        let mut continue_work = true;

        let byte = self.next_byte();
        let instr = Instr::from((byte & 0b11111000) >> 3);

        match instr {
            Instr::Nop => (),

            Instr::Add |
            Instr::Sub |
            Instr::Mul |
            Instr::Div |
            Instr::Ldr |
            Instr::Str |
            Instr::Mov |
            Instr::Cmp => self.execute_reg_reg(instr, byte),

            Instr::Put => self.execute_reg_val(instr, byte),

            Instr::Jmp |
            Instr::Jcond(_) |
            Instr::Jncond(_) => self.execute_jump(instr, byte),

            Instr::Inc => todo!(),
            Instr::Dec => todo!(),

            Instr::Push => todo!(),
            Instr::Pop => todo!(),

            Instr::Halt => continue_work = false,
        }

        self.dump();

        continue_work
    }
}
