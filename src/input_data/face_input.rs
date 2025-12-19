#[derive(thiserror::Error, Debug)]
pub enum FaceButtonError {
    #[error("Non Existent Face Button")]
    NonExistentFaceButton,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FaceButton {
    Accelerator,
    Brake,
    Item,
    Unknown,
}

pub fn parse_face_buttons(value: u8) -> Result<Vec<FaceButton>, FaceButtonError> {
    let mut buttons = Vec::new();

    if value & 0x01 != 0 {
        buttons.push(FaceButton::Accelerator);
    }
    if value & 0x02 != 0 || value & 0x08 != 0 {
        buttons.push(FaceButton::Brake);
    }
    if value & 0x04 != 0 {
        buttons.push(FaceButton::Item);
    }
    // 0x40 is the CTGP pause mask and would trigger this otherwise
    if value & 0xF0 != 0 && value & 0x40 == 0 {
        buttons.push(FaceButton::Unknown);
    }

    if value != 0x00 && value & 0x40 == 0 && buttons.is_empty() {
        return Err(FaceButtonError::NonExistentFaceButton);
    }

    Ok(buttons)
}

#[derive(thiserror::Error, Debug)]
pub enum FaceInputError {
    #[error("Invalid Face Input")]
    InvalidFaceInput,
    #[error("Invalid Face Button: {0}")]
    InvalidButton(#[from] FaceButtonError),
}

#[derive(Debug)]
pub struct FaceInput {
    buttons: Vec<FaceButton>,
    frame_duration: u32,
}

impl FaceInput {
    pub fn buttons(&self) -> &Vec<FaceButton> {
        &self.buttons
    }

    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }

    pub fn set_frame_duration(&mut self, frame_duration: u32) {
        self.frame_duration = frame_duration;
    }
}

impl PartialEq for FaceInput {
    fn eq(&self, other: &Self) -> bool {
        self.buttons == other.buttons
    }
}

impl TryFrom<&[u8]> for FaceInput {
    type Error = FaceInputError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buttons = parse_face_buttons(value[0])?;
        let frame_duration = value[1] as u32;

        Ok(Self {
            buttons,
            frame_duration,
        })
    }
}
