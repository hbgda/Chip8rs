use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

pub struct Display {
    canvas: WindowCanvas,
    scale: usize,
    pub buffer: [bool; DISPLAY_SIZE as usize],
    pub redraw: bool
}    

impl Display {
    pub fn clear(&mut self) {
        self.buffer = [false; DISPLAY_SIZE]
    }    
}    

impl Display {
    pub fn new(sdl: &Sdl, scale: usize) -> Self {
        let video = sdl.video().unwrap();
        let window = video.window(
            "CHIP-8",
            (DISPLAY_WIDTH * scale) as u32, 
            (DISPLAY_HEIGHT * scale) as u32
        )
            .position_centered()
            .build()
            .unwrap();

            let mut canvas = window.into_canvas().build().unwrap();
            canvas.set_draw_color(Color::BLACK);
            canvas.set_scale(scale as f32, scale as f32).unwrap();
            canvas.clear();
            canvas.present();

        Display {
            canvas, scale, buffer: [false; DISPLAY_SIZE], redraw: false
        }
    }

    pub fn draw(&mut self, pixels: &[bool; DISPLAY_SIZE]) {
        for (i, b) in pixels.iter().enumerate() {
            let x = (i % DISPLAY_WIDTH) as i32;
            let y = (i / DISPLAY_WIDTH) as i32;
        //     let rect = Rect::new(x, y, self.scale as u32, self.scale as u32);
            self.canvas.set_draw_color(
                if *b { Color::WHITE } else { Color::BLACK }
            );
            let rect = Rect::new(x, y, 1, 1);
            let _ = self.canvas.fill_rect(rect);
        }
        self.canvas.present();
    }
}   