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