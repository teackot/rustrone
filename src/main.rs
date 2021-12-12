use computer::Computer;
use assembler::assemble;

mod computer;
mod instructions;
mod assembler;

fn main() {
    let mut comp = Computer::new(100);
    // put a, 5; put c, 2; sub a, c;
    // put b, 3; cmp a, b; je 4;
    // halt
    // comp.load_program(vec![
    //     0b01001000, 5, 0b01001010, 2, 0b00010000, 0b010,
    //     0b01001001, 3, 0b01110000, 0b001, 0b10100000, 4,
    //     0b01101000,
    // ]);
    
    comp.load_program(assemble("program.s"));

    while comp.tick() { println!(); }
}
