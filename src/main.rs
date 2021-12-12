use computer::Computer;
use assembler::Assembler;

use std::env;

mod computer;
mod instructions;
mod assembler;

fn print_usage() {
    println!("Usage:\trustrone [file]");
    println!("\tfile - file with source code");
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage();
        return Err("invalid arguments");
    }

    let fname = args[1].clone();

    let mut comp = Computer::new(256);
    // put a, 5; put c, 2; sub a, c;
    // put b, 3; cmp a, b; je 4;
    // halt
    // comp.load_program(vec![
    //     0b01001000, 5, 0b01001010, 2, 0b00010000, 0b010,
    //     0b01001001, 3, 0b01110000, 0b001, 0b10100000, 4,
    //     0b01101000,
    // ]);
    
    let a = Assembler::new();
    comp.load_program(a.assemble(&fname));

    while comp.tick() { println!(); }

    return Ok(());
}
