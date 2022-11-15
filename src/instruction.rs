use std::fmt;

#[derive(Debug)]
pub struct Instruction {
    pub nnn: u16,
    pub n: u8,
    pub x: u8,
    pub y: u8,
    pub kk: u8
}

impl Instruction {
    pub fn new(op: u16) -> Self {
        Instruction {
            nnn: (op & 0x0FFF),
            kk:  (op & 0x00FF) as u8,
            n:  (op & 0x000F) as u8,
            x:  ((op & 0x0F00) >> 8) as u8,
            y:  ((op & 0x00F0) >> 4) as u8,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[NNN: 0x{:0x}, KK: 0x{:0x}, N: 0x{:0x}, X: 0x{:0x}, Y: 0x{:0x}]", self.nnn, self.kk, self.n, self.x, self.y)
    }
}