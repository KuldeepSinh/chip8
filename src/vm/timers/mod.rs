//! CHIP-8 has 2 8-bit timer registers, named delay timer (dt) and sound timer (st). When these timers are non-zero,
//! they are automatically decremented at a rate of 60Hz.
//! Delay timer: This timer is intended to be used for timing the events of games. Its value can be set and read.
//! Sound timer: This timer is used for sound effects. When its value is nonzero, a beeping sound is made.

#[derive(Debug)]
pub struct Timers {
    /// `dt` is a delay timer. When the timer is non-zero it will automatically decrement at the rate of 60 Hz.
    /// Delay timer: This timer is intended to be used for timing the events of games. Its value can be set and read.
    pub dt: u8,
    /// `st` is a sound timer. When the timer is non-zero it will automatically decrement at the rate of 60 Hz.
    /// Sound timer: This timer is used for sound effects. When its value is nonzero, a beeping sound is made.
    pub st: u8,
}

impl Timers {
    /// `Timers::new()` will return new Timers with both delay timer (dt) and sound timer (st) initialized to zero.
    pub fn new() -> Timers {
        Timers { dt: 0, st: 0 }
    }
}
