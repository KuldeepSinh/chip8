//! The `VRAM` of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32).
//! This can easily be implemented using a vector that hold the pixel state (1 or 0).

/// VRAM_SIZE constant ( = 2048) represents size of the CHIP-8 VRAM unit.
const VRAM_WIDTH: usize = 64;
const VRAM_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct VRam {
    pub cells: Vec<Vec<u8>>,
}

impl VRam {
    /// `VRam::new()` will return new VRAM with 2048 (64 x 32) cellls, all initialized to zero.
    pub fn new() -> VRam {
        VRam {
            cells: vec![vec![0; VRAM_WIDTH]; VRAM_HEIGHT],
        }
    }
}
