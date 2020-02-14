use log::{debug, info};
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputDriver {
    event_pump: sdl2::EventPump,
}

impl InputDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        InputDriver {
            event_pump: sdl_context
                .event_pump()
                .expect("Error : Could not initialize event pump."),
        }
    }

    pub fn process_events(&mut self) -> Result<Vec<bool>, ()> {
        //Quit if user wants to quit
        for event in self.event_pump.poll_iter() {
            info!("[InputDriver.process_events()] Started processing events.");
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Err(());
                }
                _ => {
                    info!("[InputDriver.process_events()] No matching event found.");
                }
            };
        }
        debug!("[InputDriver.process_events()] Started Reading Keys.");
        //process key presses
        let keys: Vec<Keycode> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        let mut chip8_keys = vec![false; 16];
        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };
            if let Some(i) = index {
                debug!("[InputDriver.process_events()] Some key = {} was read.", i);
                chip8_keys[i] = true;
            }
        }
        Ok(chip8_keys)
    }
}
