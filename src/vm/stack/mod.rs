//! In CHIP-8, instruction set has opcodes that allow the program to jump to a certain address or call a subroutine.
//! The stack is used to remember the current location before a jump is performed.
//! So anytime you perform a jump or call a subroutine, store the program counter (PC) in the stack before proceeding.
//! The system has 16 levels of stack and in order to remember which level of the stack is used,
//! you need to implement a stack pointer (sp).

#[derive(Debug)]
pub struct Stack {
    pub cells: Vec<u16>,
}

impl Stack {
    /// `Stack::new()` will return new Stack with 16 cellls, all initialized to zero.
    pub fn new() -> Stack {
        Stack { cells: vec![0; 16] }
    }
}
