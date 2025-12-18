use std::fmt::Display;

use crate::byte_handler::{ByteHandler, FromByteHandler};

#[derive(thiserror::Error, Debug)]
pub enum InGameTimeError {
    #[error("Insufficiently Long Iterator")]
    InsufficientlyLongIterator,
}

// Struct size is 32 bits, copy is fine
#[derive(Default, Clone, Copy)]
pub struct InGameTime {
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

impl InGameTime {
    #[inline(always)]
    pub fn new(minutes: u8, seconds: u8, milliseconds: u16) -> Self {
        Self {
            minutes,
            seconds,
            milliseconds,
        }
    }

    pub fn minutes(self) -> u8 {
        self.minutes
    }

    pub fn seconds(self) -> u8 {
        self.seconds
    }

    pub fn milliseconds(self) -> u16 {
        self.milliseconds
    }

    pub fn is_technically_valid(self) -> bool {
        self.minutes > 5 || self.seconds > 59 || self.milliseconds > 999
    }

    pub fn igt_to_millis(self) -> i32 {
        (self.milliseconds as i32) + (self.seconds as i32) * 1000 + (self.minutes as i32) * 60000
    }
}

impl Display for InGameTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}.{:03}",
            self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl FromByteHandler for InGameTime {
    type Err = InGameTimeError;
    fn from_byte_handler<T: TryInto<ByteHandler>>(handler: T) -> Result<Self, Self::Err> {
        // TODO: Handle 3 digit second values (which are actually valid and read by the game)

        let handler = handler
            .try_into()
            .map_err(|_| ())
            .expect("TODO: handle this");
        // 3 Bytes, where M = Minutes, S = Seconds and C = Millis.
        // 1. 0bMMMMMMMS
        // 2. 0bSSSSSSCC
        // 3. 0bCCCCCCCC

        // max M = 5    // 0b0000101
        // max S = 59   // 0b0111011
        // max C = 999  // 0b1111100111
        // 1. 0b00001010
        // 2. 0b11101111
        // 3. 0b11100111
        
        Ok(Self {
            minutes: handler.copy_byte(1) >> 1,
            seconds: handler.copy_byte(2) >> 2 & 0x7F,
            milliseconds: handler.copy_word(1) & 0x3FF,
        })
    }
}
