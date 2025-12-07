use crate::input_data::dpad_input::DPadButton;
use crate::input_data::face_input::FaceButton;

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    face_buttons: Vec<FaceButton>,
    stick_x: i8,
    stick_y: i8,
    dpad_button: DPadButton,
    frame_duration: u32,
}

impl Input {
    pub fn new(
        face_buttons: Vec<FaceButton>,
        stick_x: i8,
        stick_y: i8,
        dpad_button: DPadButton,
        frame_duration: u32,
    ) -> Self {
        Self {
            face_buttons,
            stick_x,
            stick_y,
            dpad_button,
            frame_duration,
        }
    }

    pub fn face_buttons(&self) -> &[FaceButton] {
        &self.face_buttons
    }

    pub fn stick_x(&self) -> i8 {
        self.stick_x
    }

    pub fn stick_y(&self) -> i8 {
        self.stick_y
    }

    pub fn dpad_button(&self) -> DPadButton {
        self.dpad_button
    }

    pub fn frame_duration(&self) -> u32 {
        self.frame_duration
    }
}
