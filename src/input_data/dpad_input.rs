#[derive(thiserror::Error, Debug)]
pub enum DPadButtonError {
    #[error("Non Existent DPad Button")]
    NonExistentDPadButton,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPadButton {
    None,
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_dpad_button(value: u8) -> Result<DPadButton, DPadButtonError> {
    let button = (value & 0x70) >> 4;

    match button {
        0 => Ok(DPadButton::None),
        1 => Ok(DPadButton::Up),
        2 => Ok(DPadButton::Down),
        3 => Ok(DPadButton::Left),
        4 => Ok(DPadButton::Right),
        _ => Err(DPadButtonError::NonExistentDPadButton),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DPadInputError {
    #[error("Invalid DPad Input")]
    InvalidDpadInput,
    #[error("Invalid DPad Button: {0}")]
    InvalidButton(#[from] DPadButtonError),
}

#[derive(Debug)]
pub struct DPadInput {
    button: DPadButton,
    frame_duration: u32,
}

impl DPadInput {
    pub fn button(&self) -> DPadButton {
        self.button
    }

    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }
}

impl PartialEq for DPadInput {
    fn eq(&self, other: &DPadInput) -> bool {
        self.button == other.button
    }
}
impl TryFrom<&[u8]> for DPadInput {
    type Error = DPadInputError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let button = parse_dpad_button(value[0])?;
        // 0x0F bit mask used if this input state was held longer than 255 frames, gives amount of 256-frame intervals to add to frame duration
        let previous_full_byte_presses: u32 = (value[0] & 0x0F).into();
        let frame_duration = value[1] as u32 + (previous_full_byte_presses * 256);

        Ok(Self {
            button,
            frame_duration,
        })
    }
}
