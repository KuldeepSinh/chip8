extern crate env_logger;
extern crate log;

use chip8::drivers::Drivers;
use chip8::vm::Machine;
use log::debug;
use std::thread;
use std::time::Duration;

fn main() {
    env_logger::init();
    println!("Hello, world!");

    //  while (true)
    // {
    //   processInput();
    //   update();
    //   render();
    //   sleep();
    // }

    //initialize
    debug!("[main()] Initializing the machine.");
    let mut machine: Machine = Machine::new();
    debug!("[main()] Initializing Drivers.");
    let mut drivers: Drivers = Drivers::init_drivers("./ROMs/BRIX");
    //load ROM in memory
    debug!("[main()] Loding ROM in memory.");
    machine.memory.load_data(&drivers.rom_reader.rom);
    //may be following line will not be needed.
    drivers.display_driver.draw_canvas(&machine.vram.cells);
    //while true
    debug!("[main()] Listening to key-board events.");
    while let Ok(keys) = drivers.input_driver.process_events() {
        //process input
        let output_state = machine.process_keys(keys);
        //update
        if output_state.vram.state_changed {
            debug!("[main()] Ouput state changed {:?}.", output_state);
            debug!("[main()] ***************************************");
            drivers.display_driver.draw_canvas(&output_state.vram.cells);
        }
        if output_state.play_sound {
            debug!("[main()] One day it will beep!");
        }
        //render
        //sleep 60 frames per second
        //thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        thread::sleep(Duration::from_millis(2));
    }
}
