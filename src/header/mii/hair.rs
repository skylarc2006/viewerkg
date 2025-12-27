use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(Clone, Copy)]
pub struct Hair {
    hair_type: HairType,
    hair_color: HairColor,
    is_flipped: bool,
}

impl Hair {
    pub fn hair_type(&self) -> HairType {
        self.hair_type
    }
    pub fn hair_color(&self) -> HairColor {
        self.hair_color
    }
    pub fn is_flipped(&self) -> bool {
        self.is_flipped
    }
}

#[derive(thiserror::Error, Debug)]
pub enum HairError {
    #[error("Type is invalid")]
    TypeInvalid,
    #[error("Color is invalid")]
    ColorInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

impl FromByteHandler for Hair {
    type Err = HairError;
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;
        handler.shift_right(1);
        Ok(Self {
            is_flipped: handler.read_bool(4),
            hair_type: HairType::try_from(handler.copy_byte(0))
                .map_err(|_| HairError::TypeInvalid)?,
            hair_color: HairColor::try_from(handler.copy_byte(1) >> 5)
                .map_err(|_| HairError::ColorInvalid)?,
        })
    }
}

#[derive(Clone, Copy, PartialEq,Debug)]
pub enum HairColor {
    Black,
    Chocolate,
    PhilippineBrown,
    Walnut,
    Gray,
    Pineapple,
    Grizzly,
    Blond,
}

impl TryFrom<u8> for HairColor {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Black),
            0x01 => Ok(Self::Chocolate),
            0x02 => Ok(Self::PhilippineBrown),
            0x03 => Ok(Self::Walnut),
            0x04 => Ok(Self::Gray),
            0x05 => Ok(Self::Pineapple),
            0x06 => Ok(Self::Grizzly),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HairType {
    NormalLong,
    NormalMedium,
    FrontLock,
    PartingExtraLong,
    MilitaryParting,
    PartingExtraLongCurved,
    ShortUnknown3,
    PeaksSquared,
    ShortUnknown5,
    Peaks,
    PeaksRounded,
    PeaksLongBottom,
    NormalLongBottom,
    NormalShort,
    NormalExtraLong,
    PartingLong,
    PartingMiddleLong,
    PartingSquared,
    LongRounded,
    PartingLongBottom,
    PartingShort,
    PartingFrontPeaks,
    NormalUnknown1,
    PeaksSide,
    PartingPeaks,
    PeaksTop,
    DreadLocks,
    Short,
    ShortUnknown4,
    Afro,
    Military,
    NoneTop,
    ShortUnknown6,
    None,
    Caps,
    Beanie,
    LongUnknown1,
    LongUnknown40,
    LongUnknown38,
    LongUnknown60,
    LongUnknown16,
    LongUnknown36,
    LongUnknown56,
    PartingFrontTwoLongBackPonyTails,
    LongUnknown31,
    LongUnknown20,
    LongUnknown15,
    LongUnknown52,
    LongUnknown7,
    LongUnknown23,
    PartingExtraLongRounded,
    LongUnknown3,
    LongUnknown11,
    LongUnknown12,
    LongUnknown29,
    LongUnknown27,
    LongUnknown17,
    LongUnknown39,
    LongUnknown24,
    LongUnknown25,
    LongUnknown61,
    LongUnknown2,
    StrandsTwoShortSidedPonyTails,
    TwoFrontStrandsLongBackPonyTail,
    LongUnknown65,
    LongUnknown63,
    ShortFrontTwoBackPonyTails,
    LongUnknown43,
    LongUnknown47,
    LongUnknown44,
    LongUnknown53,
    LongUnknown51,
}

impl TryFrom<u8> for HairType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x21 => Ok(Self::NormalLong),
            0x28 => Ok(Self::NormalMedium),
            0x33 => Ok(Self::FrontLock),
            0x2c => Ok(Self::PartingExtraLong),
            0x27 => Ok(Self::MilitaryParting),
            0x46 => Ok(Self::PartingExtraLongCurved),
            0x2d => Ok(Self::ShortUnknown3),
            0x31 => Ok(Self::PeaksSquared),
            0x3b => Ok(Self::ShortUnknown5),
            0x38 => Ok(Self::Peaks),
            0x44 => Ok(Self::PeaksRounded),
            0x1f => Ok(Self::PeaksLongBottom),
            0x20 => Ok(Self::NormalLongBottom),
            0x2f => Ok(Self::NormalShort),
            0x25 => Ok(Self::NormalExtraLong),
            0x30 => Ok(Self::PartingLong),
            0x42 => Ok(Self::PartingMiddleLong),
            0x34 => Ok(Self::PartingSquared),
            0x3a => Ok(Self::LongRounded),
            0x32 => Ok(Self::PartingLongBottom),
            0x37 => Ok(Self::PartingShort),
            0x40 => Ok(Self::PartingFrontPeaks),
            0x3c => Ok(Self::NormalUnknown1),
            0x3e => Ok(Self::PeaksSide),
            0x2b => Ok(Self::PartingPeaks),
            0x26 => Ok(Self::PeaksTop),
            0x2a => Ok(Self::DreadLocks),
            0x17 => Ok(Self::Short),
            0x43 => Ok(Self::ShortUnknown4),
            0x36 => Ok(Self::Afro),
            0x24 => Ok(Self::Military),
            0x29 => Ok(Self::NoneTop),
            0x41 => Ok(Self::ShortUnknown6),
            0x1e => Ok(Self::None),
            0x39 => Ok(Self::Caps),
            0x22 => Ok(Self::Beanie),
            0x0c => Ok(Self::LongUnknown1),
            0x0d => Ok(Self::LongUnknown40),
            0x45 => Ok(Self::LongUnknown38),
            0x1a => Ok(Self::LongUnknown60),
            0x04 => Ok(Self::LongUnknown16),
            0x19 => Ok(Self::LongUnknown36),
            0x01 => Ok(Self::LongUnknown56),
            0x13 => Ok(Self::PartingFrontTwoLongBackPonyTails),
            0x05 => Ok(Self::LongUnknown31),
            0x08 => Ok(Self::LongUnknown20),
            0x1b => Ok(Self::LongUnknown15),
            0x07 => Ok(Self::LongUnknown52),
            0x0e => Ok(Self::LongUnknown7),
            0x03 => Ok(Self::LongUnknown23),
            0x16 => Ok(Self::PartingExtraLongRounded),
            0x0a => Ok(Self::LongUnknown3),
            0x06 => Ok(Self::LongUnknown11),
            0x14 => Ok(Self::LongUnknown12),
            0x0b => Ok(Self::LongUnknown29),
            0x3f => Ok(Self::LongUnknown27),
            0x11 => Ok(Self::LongUnknown17),
            0x23 => Ok(Self::LongUnknown39),
            0x15 => Ok(Self::LongUnknown24),
            0x00 => Ok(Self::LongUnknown25),
            0x3d => Ok(Self::LongUnknown61),
            0x10 => Ok(Self::LongUnknown2),
            0x2e => Ok(Self::StrandsTwoShortSidedPonyTails),
            0x09 => Ok(Self::TwoFrontStrandsLongBackPonyTail),
            0x12 => Ok(Self::LongUnknown65),
            0x02 => Ok(Self::LongUnknown63),
            0x1c => Ok(Self::ShortFrontTwoBackPonyTails),
            0x35 => Ok(Self::LongUnknown43),
            0x47 => Ok(Self::LongUnknown47),
            0x18 => Ok(Self::LongUnknown44),
            0x0f => Ok(Self::LongUnknown53),
            0x1d => Ok(Self::LongUnknown51),
            _ => Err(()),
        }
    }
}
