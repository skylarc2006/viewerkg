pub mod face_input;

pub struct InputData {
    face_button_count: u16,
    direction_count: u16,
    trick_count: u16,
    // ...
}

impl InputData {
    pub fn face_button_count(&self) -> u16 {
        self.face_button_count
    }

    pub fn direction_count(&self) -> u16 {
        self.direction_count
    }

    pub fn trick_count(&self) -> u16 {
        self.trick_count
    }
}
