#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union ByteHandler {
    dword: u32,
    words: [u16; 2],
    bytes: [u8; 4],
}

impl ByteHandler {
    pub fn copy_dword(self) -> u32 {
        unsafe { u32::from_be_bytes(self.bytes) }
    }

    pub fn copy_words(self) -> [u16; 2] {
        unsafe {
            [
                u16::from_be_bytes([self.bytes[0], self.bytes[1]]),
                u16::from_be_bytes([self.bytes[2], self.bytes[3]]),
            ]
        }
    }

    pub fn copy_word(self, idx: usize) -> u16 {
        if idx > 1 {
            return 0;
        }
        unsafe {
            let byte_idx = idx * 2;
            u16::from_be_bytes([self.bytes[byte_idx], self.bytes[byte_idx + 1]])
        }
    }

    pub fn copy_bytes(self) -> [u8; 4] {
        unsafe { self.bytes }
    }

    pub fn copy_byte(self, idx: usize) -> u8 {
        if idx > 3 {
            return 0;
        }
        unsafe { self.bytes[idx] }
    }

    pub fn shift_right(&mut self, d: u8) {
        unsafe {
            let mut value = u32::from_be_bytes(self.bytes);
            value >>= d;
            self.bytes = value.to_be_bytes();
        }
    }

    pub fn shift_left(&mut self, d: u8) {
        unsafe {
            let mut value = u32::from_be_bytes(self.bytes);
            value <<= d;
            self.bytes = value.to_be_bytes();
        }
    }

    /// Reads the nth bit from the right counting from 0
    pub fn read_bool(&self, d: u8) -> bool {
        if d >= 32 {
            return false;
        }
        (self.copy_dword() & (1u32 << d)) != 0
    }
}

impl From<[u8; 4]> for ByteHandler {
    fn from(value: [u8; 4]) -> Self {
        ByteHandler { bytes: value }
    }
}

impl From<[u8; 3]> for ByteHandler {
    fn from(value: [u8; 3]) -> Self {
        ByteHandler { bytes: [0, value[0], value[1], value[2]] }
    }
}

impl From<[u8; 2]> for ByteHandler {
    fn from(value: [u8; 2]) -> Self {
        ByteHandler { bytes: [0, 0, value[0], value[1]] }
    }
}

impl From<u8> for ByteHandler {
    fn from(value: u8) -> Self {
        ByteHandler { bytes: [0, 0, 0, value] }
    }
}

impl From<u32> for ByteHandler {
    fn from(value: u32) -> Self {
        ByteHandler { bytes: value.to_be_bytes() }
    }
}

impl From<[u16; 2]> for ByteHandler {
    fn from(value: [u16; 2]) -> Self {
        let bytes: [u8; 4] = [
            value[0].to_be_bytes()[0],
            value[0].to_be_bytes()[1],
            value[1].to_be_bytes()[0],
            value[1].to_be_bytes()[1],
        ];
        ByteHandler { bytes }
    }
}

impl From<u16> for ByteHandler {
    fn from(value: u16) -> Self {
        ByteHandler::from([0_u16, value])
    }
}

impl TryFrom<&[u8]> for ByteHandler {
    type Error = ();
    
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Ok(ByteHandler::from(0u32)),
            1 => Ok(ByteHandler::from(value[0])),
            2 => Ok(ByteHandler::from([value[0], value[1]])),
            3 => Ok(ByteHandler::from([value[0], value[1], value[2]])),
            4 => Ok(ByteHandler::from([value[0], value[1], value[2], value[3]])),
            _ => Err(()),
        }
    }
}

impl TryFrom<&[u16]> for ByteHandler {
    type Error = ();
    
    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Ok(ByteHandler::from(0u32)),
            1 => Ok(ByteHandler::from(value[0])),
            2 => Ok(ByteHandler::from([value[0], value[1]])),
            _ => Err(()),
        }
    }
}

pub(crate) trait FromByteHandler: Sized {
    type Err;
    fn from_byte_handler<T: TryInto<ByteHandler>>(handler: T) -> Result<Self, Self::Err>;
}