//! The `display` of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32).
//! This can easily be implemented using a vector that hold the pixel state (1 or 0).

/// DISPLAY_SIZE constant ( = 2048) represents size of the CHIP-8 display unit.
const DISPLAY_SIZE: usize = 64 * 32 as usize;

#[derive(Debug)]
pub struct Display {
    pub cells: Vec<u8>,
}

impl Display {
    /// `Display::new()` will return new Display with 2048 (64 x 32) cellls, all initialized to zero.
    pub fn new() -> Display {
        Display {
            cells: vec![0; DISPLAY_SIZE],
        }
    }
}
