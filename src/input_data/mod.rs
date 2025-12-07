// https://wiki.tockdom.com/wiki/RKG_(File_Format)#Controller_Input_Data

use bitreader::BitReader;

use crate::input_data::face_input::FaceInput;
use crate::input_data::dpad_input::DPadInput;
use crate::input_data::stick_input::StickInput;

pub mod stick_input;
pub mod face_input;
pub mod dpad_input;

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
    face_input_count: u16,
    stick_input_count: u16,
    dpad_input_count: u16,

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

        let mut face_inputs: Vec<FaceInput> = Vec::new();
        for _ in 0..face_input_count {
            face_inputs.push(FaceInput::try_from(&mut input_reader)?);
        }

        let mut stick_inputs: Vec<StickInput> = Vec::new();
        for _ in 0..stick_input_count {
            stick_inputs.push(StickInput::try_from(&mut input_reader)?);
        }

        let mut dpad_inputs: Vec<DPadInput> = Vec::new();
        for _ in 0..dpad_input_count {
            dpad_inputs.push(DPadInput::try_from(&mut input_reader)?);
        }

        Ok(Self {
            face_input_count,
            stick_input_count,
            dpad_input_count,
            face_inputs,
            stick_inputs,
            dpad_inputs,
        })
    }

    pub fn face_input_count(&self) -> u16 {
        self.face_input_count
    }

    pub fn stick_input_count(&self) -> u16 {
        self.stick_input_count
    }

    pub fn dpad_input_count(&self) -> u16 {
        self.dpad_input_count
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
