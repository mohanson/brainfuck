#[derive(Debug, PartialEq)]
pub enum Opcode {
    SHR = 0x3E,
    SHL = 0x3C,
    ADD = 0x2B,
    SUB = 0x2D,
    PUTCHAR = 0x2E,
    GETCHAR = 0x2C,
    LB = 0x5B,
    RB = 0x5D,
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => panic!(),
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
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

pub fn from(code: Vec<u8>) -> Vec<Opcode> {
    let dict: Vec<u8> = vec![
        Opcode::SHL.into(),
        Opcode::SHR.into(),
        Opcode::ADD.into(),
        Opcode::SUB.into(),
        Opcode::GETCHAR.into(),
        Opcode::PUTCHAR.into(),
        Opcode::LB.into(),
        Opcode::RB.into(),
    ];
    code.iter()
        .filter(|x| dict.contains(x))
        .map(|x| Opcode::from(*x))
        .collect()
}
