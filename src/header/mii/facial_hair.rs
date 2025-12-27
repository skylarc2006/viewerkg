use std::convert::Infallible;

use crate::{
    byte_handler::{ByteHandlerError, FromByteHandler},
    header::mii::hair::HairColor,
};

pub struct FacialHair {
    beard_type: BeardType,
    mustache_type: MustacheType,
    color: HairColor,
    mustache_size: u8,
    mustache_y: u8,
}
impl FacialHair {
    pub fn beard_type(&self) -> BeardType {
        self.beard_type
    }
    pub fn mustache_type(&self) -> MustacheType {
        self.mustache_type
    }
    pub fn color(&self) -> HairColor {
        self.color
    }
    pub fn mustache_size(&self) -> u8 {
        self.mustache_size
    }
    pub fn mustache_y(&self) -> u8 {
        self.mustache_y
    }
}

impl FromByteHandler for FacialHair {
    type Err = FacialHairError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;
        let mustache_y = handler.copy_byte(1) & 0x1F;
        handler.shift_right(1);
        Ok(Self {
            mustache_y,
            mustache_size: handler.copy_byte(1) >> 4,
            color: HairColor::try_from(handler.copy_byte(0) & 0x07)
                .map_err(|_| FacialHairError::ColorInvalid)?,
            mustache_type: MustacheType::try_from(handler.copy_byte(0) >> 5)
                .map_err(|_| FacialHairError::MustacheTypeInvalid)?,
            beard_type: BeardType::try_from((handler.copy_byte(0) >> 3) & 0x03)
                .map_err(|_| FacialHairError::BeardTypeInvalid)?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FacialHairError {
    #[error("Beard Type is invalid")]
    BeardTypeInvalid,
    #[error("Mustache Type is invalid")]
    MustacheTypeInvalid,
    #[error("Color is invalid")]
    ColorInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

#[derive(Clone, Copy,PartialEq,Debug)]
pub enum BeardType {
    None,
    Goatee,
    GoateeLong,
    LionsManeLong,
}
impl TryFrom<u8> for BeardType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Goatee),
            2 => Ok(Self::GoateeLong),
            3 => Ok(Self::LionsManeLong),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy,PartialEq,Debug)]
pub enum MustacheType {
    None,
    Walrus,
    Pencil,
    Horseshoe,
}
impl TryFrom<u8> for MustacheType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Walrus),
            2 => Ok(Self::Pencil),
            3 => Ok(Self::Horseshoe),
            _ => Err(()),
        }
    }
}
