#[derive(thiserror::Error, Debug)]
pub enum StickInputError {
    #[error("Invalid Stick Input")]
    InvalidStickInput,
}

#[derive(Debug)]
pub struct StickInput {
    x: i8,
    y: i8,
    frame_duration: u32,
}

impl StickInput {
    pub fn x(&self) -> i8 {
        self.x
    }

    pub fn y(&self) -> i8 {
        self.y
    }

    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }

    pub fn set_frame_duration(&mut self, frame_duration: u32) {
        self.frame_duration = frame_duration;
    }
}

impl PartialEq for StickInput {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl TryFrom<&[u8]> for StickInput {
    type Error = StickInputError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        let x = (value[0] & 0xF0) >> 4;
        let y = value[0] & 0x0F;

        if x > 14 || y > 14 {
            return Err(StickInputError::InvalidStickInput);
        }

        // store x and y as ranging from -7 to +7, as that's more intuitive for left/right or up/down
        let x = x as i8 - 7;
        let y = y as i8 - 7;

        let frame_duration = value[1] as u32;

        Ok(Self {
            x,
            y,
            frame_duration,
        })
    }
}
