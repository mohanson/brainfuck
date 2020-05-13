use std::collections;
use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;

mod opcode;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    Syntax,
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(err) => return write!(f, "{}", err),
            Error::Syntax => return write!(f, "Syntax"),
        };
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
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
    fn cls(&self, code: Vec<u8>) -> Vec<u8> {
        code.into_iter()
            .filter(|x| opcode::Opcode::from_u8(*x).is_some())
            .collect()
    }

    #[allow(clippy::needless_pass_by_value)]
    fn bap(&self, code: Vec<u8>) -> Result<collections::HashMap<usize, usize>, Error> {
        let mut temp: Vec<usize> = Vec::new();
        let mut bmap: collections::HashMap<usize, usize> = collections::HashMap::new();
        for (i, e) in code.iter().enumerate() {
            if &opcode::Opcode::LB.into_u8() == e {
                temp.push(i);
            }
            if &opcode::Opcode::RB.into_u8() == e {
                let j = temp.pop().ok_or(Error::Syntax)?;
                bmap.insert(j, i);
                bmap.insert(i, j);
            }
        }
        Ok(bmap)
    }

    fn run(&mut self, code: Vec<u8>) -> Result<(), Error> {
        let code = self.cls(code);
        let bmap = self.bap(code.clone())?;

        let mut pc = 0;
        let mut ps = 0;
        loop {
            if pc >= code.len() {
                break;
            }
            let r = opcode::Opcode::from_u8(code[pc]);
            if let Some(op) = r {
                match op {
                    opcode::Opcode::SHL => ps = if ps == 0 { 0 } else { ps - 1 },
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
                        io::stdout().write_all(&[self.stack[ps]])?;
                    }
                    opcode::Opcode::GETCHAR => {
                        let mut buf: Vec<u8> = vec![0; 1];
                        io::stdin().read_exact(&mut buf)?;
                        self.stack[ps] = buf[0];
                    }
                    opcode::Opcode::LB => {
                        if self.stack[ps] == 0x00 {
                            pc = bmap[&pc];
                        }
                    }
                    opcode::Opcode::RB => {
                        if self.stack[ps] != 0x00 {
                            pc = bmap[&pc];
                        }
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);
    let mut f = fs::File::open(&args[1]).unwrap();
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c).unwrap();
    let mut it = Interpreter::default();
    it.run(c).unwrap();
}
