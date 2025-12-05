#[derive(thiserror::Error, Debug)]
pub enum GhostTypeError {
    #[error("Nonexistent Ghost Type")]
    NonexistentGhostType,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GhostType {
    PlayerBest,
    WorldRecord,
    ContinentalRecord,
    Rival,
    Special,
    GhostRace,
    Friend,
    NormalStaff,
    ExpertStaff,
}

impl TryFrom<u8> for GhostType {
    type Error = GhostTypeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::PlayerBest),
            0x02 => Ok(Self::WorldRecord),
            0x03 => Ok(Self::ContinentalRecord),
            0x04 => Ok(Self::Rival),
            0x05 => Ok(Self::Special),
            0x06 => Ok(Self::GhostRace),
            0x07..=0x24 => Ok(Self::Friend),
            0x25 => Ok(Self::NormalStaff),
            0x26 => Ok(Self::ExpertStaff),
            _ => Err(GhostTypeError::NonexistentGhostType),
        }
    }
}

impl From<GhostType> for u8 {
    fn from(value: GhostType) -> Self {
        match value {
            GhostType::PlayerBest => 0x01,
            GhostType::WorldRecord => 0x02,
            GhostType::ContinentalRecord => 0x03,
            GhostType::Rival => 0x04,
            GhostType::Special => 0x05,
            GhostType::GhostRace => 0x06,
            GhostType::Friend => 0x24,
            GhostType::NormalStaff => 0x25,
            GhostType::ExpertStaff => 0x26,
        }
    }
}

impl TryFrom<&mut bitreader::BitReader<'_>> for GhostType {
    type Error = GhostTypeError;
    fn try_from(value: &mut bitreader::BitReader) -> Result<Self, Self::Error> {
        value.read_u8(7)?.try_into()
    }
}
