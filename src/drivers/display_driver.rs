use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE_FACTOR: u32 = 20;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl, width: usize, height: usize) -> Self {
        //get window
        let window = DisplayDriver::get_window(
            sdl_context,
            width * (SCALE_FACTOR as usize),
            height * (SCALE_FACTOR as usize),
        );
        //get canvas
        let canvas = DisplayDriver::get_canvas(window);
        //return
        DisplayDriver { canvas: canvas }
    }

    pub fn draw_canvas(&mut self, vram: &Vec<Vec<u8>>) {
        for (h, row) in vram.iter().enumerate() {
            for (w, col) in row.iter().enumerate() {
                //set color to draw
                self.canvas.set_draw_color(DisplayDriver::get_color(col));
                //draw rectangle
                let h = (h as u32) * SCALE_FACTOR;
                let w = (w as u32) * SCALE_FACTOR;
                self.canvas
                    .fill_rect(Rect::new(w as i32, h as i32, SCALE_FACTOR, SCALE_FACTOR))
                    .expect("Error : Could not draw pixel.");
            }
        }
        self.canvas.present();
    }
}

//private methods
impl DisplayDriver {
    fn get_window(sdl_context: &sdl2::Sdl, width: usize, height: usize) -> Window {
        //video sub-system
        let video_subsystem = sdl_context
            .video()
            .expect("Error: Could not initialize video-subsystem.");

        //make window
        let window = video_subsystem
            .window("CHIP-8 VM; made with Rust", width as u32, height as u32)
            .position_centered()
            .opengl()
            .build()
            .expect("Error: Could not build window.");
        window
    }

    fn get_canvas(window: Window) -> Canvas<Window> {
        //build canvas
        let canvas = window
            .into_canvas()
            .build()
            .expect("Error: Could not build canvas.");
        canvas
    }
    fn get_color(pixel: &u8) -> pixels::Color {
        match pixel {
            0 => pixels::Color::RGB(0, 0, 0),
            _ => pixels::Color::RGB(0, 0, 255),
        }
    }
}
