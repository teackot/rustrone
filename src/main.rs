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
    // let prg: Vec<u8> = vec![
    //     9 << 2, 2 << 4, 3,
    //     1 << 2, 1,
    //     15 << 2, 2 << 4,
    // ];
    comp.load_program(prg);

    // comp.tick();comp.tick();comp.tick();comp.tick();comp.tick();comp.tick();comp.tick();
    while comp.tick() { println!(); }

    return Ok(());
}
