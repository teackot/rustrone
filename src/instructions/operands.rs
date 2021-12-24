use crate::computer::Computer;

pub fn get_next_reg_reg_operands(computer: &mut Computer) -> (usize, usize) {
    let regs_byte = computer.next_byte();
    let reg1 = ((regs_byte & 0b11110000) >> 4) as usize;
    let reg2 = (regs_byte & 0b00001111) as usize;

    (reg1, reg2)
}

pub fn get_next_reg_operand(computer: &mut Computer) -> usize {
    let regs_byte = computer.next_byte();
    let reg = ((regs_byte & 0b11110000) >> 4) as usize;

    reg
}

#[derive(PartialEq, Debug)]
pub enum OperandType {
    Register,
    Value,
}
