use std::io;
use std::io::prelude::*;

fn main() {
    let mut src: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut src).unwrap();
    let mut pc = 0;
    loop {
        if pc >= src.len() {
            break;
        }
        let op = src.get(pc).unwrap();
        pc += 1;
        println!("{}", op);
    }
}
