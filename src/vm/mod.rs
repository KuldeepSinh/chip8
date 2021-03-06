//! CHIP-8 VM has following basic components :
//! Graphics (display and font-set), Instruction set, Keyboard, Memory, Registers, Stack and Timers.

mod graphics;
mod instructions;
mod keyboard;
mod memory;
mod operator;
mod registers;
mod stack;
mod timers;

use graphics::vram::VRam;
use keyboard::KeyBoard;
use log::info;
use memory::Memory;
use operator::Operator;
use registers::Registers;
use stack::Stack;
use timers::Timers;

const VRAM_WIDTH: usize = 64;
const VRAM_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct OutputState<'a> {
    pub vram: &'a VRam,
    pub play_sound: bool,
}

pub struct Machine {
    pub vram: VRam,
    pub keyboard: KeyBoard,
    pub memory: Memory,
    pub registers: Registers,
    pub stack: Stack,
    pub timers: Timers,
    /// `i` is a 16-bit index register. It can have value from 0x000 to 0xFFF.
    pub i: u16,
    /// `pc` is a 16-bit program counter. It can have value from 0x000 to 0xFFF.
    pub pc: u16,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            vram: VRam::new(VRAM_WIDTH, VRAM_HEIGHT),
            keyboard: KeyBoard::new(),
            memory: Memory::new(),
            registers: Registers::new(),
            stack: Stack::new(),
            timers: Timers::new(),
            i: 0,
            pc: 0x200,
        }
    }

    pub fn get_operators(&self) -> Operator {
        Operator::get_operators(&self)
    }

    pub fn process_keys(&mut self, keys: Vec<bool>) -> OutputState {
        info!("[Machine.process_keys()] The machine started processing keys.");
        self.vram.state_changed = false;
        self.keyboard.keys = keys;

        if self.keyboard.keypress_awaited {
            for (i, &key) in self.keyboard.keys.iter().enumerate() {
                if key {
                    self.keyboard.keypress_awaited = false;
                    self.registers.v[self.keyboard.key_register] = i as u8;
                    break;
                }
            }
        } else {
            if self.timers.st > 0 {
                self.timers.st -= 1;
            }
            if self.timers.dt > 0 {
                self.timers.dt -= 1;
            }
            self.emulate_cycle();
        }

        OutputState {
            vram: &self.vram,
            play_sound: false,
        }
    }

    fn emulate_cycle(&mut self) {
        //let mach= self;
        let operator = self.get_operators();
        match (
            operator.nibble_1,
            operator.nibble_2,
            operator.nibble_3,
            operator.nibble_4,
        ) {
            //clear screen
            (0, 0, 0xE, 0) => instructions::execute_00e0(self),
            //return from subroutine
            (0, 0, 0xE, 0xE) => instructions::execute_00ee(self),
            //jump (goto) to address nnn
            (0x1, _, _, _) => instructions::execute_1nnn(self, &operator),
            //call subroutine at nnn
            (0x2, _, _, _) => instructions::execute_2nnn(self, &operator),
            //Skips the next instruction if VX equals NN.
            (0x3, _, _, _) => instructions::execute_3nnn(self, &operator),
            //Skips the next instruction if VX doesn't equal NN.
            (0x4, _, _, _) => instructions::execute_4nnn(self, &operator),
            //Skips the next instruction if VX equals VY.
            (0x5, _, _, 0) => instructions::execute_5xy0(self, &operator),
            //Sets VX to NN.
            (0x6, _, _, _) => instructions::execute_6xnn(self, &operator),
            //Adds NN to VX.
            (0x7, _, _, _) => instructions::execute_7xnn(self, &operator),
            //Sets VX to the value of VY.
            (0x8, _, _, 0x0) => instructions::execute_8xy0(self, &operator),
            //Sets VX to VX or VY. (Bitwise OR operation)
            (0x8, _, _, 0x1) => instructions::execute_8xy1(self, &operator),
            //Sets VX to VX and VY. (Bitwise AND operation)
            (0x8, _, _, 0x2) => instructions::execute_8xy2(self, &operator),
            //Sets VX to VX xor VY.
            (0x8, _, _, 0x3) => instructions::execute_8xy3(self, &operator),
            //Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
            (0x8, _, _, 0x4) => instructions::execute_8xy4(self, &operator),
            //VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            (0x8, _, _, 0x5) => instructions::execute_8xy5(self, &operator),
            //Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
            (0x8, _, _, 0x6) => instructions::execute_8xy6(self, &operator),
            //Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            (0x8, _, _, 0x7) => instructions::execute_8xy7(self, &operator),
            //Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
            (0x8, _, _, 0xE) => instructions::execute_8xye(self, &operator),
            //Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block)
            (0x9, _, _, 0x0) => instructions::execute_9xy0(self, &operator),
            //Sets I to the address NNN.
            (0xA, _, _, _) => instructions::execute_annn(self, &operator),
            //Jumps to the address NNN plus V0.
            (0xB, _, _, _) => instructions::execute_bnnn(self, &operator),
            //Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
            (0xC, _, _, _) => instructions::execute_cxnn(self, &operator),
            //Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
            //Each row of 8 pixels is read as bit-coded starting from memory location I;
            //I value doesn’t change after the execution of this instruction.
            //As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn,
            //and to 0 if that doesn’t happen
            (0xD, _, _, _) => instructions::execute_dxyn(self, &operator),
            //Skips the next instruction if the key stored in VX is pressed.
            //(Usually the next instruction is a jump to skip a code block)
            (0xE, _, 0x9, 0xE) => instructions::execute_ex9e(self, &operator),
            //Skips the next instruction if the key stored in VX isn't pressed.
            //(Usually the next instruction is a jump to skip a code block)
            (0xE, _, 0xA, 0x1) => instructions::execute_exa1(self, &operator),
            //Sets VX to the value of the delay timer.
            (0xF, _, 0x0, 0x7) => instructions::execute_fx07(self, &operator),
            //A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
            (0xF, _, 0x0, 0xA) => instructions::execute_fx0a(self, &operator),
            //Sets the delay timer to VX.
            (0xF, _, 0x1, 0x5) => instructions::execute_fx15(self, &operator),
            //Sets the sound timer to VX.
            (0xF, _, 0x1, 0x8) => instructions::execute_fx18(self, &operator),
            //Adds VX to I. VF is set to 1 when there is a range overflow (I+VX>0xFFF), and to 0 when there isn't.
            (0xF, _, 0x1, 0xE) => instructions::execute_fx1e(self, &operator),
            //Sets I to the location of the sprite for the character in VX.
            //Characters 0-F (in hexadecimal) are represented by a 4x5 font.
            (0xF, _, 0x2, 0x9) => instructions::execute_fx29(self, &operator),
            //Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I,
            //the middle digit at I plus 1, and the least significant digit at I plus 2.
            //(In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I,
            //the tens digit at location I+1, and the ones digit at location I+2.)
            (0xF, _, 0x3, 0x3) => instructions::execute_fx33(self, &operator),
            //Stores V0 to VX (including VX) in memory starting at address I.
            //The offset from I is increased by 1 for each value written, but I itself is left unmodified.
            (0xF, _, 0x5, 0x5) => instructions::execute_fx55(self, &operator),
            //Fills V0 to VX (including VX) with values from memory starting at address I.
            //The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
            (0xF, _, 0x6, 0x5) => instructions::execute_fx65(self, &operator),
            //If no match found, do nothing.
            (_, _, _, _) => (),
        }
    }
}
