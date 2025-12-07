// https://wiki.tockdom.com/wiki/RKG_(File_Format)#Controller_Input_Data

use bitreader::BitReader;

use crate::input_data::input::Input;
use crate::input_data::dpad_input::{DPadButton, DPadInput};
use crate::input_data::face_input::FaceInput;
use crate::input_data::stick_input::StickInput;

pub mod input;
pub mod dpad_input;
pub mod face_input;
pub mod stick_input;

#[derive(thiserror::Error, Debug)]
pub enum InputDataError {
    #[error("Face Input Error: {0}")]
    FaceInputError(#[from] face_input::FaceInputError),
    #[error("DPad Input Error: {0}")]
    DPadInputError(#[from] dpad_input::DPadInputError),
    #[error("Stick Input Error: {0}")]
    StickInputError(#[from] stick_input::StickInputError),
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

pub struct InputData {
    face_inputs: Vec<FaceInput>,
    stick_inputs: Vec<StickInput>,
    dpad_inputs: Vec<DPadInput>,
}

impl InputData {
    // Currently this only has the uncompressed input data structure in mind
    pub fn new(input_data: &[u8]) -> Result<Self, InputDataError> {
        // TODO: Determine/decompress compressed input data here

        let mut input_reader = BitReader::new(input_data);

        let face_input_count = input_reader.read_u16(16)?;
        let stick_input_count = input_reader.read_u16(16)?;
        let dpad_input_count = input_reader.read_u16(16)?;
        input_reader.skip(16)?; // padding

        let mut face_inputs: Vec<FaceInput> = Vec::with_capacity(face_input_count as usize);
        for _ in 0..face_input_count {
            face_inputs.push(FaceInput::try_from(&mut input_reader)?);
        }

        let mut stick_inputs: Vec<StickInput> = Vec::with_capacity(stick_input_count as usize);
        for _ in 0..stick_input_count {
            stick_inputs.push(StickInput::try_from(&mut input_reader)?);
        }

        let mut dpad_inputs: Vec<DPadInput> = Vec::with_capacity(dpad_input_count as usize);
        for _ in 0..dpad_input_count {
            dpad_inputs.push(DPadInput::try_from(&mut input_reader)?);
        }

        // Combine adjacent inputs when the same button is held across multiple bytes
        // (each input byte has a 255-frame limit, so buttons held longer need additional bytes)
        for index in (0..face_inputs.len() - 1).rev() {
            if face_inputs[index] == face_inputs[index + 1] {
                let f1 = face_inputs[index].frame_duration();
                let f2 = face_inputs[index + 1].frame_duration();
                face_inputs[index].set_frame_duration(f1 + f2);
                face_inputs.remove(index + 1);
            }
        }

        for index in (0..stick_inputs.len() - 1).rev() {
            if stick_inputs[index] == stick_inputs[index + 1] {
                let f1 = stick_inputs[index].frame_duration();
                let f2 = stick_inputs[index + 1].frame_duration();
                stick_inputs[index].set_frame_duration(f1 + f2);
                stick_inputs.remove(index + 1);
            }
        }

        Ok(Self {
            face_inputs,
            stick_inputs,
            dpad_inputs,
        })
    }

    pub fn inputs(&self) -> Vec<Input> {
        let mut result = Vec::new();

        // Track current position in each input stream
        let mut face_idx = 0;
        let mut stick_idx = 0;
        let mut dpad_idx = 0;

        // Track how many frames consumed from current input in each stream
        let mut face_offset = 0u32;
        let mut stick_offset = 0u32;
        let mut dpad_offset = 0u32;

        // Continue until all streams are exhausted
        while face_idx < self.face_inputs.len()
            || stick_idx < self.stick_inputs.len()
            || dpad_idx < self.dpad_inputs.len()
        {
            // Get current input from each stream (or defaults if exhausted)
            let face = self.face_inputs.get(face_idx);
            let stick = self.stick_inputs.get(stick_idx);
            let dpad = self.dpad_inputs.get(dpad_idx);

            // Calculate remaining frames for current input in each stream
            let face_remaining = face
                .map(|f| f.frame_duration() - face_offset)
                .unwrap_or(u32::MAX);
            let stick_remaining = stick
                .map(|s| s.frame_duration() as u32 - stick_offset)
                .unwrap_or(u32::MAX);
            let dpad_remaining = dpad
                .map(|d| d.frame_duration() - dpad_offset)
                .unwrap_or(u32::MAX);

            // Find the minimum remaining frames (when next change occurs)
            let duration = face_remaining.min(stick_remaining).min(dpad_remaining);

            if duration == u32::MAX { // if all streams exhausted
                break;
            }

            // Create combined input for this duration
            let combined = Input::new(
                face.map(|f| f.buttons().clone()).unwrap_or_default(),
                stick.map(|s| s.x()).unwrap_or(0),
                stick.map(|s| s.y()).unwrap_or(0),
                dpad.map(|d| d.button()).unwrap_or(DPadButton::None),
                duration,
            );
            result.push(combined);

            // Update offsets and advance indices where needed
            face_offset += duration;
            stick_offset += duration;
            dpad_offset += duration;

            if face.is_some() && face_offset >= face.unwrap().frame_duration() {
                face_idx += 1;
                face_offset = 0;
            }
            if stick.is_some() && stick_offset >= stick.unwrap().frame_duration() as u32 {
                stick_idx += 1;
                stick_offset = 0;
            }
            if dpad.is_some() && dpad_offset >= dpad.unwrap().frame_duration() {
                dpad_idx += 1;
                dpad_offset = 0;
            }
        }

        result
    }

    pub fn face_inputs(&self) -> &[FaceInput] {
        &self.face_inputs
    }

    pub fn stick_inputs(&self) -> &[StickInput] {
        &self.stick_inputs
    }

    pub fn dpad_inputs(&self) -> &[DPadInput] {
        &self.dpad_inputs
    }
}
