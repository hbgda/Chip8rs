use crate::display::{DISPLAY_WIDTH, DISPLAY_HEIGHT, DISPLAY_SIZE};
use crate::instruction::Instruction;
use crate::quirk::Quirk;
use rand::random;


pub const FONT_SET: [u8; 16 * 5] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,   // 0
    0x20, 0x60, 0x20, 0x20, 0x70,   // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0,   // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0,   // 3
    0x90, 0x90, 0xF0, 0x10, 0x10,   // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0,   // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0,   // 6
    0xF0, 0x10, 0x20, 0x40, 0x40,   // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0,   // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0,   // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90,   // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0,   // B
    0xF0, 0x90, 0x90, 0x90, 0xF0,   // C
    0xE0, 0x90, 0x90, 0x90, 0xE0,   // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0,   // E
    0xF0, 0x80, 0xF0, 0x80, 0x80    // F
];

#[allow(non_snake_case)]
pub struct CPU {
    memory: [u8; 0x1000],
    // 8bit registers
    V: [u8; 16], 
    // 16bit register, generally for storing addresses
    I: u16, 
    // Program Counter
    pc: u16, 
    // Stack Pointer
    sp: u8,
    stack: [u16; 16],
    // Display Timer
    dt: u8,
    // Sound Timer
    st: u8,

    quirks: Quirk,

    pub vbuffer: [bool; DISPLAY_SIZE],
    pub redraw: bool,

    pub pressed_keys: [bool; 16],

    // pub exec_history: Vec<u16>
}

impl CPU {
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory[0x200..0x200+rom.len()].copy_from_slice(&rom)
    }

    pub fn tick(&mut self, keys: [bool; 16]) {

        self.pressed_keys = keys;

        self.redraw = false;

        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }

        let b1 = self.memory[self.pc as usize] as u16;
        let b2 = self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;

        let op: u16 = (b1 << 8) | b2;

        let args = Instruction::new(op as u16);

        
        let opcode = op >> 12;
        // if opcode != 0x1 {
            // println!("0x{op:0x}");
            // if !self.exec_history.contains(&opcode) {
                // self.exec_history.push(opcode);
                // println!("0x{op:0x}");
            // }
            // println!("{:?}", &args);
            // println!("{:?}", self.V);
        // }
        match opcode {
            0x0 => {
                match args.kk {
                    0xE0 => self.cls(),
                    0xEE => self.ret(),
                    0x00 => return,
                    _ => panic!("Invalid op {op:0x}")
                }
            }
            0x1 => self.jp(args.nnn),
            0x2 => self.call(args.nnn),
            0x3 => self.se_x(args.x, args.kk),
            0x4 => self.sne_x(args.x, args.kk),
            0x5 => self.se_xy(args.x, args.y),
            0x9 => self.sne_xy(args.x, args.y),
            0x6 => self.ld_x(args.x, args.kk),
            0x7 => self.add(args.x, args.kk),
            0x8 => {
                match args.n {
                    0x0 => self.ld_xy(args.x, args.y),
                    0x1 => self.or(args.x, args.y),
                    0x2 => self.and(args.x, args.y),
                    0x3 => self.xor(args.x, args.y),
                    0x4 => self.add_xy(args.x, args.y),
                    0x5 => self.sub(args.x, args.y),
                    0x6 => self.shr(args.x, args.y),
                    0x7 => self.subn(args.x, args.y),
                    0xE => self.shl(args.x, args.y),
                    _ => panic!("Invalid op 0x{op:0x}")
                }
            },
            0xA => self.ld_i(args.nnn),
            0xB => self.jp_v0(args.nnn),
            0xC => self.rnd(args.x, args.kk),
            0xD => self.drw(args.x, args.y, args.n),
            0xE => {
                match args.kk {
                    0x9E => self.skp(args.x),
                    0xA1 => self.sknp(args.x),
                    _ => panic!("Invalid op 0x{op:0x}")
                }
            }
            0xF => {
                match args.kk {
                    0x07 => self.ld_xdt(args.x),
                    0x0A => self.ld_xk(args.x),
                    0x15 => self.ld_dtx(args.x),
                    0x18 => self.ld_stx(args.x),
                    0x29 => self.ld_ix(args.x),
                    0x33 => self.ld_ix_bcd(args.x),
                    0x55 => self.ld_ivx(args.x),
                    0x65 => self.ld_vxi(args.x),
                    0x1E => self.add_i(args.x),
                    _ => panic!("Invalid op 0x{op:0x}")
                }
            }
            o => {
                println!("0x{o:0x} not implemented or invalid.");
            }
        }
    }

    fn read_v(&self, addr: u8) -> u8 {
        self.V[addr as usize]
    }

    fn write_v(&mut self, addr: u8, val: u8) {
        self.V[addr as usize] = val
    }

    fn stack_pop(&mut self) -> u16 {
        let val = self.stack[self.sp as usize];
        self.sp -= 1;
        val
    }

    fn stack_push(&mut self, val: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = val;
    }

}

impl CPU {
    // 0x00E0
    pub fn cls(&mut self) {
        self.vbuffer = [false; DISPLAY_SIZE]
    }

    // 0x00EE
    pub fn ret(&mut self) {
        self.pc = self.stack_pop();
    }   

    // 0x1NNN
    pub fn jp(&mut self, addr: u16) {
        self.pc = addr;
    }

    // 0xBnnn
    pub fn jp_v0(&mut self, addr: u16) {
        let target = if self.quirks.jump_vx { (addr & 0x0F00) >> 8 } else { 0 } as u8;
        let vt = self.read_v(target) as u16;
        self.pc = vt.wrapping_add(addr);
    }

    //0x2NNN
    pub fn call(&mut self, addr: u16) {
        self.stack_push(self.pc);
        self.pc = addr;
    }

    // 0x3xkk
    pub fn se_x(&mut self, x: u8, byte: u8) {
        if self.read_v(x) == byte {
            self.pc += 2;
        }
    }

    // 0x5xy0
    pub fn se_xy(&mut self, x: u8, y: u8) {
        if self.read_v(x) == self.read_v(y) {
            self.pc += 2;
        }
    }

    // 0x4xkk
    pub fn sne_x(&mut self, x: u8, byte: u8) {
        if self.read_v(x) != byte {
            self.pc += 2;
        }
    }

    // 0x9xy0
    pub fn sne_xy(&mut self, x: u8, y: u8) {
        if self.read_v(x) != self.read_v(y) {
            self.pc += 2;
        }
    }

    // 0x6xkk
    pub fn ld_x(&mut self, x: u8, byte: u8) {
        self.write_v(x, byte);
    }

    // 0x8xy0
    pub fn ld_xy(&mut self, x: u8, y: u8) {
        self.write_v(x, self.read_v(y));
    }

    // 0xAnnn
    pub fn ld_i(&mut self, addr: u16) {
        self.I = addr;
    }

    // 0xFx07
    pub fn ld_xdt(&mut self, x: u8) {
        self.write_v(x, self.dt);
    }

    // 0xFx0A
    pub fn ld_xk(&mut self, x: u8) {
        for (i, k) in self.pressed_keys.iter().enumerate() {
            if *k {
                self.write_v(x, i as u8);
                return;
            }
        }
        self.pc -= 2;
    }

    // 0xFx15 
    pub fn ld_dtx(&mut self, x: u8) {
        self.dt = self.read_v(x);
    }

    // 0xFx18
    pub fn ld_stx(&mut self, x: u8) {
        self.st = self.read_v(x);
    }

    // 0xFx29
    pub fn ld_ix(&mut self, x: u8) {
        self.I = self.read_v(x) as u16 * 5;
    }

    // 0xFx33
    pub fn ld_ix_bcd(&mut self, x: u8) {
        let vx = self.read_v(x);
        self.memory[self.I as usize] = vx / 100;
        self.memory[self.I as usize + 1] = (vx % 100) / 10;
        self.memory[self.I as usize + 2] = vx % 10;
    }

    // 0xFx55
    pub fn ld_ivx(&mut self, x: u8) {
        for i in 0..=x {
            let vi = self.read_v(i);
            let idx = self.I + i as u16;
            self.memory[idx as usize] = vi;
        }
        if self.quirks.mem_inc {
            self.I += (x + 1) as u16;
        }
    }

    // 0xFx65
    pub fn ld_vxi(&mut self, x: u8) {
        for i in 0..=x {
            let idx = self.I + i as u16;
            let mi = self.memory[idx as usize];
            self.write_v(i, mi)
        }
        if self.quirks.mem_inc {
            self.I += (x + 1) as u16;
        }
    }

    // 0x7xkk
    pub fn add(&mut self, x: u8, byte: u8) {
        let vx = self.read_v(x) as u16;
        let kk = byte as u16;
        let sum = vx + kk;
        self.write_v(x, sum as u8);
    }
    
    // 0x8xy1
    pub fn or(&mut self, x: u8, y: u8) {
        self.V[x as usize] |= self.read_v(y);
        if self.quirks.vf_reset {
            self.V[0xF] = 0;
        }
    }

    // 0x8xy2
    pub fn and(&mut self, x: u8, y: u8) {
        self.V[x as usize] &= self.read_v(y);
        if self.quirks.vf_reset {
            self.V[0xF] = 0;
        }
    }

    // 0x8xy3
    pub fn xor(&mut self, x: u8, y: u8) {
        self.V[x as usize] ^= self.read_v(y);
        if self.quirks.vf_reset {
            self.V[0xF] = 0;
        }
    }

    // 0x8xy4
    pub fn add_xy(&mut self, x: u8, y: u8) {
        let vx = self.read_v(x) as u16;
        let vy = self.read_v(y) as u16;
        let sum = vx + vy;
        self.write_v(x, sum as u8);

        if sum > 0xFF {
            self.V[0xF] = 1;
        }
        else {
            self.V[0xF] = 0
        }
    }

    // 0xFx1E
    pub fn add_i(&mut self, x: u8) {
        self.I += self.read_v(x) as u16;
        self.V[0x0F] = if self.I > 0x0F00 { 1 } else { 0 };
    }

    // 0x8xy5
    pub fn sub(&mut self, x: u8, y: u8) {
        let vx = self.read_v(x) as u16;
        let vy = self.read_v(y) as u16;
        let sum = vx.wrapping_sub(vy);
        self.write_v(x, sum as u8);

        if self.read_v(x) > self.read_v(y) {
            self.V[0xF] = 1;
        }
        else {
            self.V[0xF] = 0;
        }
    }

    // 0x8xy7
    pub fn subn(&mut self, x: u8, y: u8) {
        let vx = self.read_v(x) as u16;
        let vy = self.read_v(y) as u16;
        let sum = vy.wrapping_sub(vx);
        self.write_v(x, sum as u8);

        if self.read_v(y) > self.read_v(x) {
            self.V[0xF] = 1;
        }
        else {
            self.V[0xF] = 0;
        }
    }

    // For both shr and shl, in older versions of chip8 interpreters
    // the op would shift Vx, but newer versions would shift Vy,
    // may add compatibility for older versions at some point but using new for now

    // 0x8xy6
    pub fn shr(&mut self, x: u8, y: u8) {
        let target = if self.quirks.shift_x { x } else { y }; 

        let flag = self.read_v(target) & 1;
        self.V[x as usize] = self.read_v(target) >> 1;
        self.V[0xF] = flag;
    }

    // 0x8xyE
    pub fn shl(&mut self, x: u8, y: u8) {
        let target = if self.quirks.shift_x { x } else { y }; 

        let flag = self.read_v(target) >> 7;
        self.V[x as usize] = self.read_v(target) << 1;
        self.V[0xF] = flag;
    }

    // 0xCxkk
    pub fn rnd(&mut self, x: u8, kk: u8) {
        self.V[x as usize] = random::<u8>() & kk;
    }

    // 0xDxyn
    pub fn drw(&mut self, x: u8, y: u8, n: u8) {
        self.V[0x0F] = 0;
        'pixel: for byte in 0..n {
            for bit in 0..8 {
                let sprite_bit = (self.memory[self.I as usize + byte as usize] >> (7 - bit)) & 1;

                let mut pos_x = (self.read_v(x) + bit) as usize;
                let mut pos_y = (self.read_v(y) + byte) as usize;
                
                // Doesn't work idk
                // if (pos_x >= DISPLAY_WIDTH || pos_y >= DISPLAY_HEIGHT) && self.quirks.clipping {
                //     // println!("Clipping: X{pos_x} Y{pos_y}");
                //     continue 'pixel;
                // }

                pos_x %= DISPLAY_WIDTH;
                pos_y %= DISPLAY_HEIGHT;
                
                let pixel_index: usize = (pos_y * DISPLAY_WIDTH) + pos_x;

                let old_bit = if self.vbuffer[pixel_index] {1} else {0};

                self.V[0xF] |= sprite_bit & old_bit;
                self.vbuffer[pixel_index] = old_bit ^ sprite_bit != 0;

            }
        }
        self.redraw = true;
    }
    // pub fn drw(&mut self, x: u8, y: u8, n: u8) {
    //     let cx: u8 = self.read_v(x);
    //     let cy: u8 = self.read_v(y);
        
    //     // println!("{cx}, {cy}");

    //     self.V[0xF] = 0;

    //     for byte in 0..n {
    //         let sprite_row = self.memory[(self.I + byte as u16) as usize];
    //         let pos_y: usize = (cy + byte) as usize % DISPLAY_HEIGHT;

    //         for bit in 0..8 {
    //             let pos_x: usize = (cx as usize + bit) % DISPLAY_WIDTH;
    //             let pixel_index: usize = (pos_y * DISPLAY_WIDTH) + pos_x;

    //             let old_bit: u8 = if self.vbuffer[pixel_index] {1} else {0};
    //             let sprite_bit: u8 = (sprite_row >> (7 - bit)) & 1;

    //             self.V[0xF] |= sprite_bit & old_bit;
    //             self.vbuffer[pixel_index] = old_bit ^ sprite_bit != 0;

    //             // println!("x: {pos_x} y: {pos_y} idx: {pixel_index}");


    //             // let new_bit = old_bit ^ sprite_bit;
    //             // self.vbuffer[pixel_index] = new_bit == 1;

    //             // if old_bit == 1 && new_bit == 0 {
    //             //     self.V[0xF] = 1;
    //             // }
    //         }
    //     }
    //     self.redraw = true;
    // }
 
    // 0xEx9E
    pub fn skp(&mut self, x: u8) {
        if self.pressed_keys[self.read_v(x) as usize] {
            self.pc += 2;
        }
    }

    // 0xExA1
    pub fn sknp(&mut self, x: u8) {
        if !self.pressed_keys[self.read_v(x) as usize] {
            self.pc += 2;
        }
    }

}

impl CPU {
    pub fn new(quirks: Quirk) -> Self {
        let mut cpu = CPU {
            memory: [0; 0x1000],
            V: [0; 16],
            I: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            dt: 0,
            st: 0,
            pressed_keys: [false; 16],
            vbuffer: [false; DISPLAY_SIZE],
            redraw: true,
            quirks
            // exec_history: Vec::new()
        };
        cpu.memory[..FONT_SET.len()].copy_from_slice(&FONT_SET);
        cpu
    }
}