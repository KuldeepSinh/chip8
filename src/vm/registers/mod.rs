//! CHIP-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit from 0 through F.
//! The VF register doubles as a flag for some instructions; thus, it should be avoided.
//! In an addition operation, VF is the carry flag, while in subtraction, it is the "no borrow" flag.
//! In the draw instruction VF is set upon pixel collision.

#[derive(Debug)]
pub struct Registers {
    pub v: Vec<u8>,
}

impl Registers {
    /// `Registers::new()` will return new Reigsters with all the v[16] registers initialized to zero.
    pub fn new() -> Registers {
        Registers { v: vec![0; 16] }
    }
}
