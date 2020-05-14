use std::io::prelude::*;

mod opcode;

struct Code {
    instrs: Vec<opcode::Opcode>,
    jtable: std::collections::HashMap<usize, usize>,
}

impl Code {
    fn from(data: Vec<u8>) -> Self {
        let dict: Vec<u8> = vec![
            opcode::Opcode::SHL.into(),
            opcode::Opcode::SHR.into(),
            opcode::Opcode::ADD.into(),
            opcode::Opcode::SUB.into(),
            opcode::Opcode::GETCHAR.into(),
            opcode::Opcode::PUTCHAR.into(),
            opcode::Opcode::LB.into(),
            opcode::Opcode::RB.into(),
        ];
        let instrs: Vec<opcode::Opcode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| opcode::Opcode::from(*x))
            .collect();

        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (i, e) in instrs.iter().enumerate() {
            if opcode::Opcode::LB == *e {
                jstack.push(i);
            }
            if opcode::Opcode::RB == *e {
                let j = jstack.pop().unwrap();
                jtable.insert(j, i);
                jtable.insert(i, j);
            }
        }

        Code { instrs, jtable }
    }
}

struct Interpreter {
    stack: Vec<u8>,
}

impl std::default::Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {
    fn run(&mut self, code: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(code);
        let code_len = code.instrs.len();
        let mut pc = 0;
        let mut ps = 0;
        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                opcode::Opcode::SHL => {
                    if ps == 0 {
                        ps = 0
                    } else {
                        ps = ps - 1
                    }
                }
                opcode::Opcode::SHR => {
                    ps += 1;
                    if ps == self.stack.len() {
                        self.stack.push(0)
                    }
                }
                opcode::Opcode::ADD => {
                    self.stack[ps] = self.stack[ps].overflowing_add(1).0;
                }
                opcode::Opcode::SUB => {
                    self.stack[ps] = self.stack[ps].overflowing_sub(1).0;
                }
                opcode::Opcode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[ps]])?;
                }
                opcode::Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[ps] = buf[0];
                }
                opcode::Opcode::LB => {
                    if self.stack[ps] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                opcode::Opcode::RB => {
                    if self.stack[ps] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 2);
    let mut f = std::fs::File::open(&args[1]).unwrap();
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c).unwrap();
    let mut it = Interpreter::default();
    it.run(c).unwrap();
}
