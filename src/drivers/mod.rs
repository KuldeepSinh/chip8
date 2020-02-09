mod display_driver;
mod input_driver;

use display_driver::DisplayDriver;
use input_driver::InputDriver;
use sdl2;

pub struct Drivers {
    pub display_driver: DisplayDriver,
    pub input_driver: InputDriver,
}

impl Drivers {
    pub fn new() -> Drivers {
        let sdl_context = Drivers::init_sdl_context();
        let display_driver = Drivers::init_display_driver(&sdl_context);
        Drivers {
            display_driver: display_driver,
            input_driver: InputDriver::new(&sdl_context),
        }
    }

    fn init_sdl_context() -> sdl2::Sdl {
        let sdl_context = sdl2::init().expect("Error: Could not initialize SDL Context");
        sdl_context
    }

    fn init_display_driver(sdl2_context: &sdl2::Sdl) -> DisplayDriver {
        DisplayDriver::new(sdl2_context, 64, 32)
    }
}
