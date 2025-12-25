#[derive(thiserror::Error, Debug)]
pub enum ByteHandlerError {
    #[error("Couldn't convert type to ByteHandler: Too Long")]
    ConversionErrorTooLong,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union ByteHandler {
    dword: u32,
    words: [u16; 2],
    bytes: [u8; 4],
}

impl ByteHandler {
    pub const fn copy_dword(self) -> u32 {
        unsafe { self.dword }
    }

    pub const fn copy_word(self, idx: usize) -> u16 {
        if idx > 1 {
            return 0;
        }

        #[cfg(target_endian = "little")]
        let idx = 1 - idx;

        unsafe { self.words[idx] }
    }

    pub const fn copy_byte(self, idx: usize) -> u8 {
        if idx > 3 {
            return 0;
        }

        #[cfg(target_endian = "little")]
        let idx = 3 - idx;

        unsafe { self.bytes[idx] }
    }

    pub const fn shift_right(&mut self, d: u8) {
        unsafe {
            self.dword >>= d;
        }
    }

    pub const fn shift_left(&mut self, d: u8) {
        unsafe {
            self.dword <<= d;
        }
    }

    /// Reads the nth bit from the right counting from 0
    pub const fn read_bool(&self, d: u8) -> bool {
        if d >= 32 {
            return false;
        }

        #[cfg(target_endian = "little")]
        return (unsafe { self.dword } & (1u32 << ((d % 8) + 8 * (3 - (d / 8))))) != 0;

        #[cfg(target_endian = "big")]
        return (unsafe { self.dword } & (1u32 << d)) != 0;
    }
}

impl From<[u8; 4]> for ByteHandler {
    fn from(value: [u8; 4]) -> Self {
        ByteHandler {
            #[cfg(target_endian = "big")]
            bytes: [value[0], value[1], value[2], value[3]],
            #[cfg(target_endian = "little")]
            bytes: [value[3], value[2], value[1], value[0]],
        }
    }
}

impl From<[u8; 3]> for ByteHandler {
    fn from(value: [u8; 3]) -> Self {
        ByteHandler {
            #[cfg(target_endian = "big")]
            bytes: [value[0], value[1], value[2], 0],
            #[cfg(target_endian = "little")]
            bytes: [0, value[2], value[1], value[0]],
        }
    }
}

impl From<[u8; 2]> for ByteHandler {
    fn from(value: [u8; 2]) -> Self {
        ByteHandler {
            #[cfg(target_endian = "big")]
            bytes: [value[0], value[1], 0, 0],
            #[cfg(target_endian = "little")]
            bytes: [0, 0, value[1], value[0]],
        }
    }
}

impl From<u8> for ByteHandler {
    fn from(value: u8) -> Self {
        ByteHandler {
            #[cfg(target_endian = "big")]
            bytes: [value, 0, 0, 0],
            #[cfg(target_endian = "little")]
            bytes: [0, 0, 0, value],
        }
    }
}

impl From<u32> for ByteHandler {
    fn from(value: u32) -> Self {
        ByteHandler { dword: value }
    }
}

impl From<[u16; 2]> for ByteHandler {
    fn from(value: [u16; 2]) -> Self {
        ByteHandler {
            #[cfg(target_endian = "big")]
            words: [value[0], value[1]],
            #[cfg(target_endian = "little")]
            words: [value[1], value[0]],
        }
    }
}

impl From<u16> for ByteHandler {
    fn from(value: u16) -> Self {
        ByteHandler {
            #[cfg(target_endian = "big")]
            words: [value, 0],
            #[cfg(target_endian = "little")]
            words: [0, value],
        }
    }
}

macro_rules! shorten_syntax {
    ($num:literal $value:ident $type:ty) => {
        Ok(From::from(unsafe {
            TryInto::<[$type; $num]>::try_into($value).unwrap_unchecked()
        }))
    };
}

impl TryFrom<&[u8]> for ByteHandler {
    type Error = ByteHandlerError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Ok(From::from(0u32)),
            1 => Ok(From::from(value[0])),
            2 => shorten_syntax!(2 value u8),
            3 => shorten_syntax!(3 value u8),
            4 => shorten_syntax!(4 value u8),
            _ => Err(ByteHandlerError::ConversionErrorTooLong),
        }
    }
}

impl TryFrom<&[u16]> for ByteHandler {
    type Error = ByteHandlerError;

    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Ok(From::from(0u32)),
            1 => Ok(From::from(value[0])),
            2 => shorten_syntax!(2 value u16),
            _ => Err(ByteHandlerError::ConversionErrorTooLong),
        }
    }
}

pub(crate) trait FromByteHandler: Sized {
    type Err;

    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<ByteHandler>,
        Self::Err: From<T::Error>;
}
