extern crate env_logger;
extern crate log;

use chip8::drivers::Drivers;
use chip8::vm::Machine;
use log::{debug, info};
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    env_logger::init();
    //Read from commandline.
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    //initialize
    debug!("[main()] Initializing the Machine.");
    let mut machine: Machine = Machine::new();
    debug!("[main()] Initializing Drivers.");
    let mut drivers: Drivers = Drivers::init_drivers();
    drivers.rom_reader.read_rom(filename);

    //load ROM in memory
    debug!("[main()] Loding ROM in memory.");
    machine.memory.load_data(&drivers.rom_reader.rom);

    //while true
    info!("[main()] Listening to key-board events.");
    while let Ok(keys) = drivers.input_driver.process_events() {
        //process input
        info!("[main()] Keys found {:?}", keys);
        let output_state = machine.process_keys(keys);
        //update
        if output_state.vram.state_changed {
            info!("[main()] Drawing canvas.");
            drivers.display_driver.draw_canvas(&output_state.vram.cells);
        }
        if output_state.play_sound {
            debug!("[main()] One day it will beep!");
        }
        
        thread::sleep(Duration::from_millis(2));
    }
}
