extern crate env_logger;
extern crate log;

use chip8::drivers::Drivers;
use chip8::vm::Machine;
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
    let mut machine: Machine = Machine::new();
    let mut drivers: Drivers = Drivers::init_drivers("./ROMs/UFO");
    //load ROM in memory
    machine.memory.load_data(&drivers.rom_reader.rom);
    //may be following line will not be needed.
    drivers.display_driver.draw_canvas(&machine.vram.cells);
    //while true
    while let Ok(keys) = drivers.input_driver.process_events() {
        //process input

        //update

        //render
        //sleep 60 frames per second
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
