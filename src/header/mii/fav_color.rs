use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(thiserror::Error, Debug)]
pub enum FavColorError {
    #[error("Color is invalid")]
    ColorInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FavColor {
    Red,
    Orange,
    Yellow,
    LimeGreen,
    ForestGreen,
    RoyalBlue,
    SkyBlue,
    Pink,
    Purple,
    Brown,
    White,
    Black,
}

impl TryFrom<u8> for FavColor {
    type Error = FavColorError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Red),
            1 => Ok(Self::Orange),
            2 => Ok(Self::Yellow),
            3 => Ok(Self::LimeGreen),
            4 => Ok(Self::ForestGreen),
            5 => Ok(Self::RoyalBlue),
            6 => Ok(Self::SkyBlue),
            7 => Ok(Self::Pink),
            8 => Ok(Self::Purple),
            9 => Ok(Self::Brown),
            10 => Ok(Self::White),
            11 => Ok(Self::Black),
            _ => Err(FavColorError::ColorInvalid),
        }
    }
}

impl From<FavColor> for u8 {
    fn from(value: FavColor) -> Self {
        match value {
            FavColor::Red => 0,
            FavColor::Orange => 1,
            FavColor::Yellow => 2,
            FavColor::LimeGreen => 3,
            FavColor::ForestGreen => 4,
            FavColor::RoyalBlue => 5,
            FavColor::SkyBlue => 6,
            FavColor::Pink => 7,
            FavColor::Purple => 8,
            FavColor::Brown => 9,
            FavColor::White => 10,
            FavColor::Black => 11,
        }
    }
}

impl FromByteHandler for FavColor {
    type Err = FavColorError;

    /// Expects byte 0x01
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let handler = handler.try_into()?;
        Self::try_from((handler.copy_byte(0) >> 1) & 0x0F)
    }
}
