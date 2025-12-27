use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(Clone, Copy)]
pub struct Glasses {
    y: u8,
    size: u8,
    glasses_type: GlassesType,
    glasses_color: GlassesColor,
}

impl Glasses {
    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn size(&self) -> u8 {
        self.size
    }
    pub fn glasses_type(&self) -> GlassesType {
        self.glasses_type
    }
    pub fn glasses_color(&self) -> GlassesColor {
        self.glasses_color
    }
}

impl FromByteHandler for Glasses {
    type Err = GlassesError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;

        let glasses_type = GlassesType::try_from(handler.copy_byte(0) >> 4)
            .map_err(|_| GlassesError::TypeInvalid)?;
        let y = handler.copy_byte(1) & 0x1F;
        handler.shift_right(1);
        let glasses_color = GlassesColor::try_from(handler.copy_byte(0) & 0x03)
            .map_err(|_| GlassesError::ColorInvalid)?;
        let size = handler.copy_byte(1) >> 4;

        Ok(Self {
            glasses_type,
            y,
            glasses_color,
            size,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GlassesError {
    #[error("Type is invalid")]
    TypeInvalid,
    #[error("Color is invalid")]
    ColorInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

#[derive(Clone, Copy,PartialEq,Debug)]
pub enum GlassesColor {
    Black,
    Brown,
    Red,
    Blue,
    Yellow,
    Gray,
}
impl TryFrom<u8> for GlassesColor {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Black),
            1 => Ok(Self::Brown),
            2 => Ok(Self::Red),
            3 => Ok(Self::Blue),
            4 => Ok(Self::Yellow),
            5 => Ok(Self::Gray),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy,PartialEq,Debug)]
pub enum GlassesType {
    None,
    Square,
    Rectangle,
    Rounded,
    Oval,
    CatEye,
    SemiOpaqueAviator,
    SemiOpaqueRectangle,
    SemiOpaqueCatEye,
}

impl TryFrom<u8> for GlassesType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Square),
            2 => Ok(Self::Rectangle),
            3 => Ok(Self::Rounded),
            4 => Ok(Self::Oval),
            5 => Ok(Self::CatEye),
            6 => Ok(Self::SemiOpaqueAviator),
            7 => Ok(Self::SemiOpaqueRectangle),
            8 => Ok(Self::SemiOpaqueCatEye),
            _ => Err(()),
        }
    }
}
