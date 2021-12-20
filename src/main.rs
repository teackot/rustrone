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
    
    let a = Assembler::new();
    let prg = a.assemble(&fname);
    comp.load_program(prg);

    comp.dump_memory(0..20);
    println!();
    while comp.tick() { println!(); }

    return Ok(());
}
