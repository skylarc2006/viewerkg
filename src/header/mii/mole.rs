use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(Clone, Copy)]
pub struct Mole {
    has_mole: bool,
    x: u8,
    y: u8,
    size: u8,
}
impl Mole {
    pub fn has_mole(&self)->bool{self.has_mole}
    pub fn x(&self)->u8{self.x}
    pub fn y(&self)->u8{self.y}
    pub fn size(&self)->u8{self.size}
}
impl FromByteHandler for Mole {
    type Err = MoleError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
        where
            T: TryInto<crate::byte_handler::ByteHandler>,
            Self::Err: From<T::Error> {
        let mut handler = handler.try_into()?;
        let has_mole = handler.read_bool(15);
        handler.shift_right(1);
        let x = handler.copy_byte(1) & 0x1F;
        handler.shift_right(2);
        let y = handler.copy_byte(1)>>3;
        let size = handler.copy_byte(0)&0x0F;
        Ok(Self {has_mole,x,y,size})
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MoleError {
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}
