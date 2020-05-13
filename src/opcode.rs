#[derive(PartialEq)]
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

impl Opcode {
    pub fn from_u8(u: u8) -> Option<Self> {
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

    pub fn into_u8(self) -> u8 {
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
