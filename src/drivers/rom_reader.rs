use log::debug;
use std::fs::File;
use std::io::prelude::*;

pub struct RomReader {
    pub rom: [u8; 3584],
    pub size: usize,
}

impl RomReader {
    pub fn new(filename: &str) -> Self {
        debug!("[RomReader::new()] Going to read {}.", filename);
        let mut f = File::open(filename).expect("file not found");
        let mut rom = [0u8; 3584];

        let bytes_read = match f.read(&mut rom) {
            Ok(bytes_read) => {
                debug!("[RomReader::new()] bytes read = {}.", bytes_read);
                bytes_read
            }
            _ => {
                debug!("[RomReader::new()] Not able to read {}.", filename);
                0
            }
        };
        RomReader {
            rom: rom,
            size: bytes_read,
        }
    }
}
