extern crate env_logger;
extern crate log;

use chip8::drivers;
use chip8::vm;
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

    let machine = vm::Machine::new();
    let mut drivers = drivers::Drivers::init_drivers("./ROMs/UFO");
    drivers.display_driver.draw_canvas(&machine.vram.cells);

    //while true
    while let Ok(_keypad) = drivers.input_driver.process_events() {
        //process input

        //update

        //render
        //sleep 60 frames per second
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
