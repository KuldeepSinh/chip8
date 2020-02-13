//! CHIP-8 has a HEX based keypad (0x0-0xF). It contains 16 keys.

/// `KeyBoard` : CHIP-8 has a HEX based keypad (0x0-0xF). It contains 16 keys.
pub struct KeyBoard {
    pub keys: Vec<bool>,
    pub keypress_awaited: bool,
    pub key_register: usize,
}

impl KeyBoard {
    /// `KeyBoard::new()` will return a new keyboard with each key initialized to false, indicating all the keys are UP.
    pub fn new() -> KeyBoard {
        KeyBoard {
            keys: vec![false; 16],
            keypress_awaited: false,
            key_register: 0,
        }
    }
    /// `KeyBoard.key_down()` will set key value to true, indicating the key is DOWN.
    pub fn key_down(&mut self, index: u8) {
        self.keys[index as usize] = true;
    }
    /// `KeyBoard.key_up()` will set key value to false, indicating the key is UP.
    pub fn key_up(&mut self, index: u8) {
        self.keys[index as usize] = false;
    }
    ///`KeyBoard.is_key_down()` will return value at the key-index
    pub fn is_key_down(&self, index: u8) -> bool {
        self.keys[index as usize]
    }
}
