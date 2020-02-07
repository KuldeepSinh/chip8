//! CHIP-8 Memory (RAM) size is 4,096 bytes (4-KB). Memory locations are numbered from 0x000 (0) to 0xFFF (4095).
//! The first 512 bytes, from 0x000 to 0x1FF, are used for the CHIP-8 interpreter itself and should not be used by programs.
//! Most CHIP-8 programs start ad 0x2000 (512).

/// MEMORY_SIZE constant (= 4096) represents size of the CHIP-8 memory (RAM).
const MEMORY_SIZE: usize = 4096 as usize;

/// `Memory` : CHIP-8 momory size is 4,096 bytes (4-KB). Memory locations are numbered from 0x000 (0) to 0xFFF (4095).
/// We will call each location as a cell.
pub struct Memory {
    /// Memory is an array of `u8` type of `cells`, with length = 4096 bytes. Each `cell` can be accessed by their index value,
    /// starting from 0 and ending with 4095.
    pub cells: Vec<u8>,
}

impl Memory {
    /// `Memory::new()` will return a new memory with each cell initialized to 0, Index register `i` set to 0 and
    /// Program Counter `pc` set to 0x200 (512).
    pub fn new() -> Memory {
        let mut memory = Memory {
            cells: vec![0; MEMORY_SIZE],
        };
        memory.set_fonts();
        memory
    }

    /// `Memory.set_fonts()` populates fontset in the memory.
    /// Mainly, it loads predefined FontSet (graphics::Fontset), into its first 80 cells.
    fn set_fonts(&mut self) {
        //get fontset
        use super::graphics::fontset;
        let font_set = fontset::FontSet::new();
        //update memory cells with the font set
        for (i, &font) in font_set.fonts.iter().enumerate() {
            self.cells[i] = font;
        }
    }
}
