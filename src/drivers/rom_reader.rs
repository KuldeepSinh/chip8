use log::debug;
use std::fs::File;
use std::io::prelude::*;

pub struct RomReader {
    pub rom: [u8; 3584],
    pub size: usize,
}

impl RomReader {
    pub fn new() -> Self {
        RomReader {
            size: 0,
            rom: [0u8; 3584],
        }
    }

    pub fn read_rom(&mut self, filename: &str) {
        debug!("[RomReader.read_rom()] Going to read {}.", filename);
        let mut f = File::open(filename).expect("file not found");
        self.size = match f.read(&mut self.rom) {
            Ok(bytes_read) => {
                debug!("[RomReader::new()] bytes read = {}.", bytes_read);
                bytes_read
            }
            _ => {
                debug!("[RomReader::new()] Not able to read {}.", filename);
                0
            }
        };
    }
}
