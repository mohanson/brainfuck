use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    InvalidOpcode,
    OutOfStack,
    Unknown,
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(err) => return write!(f, "{}", err),
            Error::InvalidOpcode => return write!(f, "InvalidOpcode"),
            Error::OutOfStack => return write!(f, "OutOfStack"),
            Error::Unknown => return write!(f, "Unknown"),
        };
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    fn from(u: u8) -> Option<Self> {
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
}

struct Interpreter {
    stack: Vec<u8>,
}

impl std::default::Default for Interpreter {
    fn default() -> Self {
        Self {
            stack: vec![0; 1024],
        }
    }
}

impl Interpreter {
    fn exec(&mut self, code: Vec<u8>) -> Result<(), Error> {
        let mut pc = 0;
        let mut ps = 0;
        loop {
            if pc >= code.len() {
                break;
            }
            let r = Opcode::from(code[pc]);
            if r.is_none() {
                return Err(Error::InvalidOpcode);
            }
            match r.unwrap() {
                Opcode::SHL => {
                    if ps == 0 {
                        return Err(Error::OutOfStack);
                    }
                    ps -= 1;
                }
                Opcode::SHR => {
                    if ps >= self.stack.len() - 1 {
                        return Err(Error::OutOfStack);
                    }
                    ps += 1
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
                    self.stack[ps] = buf[1];
                }
                Opcode::LB => {
                    if self.stack[ps] == 0x00 {
                        loop {
                            pc += 1;
                            if code[pc] == Opcode::RB as u8 {
                                break;
                            }
                        }
                    }
                }
                Opcode::RB => {
                    if self.stack[ps] != 0x00 {
                        loop {
                            pc -= 1;
                            if code[pc] == Opcode::LB as u8 {
                                break;
                            }
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
    let f = fs::File::open(&args[1]).unwrap();
    let reader = BufReader::new(f);
    let mut buf: Vec<u8> = Vec::new();
    for line in reader.lines() {
        let mut line: Vec<u8> = Vec::from(line.unwrap());
        buf.append(&mut line);
    }
    let mut it = Interpreter::default();
    it.exec(buf).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        let mut it = Interpreter::default();
        it.exec(Vec::from("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.".as_bytes())).unwrap();
    }
}
