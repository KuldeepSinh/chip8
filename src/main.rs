extern crate env_logger;
extern crate log;

use chip8::vm;

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

    // //check if memory is initialized
    // let mut machine = vm::Machine::new();
    // //let mut mem = memory::Memory::new();
    // //mem.initialize();
    // for i in machine.memory.cells.iter() {
    //     print!("{:03} ", i);
    // }
    let mut machine = vm::Machine::new();
    machine.pc = 4;
    machine.stack.cells.push(1);
    machine.stack.cells.push(42);
    println!("{}", machine.pc);

    for j in 0..machine.vram.cells.len() {
        machine.vram.cells[j] = 1;
        print!("{:2}", machine.vram.cells[j]);
    }
    println!();

    machine.execute_instruction();
    for j in 0..machine.vram.cells.len() {
        print!("{:2}", machine.vram.cells[j]);
    }
    println!();
    //machine.pc = machine.stack.cells.pop().expect("error");
    // machine.pc = 42;
    // machine.stack.cells.push(1);
    // machine.stack.cells.push(2);
    // set_pc(&mut machine);

    println!("{}", machine.pc);

    println!("{:?}", 127u8.overflowing_shr(1u32));
    println!("{:?}", 127u8.overflowing_shl(1u32));

    println!("{:b} {}", 3u8, 1u8 >> 7);
    println!("{:b} {}", 4u8, 2u8 >> 7);

    let mut i: u8 = 0;
    while i < 255 {
        print!("({:b},{}) ", i, i & 0x80);
        i = i + 1;
    }
    println!();
}

// fn set_pc(machine: &mut vm::Machine) {
//     machine.pc = machine.stack.cells.pop().expect("error");
// }
