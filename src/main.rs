use std::collections;
use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;

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

#[derive(PartialEq)]
enum Opcode {
    SHR = 0x3E,
    SHL = 0x3C,
    ADD = 0x2B,
    SUB = 0x2D,
    PUTCHAR = 0x2E,
    GETCHAR = 0x2C,
    LB = 0x5B,
    RB = 0x5D,
}

impl Opcode {
    fn from_u8(u: u8) -> Option<Self> {
        match u {
            0x3E => Some(Opcode::SHR),
            0x3C => Some(Opcode::SHL),
            0x2B => Some(Opcode::ADD),
            0x2D => Some(Opcode::SUB),
            0x2E => Some(Opcode::PUTCHAR),
            0x2C => Some(Opcode::GETCHAR),
            0x5B => Some(Opcode::LB),
            0x5D => Some(Opcode::RB),
            _ => None,
        }
    }

    fn into_u8(self) -> u8 {
        match self {
            Opcode::SHR => 0x3E,
            Opcode::SHL => 0x3C,
            Opcode::ADD => 0x2B,
            Opcode::SUB => 0x2D,
            Opcode::PUTCHAR => 0x2E,
            Opcode::GETCHAR => 0x2C,
            Opcode::LB => 0x5B,
            Opcode::RB => 0x5D,
        }
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
            .filter(|x| Opcode::from_u8(*x).is_some())
            .collect()
    }

    #[allow(clippy::needless_pass_by_value)]
    fn bap(&self, code: Vec<u8>) -> Result<collections::HashMap<usize, usize>, Error> {
        let mut temp: Vec<usize> = Vec::new();
        let mut bmap: collections::HashMap<usize, usize> = collections::HashMap::new();
        for (i, e) in code.iter().enumerate() {
            if &Opcode::LB.into_u8() == e {
                temp.push(i);
            }
            if &Opcode::RB.into_u8() == e {
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
            let r = Opcode::from_u8(code[pc]);
            if let Some(op) = r {
                match op {
                    Opcode::SHL => ps = if ps == 0 { 0 } else { ps - 1 },
                    Opcode::SHR => {
                        ps += 1;
                        if ps == self.stack.len() {
                            self.stack.push(0)
                        }
                    }
                    Opcode::ADD => {
                        self.stack[ps] = self.stack[ps].overflowing_add(1).0;
                    }
                    Opcode::SUB => {
                        self.stack[ps] = self.stack[ps].overflowing_sub(1).0;
                    }
                    Opcode::PUTCHAR => {
                        io::stdout().write_all(&[self.stack[ps]])?;
                    }
                    Opcode::GETCHAR => {
                        let mut buf: Vec<u8> = vec![0; 1];
                        io::stdin().read_exact(&mut buf)?;
                        self.stack[ps] = buf[0];
                    }
                    Opcode::LB => {
                        if self.stack[ps] == 0x00 {
                            pc = bmap[&pc];
                        }
                    }
                    Opcode::RB => {
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
