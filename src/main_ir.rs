use std::io::prelude::*;

mod ir;
mod opcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 2);
    let mut f = std::fs::File::open(&args[1])?;
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c)?;

    let opcode_code = opcode::Code::from(c)?;
    let ir_code = ir::Code::from(opcode_code.instrs);
    println!("{:?}", ir_code);
    Ok(())
}
