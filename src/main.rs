use std::{fs, time::Duration, thread};

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
    pub scale: Option<usize>,

    // Quirks
    /// Opcodes [0x8XY1-3] will reset VF.
    #[arg(short, long)]
    pub vf_reset: Option<bool>,

    /// Opcodes [0xFX55] and [0xFX65] will increment I.
    #[arg(short, long)]
    pub mem_inc: Option<bool>,

    // Idk not implemented
    // pub display_wait: Option<bool>,
    
    // Clip sprites instead of wrapping when they go off screen.
    // #[arg(short, long)]
    // pub clipping: Option<bool>,

    /// Shift use Vx for shifting instead of Vy in [0x8XY6] and [0x8XYE].
    #[arg(short, long)]
    pub shift_x: Option<bool>,
    /// Opcode [0xBNNN] will jump to address + VX, where X is the highest nibble of NNN, instead of address + V0.
    #[arg(short, long)]
    pub jump_vx: Option<bool>
}

fn main() {
    let (rom_path, scale, quirks) = &parse_args();

    let rom = fs::read(rom_path).expect(format!("Failed to read file at: {}", rom_path).as_str());

    let context = sdl2::init().unwrap();

    let mut display = Display::new(&context, *scale);
    let mut chip = cpu::CPU::new(*quirks);
    chip.load_rom(rom);

    // display.draw(&chip.vbuffer);

    let mut event_pump = context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let keys = input::get_keys(&event_pump);
        chip.tick(keys);
        if chip.redraw {
            display.draw(&chip.vbuffer);
            chip.redraw = false;
        }

        thread::sleep(Duration::from_millis(2));
        // The rest of the game loop goes here...
    }
    
    // loop {
    //     chip.tick();
    //     chip.display.draw();
    // }
}

pub fn parse_args() -> (String, usize, Quirk) {
    let cli = CLI::parse();
    let rom_path = cli.rom_path
        .to_str().unwrap()
        .to_string();

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

    (rom_path, cli.scale.unwrap_or(10), quirks)
}