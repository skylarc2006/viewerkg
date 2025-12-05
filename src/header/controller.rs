#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("Nonexistent Controller ID")]
    NonexistentControllerID,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Controller {
    WiiWheel,
    Nunchuk,
    Classic,
    Gamecube,
}

impl TryFrom<u8> for Controller {
    type Error = ControllerError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::WiiWheel),
            1 => Ok(Self::Nunchuk),
            2 => Ok(Self::Classic),
            3 => Ok(Self::Gamecube),
            _ => Err(ControllerError::NonexistentControllerID),
        }
    }
}

impl TryFrom<&mut bitreader::BitReader<'_>> for Controller {
    type Error = ControllerError;
    fn try_from(value: &mut bitreader::BitReader) -> Result<Self, Self::Error> {
        value.read_u8(4)?.try_into()
    }
}
