use super::opcode;

#[derive(Debug, PartialEq)]
pub enum IR {
    SHR(u32),
    SHL(u32),
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    JIZ(u32),
    JNZ(u32),
}

#[derive(Debug)]
pub struct Code {
    pub instrs: Vec<IR>,
}

impl Code {
    pub fn from(data: Vec<opcode::Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instrs: Vec<IR> = Vec::new();
        let mut jstack: Vec<u32> = Vec::new();
        for e in data {
            match e {
                opcode::Opcode::SHR => match instrs.last_mut() {
                    Some(IR::SHR(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instrs.push(IR::SHR(1));
                    }
                },
                opcode::Opcode::SHL => match instrs.last_mut() {
                    Some(IR::SHL(x)) => {
                        *x += 1;
                    }
                    _ => {
                        instrs.push(IR::SHL(1));
                    }
                },
                opcode::Opcode::ADD => match instrs.last_mut() {
                    Some(IR::ADD(x)) => {
                        let (b, _) = x.overflowing_add(1);
                        *x = b;
                    }
                    _ => {
                        instrs.push(IR::ADD(1));
                    }
                },
                opcode::Opcode::SUB => match instrs.last_mut() {
                    Some(IR::SUB(x)) => {
                        let (b, _) = x.overflowing_add(1);
                        *x = b;
                    }
                    _ => {
                        instrs.push(IR::SUB(1));
                    }
                },
                opcode::Opcode::GETCHAR => {
                    instrs.push(IR::GETCHAR);
                }
                opcode::Opcode::PUTCHAR => {
                    instrs.push(IR::PUTCHAR);
                }
                opcode::Opcode::LB => {
                    instrs.push(IR::JIZ(0));
                    jstack.push((instrs.len() - 1) as u32);
                }
                opcode::Opcode::RB => {
                    let j = jstack.pop().ok_or("pop from empty list")?;
                    instrs.push(IR::JNZ(j));
                    let instrs_len = instrs.len();
                    match &mut instrs[j as usize] {
                        IR::JIZ(x) => {
                            *x = (instrs_len - 1) as u32;
                        }
                        _ => {
                            unimplemented!();
                        }
                    }
                }
            }
        }
        Ok(Code { instrs })
    }
}
