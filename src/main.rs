use std::{fs, time::Duration, thread, io};

use display::Display;
use quirk::Quirk;
use sdl2::{self, event::Event, keyboard::Keycode};
use clap::Parser;

pub mod display;
pub mod cpu;
pub mod instruction;
pub mod input;
pub mod quirk;

#[derive(Parser)]
pub struct CLI {
    /// Path to the rom to load.
    pub rom_path: std::path::PathBuf,

    /// Scale to multiply the screen size, default 10.
    #[arg(short, long)]
    pub scale: Option<usize>,

    /// .
    #[arg(short, long)]
    pub tick_delay: Option<u64>,

    /// Amount of ticks between each render, (default 60).
    #[arg(short='p', long)]
    pub ticks_per_frame: Option<u8>,

    /// Debug mode, requires a key press to proceed execution, prints contents of registers and the current instruction at each cycle.
    #[arg(short, long)]
    pub debug: Option<bool>,

    // Quirks
    /// Opcodes [0x8XY1-3] will reset VF.
    #[arg(short, long)]
    pub vf_reset: Option<bool>,

    /// Opcodes [0xFX55] and [0xFX65] will increment I.
    #[arg(short, long)]
    pub mem_inc: Option<bool>,

    // Idk not implemented
    // pub display_wait: Option<bool>,
    
    /// Clip sprites instead of wrapping when they go off screen.
    #[arg(short, long)]
    pub clipping: Option<bool>,

    /// Shift use Vx for shifting instead of Vy in [0x8XY6] and [0x8XYE].
    #[arg(short='x', long)]
    pub shift_x: Option<bool>,
    /// Opcode [0xBNNN] will jump to NNN + VX, where X is the highest nibble of NNN, instead of NNN + V0.
    #[arg(short, long)]
    pub jump_vx: Option<bool>
}

pub struct CHIP8Options {
    pub rom_path: String,
    pub scale: usize,
    pub ticks_per_frame: u8,
    pub tick_delay: u64,
    pub debug: bool,
    pub quirks: Quirk
}

fn main() {
    let opts = parse_args();

    let rom = fs::read(&opts.rom_path).expect(format!("Failed to read file at: {}", &opts.rom_path).as_str());

    let context = sdl2::init().unwrap();

    let mut display = Display::new(&context, opts.scale);
    let mut chip = cpu::CPU::new(opts.quirks, opts.debug);
    chip.load_rom(rom);

    // display.draw(&chip.vbuffer);

    let mut event_pump = context.event_pump().unwrap();

    'running: loop {
        
        let keys = input::get_keys(&event_pump);
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => {
                    if keycode.unwrap() == Keycode::I {
                        println!("PEEK RAM: ");
                        let mut idx_str = String::new();
                        io::stdin().read_line(&mut idx_str).expect("Failed to read index.");
                        let idx = idx_str.trim().parse::<u16>().expect("Failed to parse idx_str");
                        println!("RAM[0x{idx:0x}] = {}", chip.memory[idx as usize]);
                    }
                    if opts.debug {
                        chip.tick(keys)
                    }
                }
                _ => {}
            }
        }
        if !opts.debug {
            chip.tick(keys)
        }

        if chip.redraw {
            display.draw(&chip.vbuffer);
            chip.redraw = false;
        }

        thread::sleep(Duration::from_millis(opts.tick_delay));
        // The rest of the game loop goes here...
    }
    
    // loop {
    //     chip.tick();
    //     chip.display.draw();
    // }
}

pub fn parse_args() -> CHIP8Options {
    let cli = CLI::parse();

    let quirks = Quirk {
        vf_reset: cli.vf_reset.unwrap_or(true),
        mem_inc: cli.mem_inc.unwrap_or(true),
        display_wait: false,
        // display_wait: cli.display_wait.unwrap_or(false),
        clipping: false,
        // clipping: cli.clipping.unwrap_or(false),
        shift_x: cli.shift_x.unwrap_or(false),
        jump_vx: cli.jump_vx.unwrap_or(false),
    };

    print!("{quirks:?}");
    CHIP8Options {
        rom_path: cli.rom_path.to_str().unwrap().to_string(), 
        scale: cli.scale.unwrap_or(10), 
        tick_delay: cli.tick_delay.unwrap_or(2),
        ticks_per_frame: cli.ticks_per_frame.unwrap_or(30), 
        debug: cli.debug.unwrap_or(false),
        quirks: quirks
    }
}