//! The `VRAM` of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32).
//! This can easily be implemented using a vector that hold the pixel state (1 or 0).

#[derive(Debug)]
pub struct VRam {
    pub cells: Vec<Vec<u8>>,
    pub state_changed: bool,
}

impl VRam {
    /// `VRam::new()` will return new VRAM with 2048 (64 x 32) cellls, all initialized to zero.
    pub fn new(width: usize, height: usize) -> VRam {
        VRam {
            cells: vec![vec![0; width]; height],
            state_changed: false,
        }
    }
}
