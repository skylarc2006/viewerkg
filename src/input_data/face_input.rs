#[derive(thiserror::Error, Debug)]
pub enum FaceButtonError {
    #[error("Non Existent Face Button")]
    NonExistentFaceButton,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FaceButton {
    Accelerator,
    Brake,
    Item,
    BrakeWhileAcceleratorHeld,
    Unknown,
}

impl From<FaceButton> for u16 {
    fn from(value: FaceButton) -> u16 {
        match value {
            FaceButton::Accelerator => 0x01,
            FaceButton::Brake => 0x02,
            FaceButton::Item => 0x04,
            FaceButton::BrakeWhileAcceleratorHeld => 0x08,
            FaceButton::Unknown => 0xF0,
        }
    }
}

impl TryFrom<u8> for FaceButton {
    type Error = FaceButtonError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(FaceButton::Accelerator),
            0x02 => Ok(FaceButton::Brake),
            0x04 => Ok(FaceButton::Item),
            0x08 => Ok(FaceButton::BrakeWhileAcceleratorHeld),
            0xF0 => Ok(FaceButton::Unknown),
            _ => Err(FaceButtonError::NonExistentFaceButton),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FaceInputError {
    #[error("Invalid Face Input")]
    InvalidFaceInput,
    #[error("Invalid Face Button: {0}")]
    InvalidButton(#[from] FaceButtonError),
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}
pub struct FaceInput {
    button: FaceButton,
    frame_duration: u8
}

impl FaceInput {
    pub fn button(&self) -> FaceButton {
        self.button
    }
    
    pub fn frame_duration(&self) -> u8 {
        self.frame_duration
    }
}

impl TryFrom<u16> for FaceInput {
    type Error = FaceInputError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let bytes = value.to_be_bytes();
        let button = FaceButton::try_from(bytes[0])?;
        let frame_duration = bytes[1];
        Ok(Self {
            button,
            frame_duration
        })
    }
}

impl TryFrom<&mut bitreader::BitReader<'_>> for FaceInput {
    type Error = FaceInputError;
    fn try_from(value: &mut bitreader::BitReader<'_>) -> Result<Self, Self::Error> {
        FaceInput::try_from(value.read_u16(16)?)
    }
}
