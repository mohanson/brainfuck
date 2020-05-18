use std::io::prelude::*;

use brainfuck::ir;
use brainfuck::opcode;

struct Interpreter {
    stack: Vec<u8>,
}

impl std::default::Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let opcode_code = opcode::Code::from(data)?;
        let code = ir::Code::from(opcode_code.instrs)?;
        let code_len = code.instrs.len();
        let mut pc: usize = 0;
        let mut ps: usize = 0;
        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                ir::IR::SHL(x) => ps = if ps == 0 { 0 } else { ps - x as usize },
                ir::IR::SHR(x) => {
                    ps += x as usize;
                    if ps >= self.stack.len() {
                        let expand = ps - self.stack.len() + 1;
                        for _ in 0..expand {
                            self.stack.push(0);
                        }
                    }
                }
                ir::IR::ADD(x) => {
                    self.stack[ps] = self.stack[ps].overflowing_add(x).0;
                }
                ir::IR::SUB(x) => {
                    self.stack[ps] = self.stack[ps].overflowing_sub(x).0;
                }
                ir::IR::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[ps]])?;
                }
                ir::IR::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[ps] = buf[0];
                }
                ir::IR::JIZ(x) => {
                    if self.stack[ps] == 0x00 {
                        pc = x as usize;
                    }
                }
                ir::IR::JNZ(x) => {
                    if self.stack[ps] != 0x00 {
                        pc = x as usize;
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 2);
    let mut f = std::fs::File::open(&args[1])?;
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c)?;
    let mut i = Interpreter::default();
    i.run(c)
}
