use sdl2::{EventPump, keyboard::Keycode};

pub fn get_keys(events: &EventPump) -> [bool; 16] {
    let pressed_keys: Vec<Keycode> = events
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

    let mut chip_keys = [false; 16];
    for key in pressed_keys {
        let i = match key {
            Keycode::Num1 => 0x1, Keycode::Num2 => 0x2, Keycode::Num3 => 0x3, Keycode::Num4 => 0xC,
            Keycode::Q    => 0x4, Keycode::W    => 0x5, Keycode::E    => 0x6, Keycode::R    => 0xD,
            Keycode::A    => 0x7, Keycode::S    => 0x8, Keycode::D    => 0x9, Keycode::F    => 0xE,
            Keycode::Z    => 0xA, Keycode::X    => 0x0, Keycode::C    => 0xB, Keycode::V    => 0xF,
            _ => continue
        };
        chip_keys[i] = true;
    }

    chip_keys
}