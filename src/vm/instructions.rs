use crate::vm::operator::Operator;
use crate::vm::Machine;
use log::{debug, info};
use rand::Rng;

//pub fn execute_0nnn() {}

/// `instructions::execute_00e0()`
/// Type = Display
/// Explanation = Clears the screen.
pub fn execute_00e0(machine: &mut Machine) {
    info!("[execute_00e0()] Clearing display.");
    let vram_height = machine.vram.cells.len();
    let vram_width = machine.vram.cells[0].len();
    for h in 0..vram_height {
        for w in 0..vram_width {
            machine.vram.cells[h][w] = 0;
        }
    }
    machine.vram.state_changed = true;
    machine.pc += 2;
    debug!("[execute_00e0()] Cleared display.");
}

/// `instructions::execute_00ee()`
/// Type = Flow
/// Explanation = Returns from a subroutine.
pub fn execute_00ee(machine: &mut Machine) {
    info!("[execute_00ee]");
    machine.pc = machine
        .stack
        .cells
        .pop()
        .expect("Returning from the subroutine is impossible, because the Stack is empty.");
}

/// `instructions::execute_1nnn()`
/// Type = Flow
/// Explanation = Jumps to address NNN.
pub fn execute_1nnn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_1nnn]");
    machine.pc = operator.nnn_address;
}

/// `instructions::execute_2nnn()`
/// Type = Flow
/// Explanation = Calls subroutine at NNN.
pub fn execute_2nnn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_2nnn]");
    machine.stack.cells.push(machine.pc + 2);
    machine.pc = operator.nnn_address;
}

/// `instructions::execute_3nnn()`
/// Type = Condition
/// Explanation = Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block).
pub fn execute_3nnn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_3nnn]");
    match operator.vx == operator.nn_const {
        true => machine.pc += 4,
        false => machine.pc += 2,
    };
}

/// `instructions::execute_4nnn()`
/// Type = Condition
/// Explanation = Skips the next instruction if VX doesn't equal NN. (Usually the next instruction is a jump to skip a code block).
pub fn execute_4nnn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_4nnn]");
    match operator.vx != operator.nn_const {
        true => machine.pc += 4,
        false => machine.pc += 2,
    };
}

/// `instructions::execute_5xy0()`
/// Type = Condition
/// Explanation = Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block).
pub fn execute_5xy0(machine: &mut Machine, operator: &Operator) {
    info!("[execute_5xy0]");
    match operator.vx == operator.vy {
        true => machine.pc += 4,
        false => machine.pc += 2,
    };
}

/// `instructions::execute_6xnn()`
/// Type = Const
/// Explanation = Sets VX to NN.
pub fn execute_6xnn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_6xnn]");
    machine.registers.v[operator.x] = operator.nn_const;
    machine.pc += 2;
}

/// `instructions::execute_7xnn()`
/// Type = Const
/// Explanation = Adds NN to VX. (Carry flag is not changed).
pub fn execute_7xnn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_7xnn]");
    machine.registers.v[operator.x] = machine.registers.v[operator.x]
        .overflowing_add(operator.nn_const)
        .0;
    machine.pc += 2;
}

/// `instructions::execute_8xy0()`
/// Type = Assign
/// Explanation = Sets VX to the value of VY.
pub fn execute_8xy0(machine: &mut Machine, operator: &Operator) {
    info!("[execute_8xy0]");
    machine.registers.v[operator.x] = machine.registers.v[operator.y];
    machine.pc += 2;
}

/// `instructions::execute_8xy1()`
/// Type = Bitwise Operation
/// Explanation = Sets VX to VX or VY. (Bitwise OR operation)
pub fn execute_8xy1(machine: &mut Machine, operator: &Operator) {
    info!("[execute_8xy1]");
    machine.registers.v[operator.x] |= machine.registers.v[operator.y];
    machine.pc += 2;
}

/// `instructions::execute_8xy2()`
/// Type = Bitwise Operation
/// Explanation = Sets VX to VX and VY. (Bitwise AND operation)
pub fn execute_8xy2(machine: &mut Machine, operator: &Operator) {
    info!("[execute_8xy2]");
    machine.registers.v[operator.x] &= machine.registers.v[operator.y];
    machine.pc += 2;
}

/// `instructions::execute_8xy3()`
/// Type = Bitwise Operation
/// Explanation = Sets VX to VX xor VY.
pub fn execute_8xy3(machine: &mut Machine, operator: &Operator) {
    info!("[execute_8xy3]");
    machine.registers.v[operator.x] ^= machine.registers.v[operator.y];
    machine.pc += 2;
}

/// `instructions::execute_8xy4()`
/// Type = Math
/// Explanation = Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
pub fn execute_8xy4(machine: &mut Machine, operator: &Operator) {
    info!("[execute_8xy4]");
    let (result, overflow) =
        machine.registers.v[operator.x].overflowing_add(machine.registers.v[operator.y]);
    match overflow {
        true => machine.registers.v[0xF] = 1,
        false => machine.registers.v[0xF] = 0,
    };
    machine.registers.v[operator.x] = result;
    machine.pc += 2;
}

/// `instructions::execute_8xy5()`
/// Type = Math
/// Explanation = VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
pub fn execute_8xy5(machine: &mut Machine, operator: &Operator) {
    info!("[execute_8xy5]");
    let (result, overflow) =
        machine.registers.v[operator.x].overflowing_sub(machine.registers.v[operator.y]);
    match overflow {
        true => machine.registers.v[0xF] = 0,
        false => machine.registers.v[0xF] = 1,
    };
    machine.registers.v[operator.x] = result;
    machine.pc += 2;
}

/// `instructions::execute_8xy6()`
/// Type = Bitwise Operation
/// Explanation = Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
pub fn execute_8xy6(machine: &mut Machine, operator: &Operator) {
    info!("[execute_8xy6]");
    machine.registers.v[0xF] = machine.registers.v[operator.x] & 0x1;
    machine.registers.v[operator.x] >>= 1;
    machine.pc += 2;
}

/// `instructions::execute_8xy7()`
/// Type = Math
/// Explanation = Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
pub fn execute_8xy7(machine: &mut Machine, operator: &Operator) {
    info!("[execute_8xy7]");
    let (result, overflow) =
        machine.registers.v[operator.y].overflowing_sub(machine.registers.v[operator.x]);
    match overflow {
        true => machine.registers.v[0xF] = 0,
        false => machine.registers.v[0xF] = 1,
    };
    machine.registers.v[operator.x] = result;
    machine.pc += 2;
}

/// `instructions::execute_8xye()`
/// Type = Bitwise Operation
/// Explanation = Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
pub fn execute_8xye(machine: &mut Machine, operator: &Operator) {
    info!("[execute_8xye]");
    machine.registers.v[0xF] = (machine.registers.v[operator.x] & 0b10000000) >> 7;
    machine.registers.v[operator.x] <<= 1;
    machine.pc += 2;
}

/// `instructions::execute_9xy0()`
/// Type = Condition
/// Explanation = Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block)
pub fn execute_9xy0(machine: &mut Machine, operator: &Operator) {
    info!("[execute_9xy0]");
    match operator.vx != operator.vy {
        true => machine.pc += 4,
        false => machine.pc += 2,
    };
}

/// `instructions::execute_annn()`
/// Type = Memory
/// Explanation = Sets I to the address NNN.
pub fn execute_annn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_annn]");
    machine.i = operator.nnn_address;
    machine.pc += 2;
}

/// `instructions::execute_bnnn()`
/// Type = Flow
/// Explanation = Jumps to the address NNN plus V0.
pub fn execute_bnnn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_bnnn]");
    machine.pc = operator.nnn_address + machine.registers.v[0] as u16;
}

/// `instructions::execute_cxnn()`
/// Type = Random
/// Explanation = Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
pub fn execute_cxnn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_cxnn]");
    machine.registers.v[operator.x] = operator.nn_const & rand::thread_rng().gen::<u8>();
    machine.pc += 2;
}

/// `instructions::execute_dxyn()`
/// Type = Display
/// Explanation = Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N (1-15) pixels.
/// Each row of 8 pixels is read as bit-coded starting from memory location I;
/// I value doesn’t change after the execution of this instruction.
/// Sprite pixels are XOR'd with corresponding screen pixels.
/// In other words, sprite pixels that are set flip the color of the corresponding screen pixel,
/// while unset sprite pixels do nothing.
/// The carry flag (VF) is set to 1 if any screen pixels are flipped from set to unset when a sprite is drawn
/// and set to 0 otherwise. This is used for collision detection.
pub fn execute_dxyn(machine: &mut Machine, operator: &Operator) {
    info!("[execute_dxyn]");
    machine.registers.v[0xF] = 0;

    let vram_height = machine.vram.cells.len();
    let vram_width = machine.vram.cells[0].len();
    for byte in 0..operator.n_const {
        let y = (machine.registers.v[operator.y] as usize + byte as usize) % vram_height;
        for bit in 0..8 {
            let x = (machine.registers.v[operator.x] as usize + bit as usize) % vram_width;
            let color = (machine.memory.cells[machine.i as usize + byte as usize] >> (7 - bit)) & 1;
            machine.registers.v[0xF] |= color & machine.vram.cells[y][x];
            machine.vram.cells[y][x] ^= color;
        }
    }
    machine.vram.state_changed = true;
    machine.pc += 2;
}

/// `instructions::execute_ex9e()`
/// Type = KeyOp
/// Explanation = Skips the next instruction if the key stored in VX is pressed.
/// (Usually the next instruction is a jump to skip a code block)
pub fn execute_ex9e(machine: &mut Machine, operator: &Operator) {
    info!("[execute_ex9e]");
    match machine
        .keyboard
        .is_key_down(machine.registers.v[operator.x])
    {
        true => machine.pc += 4,
        false => machine.pc += 2,
    }
}

/// `instructions::execute_exa1()`
/// Type = KeyOp
/// Explanation = Skips the next instruction if the key stored in VX isn't pressed.
/// (Usually the next instruction is a jump to skip a code block)
pub fn execute_exa1(machine: &mut Machine, operator: &Operator) {
    info!("[execute_exa1]");
    match machine
        .keyboard
        .is_key_down(machine.registers.v[operator.x])
    {
        true => machine.pc += 2,
        false => machine.pc += 4,
    };
}

/// `instructions::execute_fx07()`
/// Type = Timer
/// Explanation = Sets VX to the value of the delay timer.
pub fn execute_fx07(machine: &mut Machine, operator: &Operator) {
    info!("[execute_fx07]");
    machine.registers.v[operator.x] = machine.timers.dt;
    machine.pc += 2;
}

/// `instructions::execute_fx0a()`
/// Type = KeyOp
/// Explanation = A key press is awaited, and then stored in VX.
/// (Blocking Operation. All instruction halted until next key event)
pub fn execute_fx0a(machine: &mut Machine, operator: &Operator) {
    info!("[execute_fx0a]");
    machine.keyboard.keypress_awaited = true;
    machine.keyboard.key_register = operator.x;
    machine.pc += 2;
}

/// `instructions::execute_fx15()`
/// Type = Timer
/// Explanation = Sets the delay timer to VX.
pub fn execute_fx15(machine: &mut Machine, operator: &Operator) {
    info!("[execute_fx15]");
    machine.timers.dt = machine.registers.v[operator.x];
    machine.pc += 2;
}

/// `instructions::execute_fx18()`
/// Type = Sound
/// Explanation = Sets the sound timer to VX.
pub fn execute_fx18(machine: &mut Machine, operator: &Operator) {
    info!("[execute_fx18]");
    machine.timers.st = machine.registers.v[operator.x];
    machine.pc += 2;
}

/// `instructions::execute_fx1e()`
/// Type = Memory
/// Explanation = Adds VX to I. VF is set to 1 when there is a range overflow (I+VX>0xFFF), and to 0 when there isn't.
pub fn execute_fx1e(machine: &mut Machine, operator: &Operator) {
    info!("[execute_fx1e]");
    let (result, overflow) = machine
        .i
        .overflowing_add(machine.registers.v[operator.x] as u16);
    match overflow {
        true => machine.registers.v[0xF] = 1,
        false => machine.registers.v[0xF] = 0,
    };
    machine.i = result;
    machine.pc += 2;
}

/// `instructions::execute_fx29()`
/// Type = Memory
/// Explanation = Sets I to the location of the sprite for the character in VX.
/// Characters 0-F (in hexadecimal) are represented by a 4x5 font.
pub fn execute_fx29(machine: &mut Machine, operator: &Operator) {
    info!("[execute_fx29]");
    machine.i = machine.registers.v[operator.x] as u16 * 5;
    machine.pc += 2;
}

/// `instructions::execute_fx33()`
/// Type = BCD (Binary Coded Decimal)
/// Explanation = Stores the binary-coded decimal representation of VX,
/// with the most significant of three digits at the address in I,
/// the middle digit at I plus 1, and the least significant digit at I plus 2.
/// (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I,
/// the tens digit at location I+1, and the ones digit at location I+2.)
pub fn execute_fx33(machine: &mut Machine, operator: &Operator) {
    info!("[execute_fx33]");
    machine.memory.cells[machine.i as usize] = machine.registers.v[operator.x] / 100;
    machine.memory.cells[machine.i as usize + 1] = (machine.registers.v[operator.x] % 100) / 10;
    machine.memory.cells[machine.i as usize + 2] = machine.registers.v[operator.x] % 10;
    machine.pc += 2;
}

/// `instructions::execute_fx55()`
/// Type = Memory
/// Explanation = Stores V0 to VX (including VX) in memory starting at address I.
/// The offset from I is increased by 1 for each value written, but I itself is left unmodified.
pub fn execute_fx55(machine: &mut Machine, operator: &Operator) {
    info!("[execute_fx55]");
    machine.memory.cells[(machine.i as usize)..(machine.i + (operator.x + 1) as u16) as usize]
        .copy_from_slice(&machine.registers.v[0..((operator.x + 1) as usize)]);
    machine.pc += 2;
}

/// `instructions::execute_fx65()`
/// Type = Memory
/// Explanation = Fills V0 to VX (including VX) with values from memory starting at address I.
/// The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
pub fn execute_fx65(machine: &mut Machine, operator: &Operator) {
    info!("[execute_fx65]");
    machine.registers.v[0..((operator.x + 1) as usize)].copy_from_slice(
        &machine.memory.cells[(machine.i as usize)..(machine.i + (operator.x + 1) as u16) as usize],
    );
    machine.pc += 2;
}
