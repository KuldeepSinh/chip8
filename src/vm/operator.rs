use crate::vm::Machine;
use log::debug;

type OpCode = u16;

pub struct Operator {
    pub opcode: OpCode,
    pub nnn_address: u16,
    pub nn_const: u8,
    pub n_const: u8,
    pub x: usize,
    pub y: usize,
    pub vx: u8,
    pub vy: u8,
    pub nibble_1: u8,
    pub nibble_2: u8,
    pub nibble_3: u8,
    pub nibble_4: u8,
}

impl Operator {
    pub fn get_operators(machine: &Machine) -> Operator {
        let opcode = extract_opcode(machine);
        Operator {
            opcode,
            nnn_address: opcode & 0x0FFF,
            nn_const: (opcode & 0x00FF) as u8,
            n_const: (opcode & 0x000F) as u8,
            //registers
            x: ((opcode & 0x0F00) >> 8) as usize,
            y: ((opcode & 0x00F0) >> 8) as usize,
            vx: machine.registers.v[((opcode & 0x0F00) >> 8) as usize],
            vy: machine.registers.v[((opcode & 0x00F0) >> 8) as usize],
            //nibbles
            nibble_1: ((opcode & 0xF000) >> 12) as u8,
            nibble_2: ((opcode & 0x0F00) >> 8) as u8,
            nibble_3: ((opcode & 0x00F0) >> 4) as u8,
            nibble_4: (opcode & 0x000F) as u8,
        }
    }
}

fn extract_opcode(machine: &Machine) -> OpCode {
    debug!(
        "[extract_opcode()] Trying to extract opcode at pc = {}.",
        machine.pc
    );
    (machine.memory.cells[machine.pc as usize] as u16) << 8
        | (machine.memory.cells[(machine.pc + 1) as usize] as u16)
}
