use bitreader::BitReader;

use crate::input_data::dpad_input::{DPadButton, DPadInput};
use crate::input_data::face_input::FaceInput;
use crate::input_data::input::Input;
use crate::input_data::stick_input::StickInput;

pub mod dpad_input;
pub mod face_input;
pub mod input;
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

/// Handles all input data being read
/// Tockdom wiki: https://wiki.tockdom.com/wiki/RKG_(File_Format)#Controller_Input_Data
pub struct InputData {
    face_input_count: u16,
    stick_input_count: u16,
    dpad_input_count: u16,
    face_inputs: Vec<FaceInput>,
    stick_inputs: Vec<StickInput>,
    dpad_inputs: Vec<DPadInput>,
}

impl InputData {
    pub fn new(input_data: &[u8]) -> Result<Self, InputDataError> {
        let input_data = if input_data[4..8] == [0x59, 0x61, 0x7A, 0x31] {
            // YAZ1 header, decompress
            yaz1_decompress(&input_data[4..]).unwrap()
        } else {
            Vec::from(input_data)
        };

        let face_input_count = u16::from_be_bytes([input_data[0], input_data[1]]);
        let stick_input_count = u16::from_be_bytes([input_data[2], input_data[3]]);
        let dpad_input_count = u16::from_be_bytes([input_data[4], input_data[5]]);
        // bytes 6-7: padding

        let mut current_byte = 8;
        let mut face_inputs: Vec<FaceInput> = Vec::with_capacity(face_input_count as usize);
        while current_byte < 8 + face_input_count * 2 {
            let idx = current_byte as usize;
            let input = &input_data[idx..idx + 2];
            face_inputs.push(FaceInput::try_from(input)?);
            current_byte += 2;
        }

        current_byte = 8 + face_input_count * 2;
        let mut stick_inputs: Vec<StickInput> = Vec::with_capacity(stick_input_count as usize);
        while current_byte < 8 + (face_input_count + stick_input_count) * 2 {
            let idx = current_byte as usize;
            let input = &input_data[idx..idx + 2];
            stick_inputs.push(StickInput::try_from(input)?);
            current_byte += 2;
        }

        current_byte = 8 + (face_input_count + stick_input_count) * 2;
        let mut dpad_inputs: Vec<DPadInput> = Vec::with_capacity(dpad_input_count as usize);
        while current_byte < 8 + (face_input_count + stick_input_count + dpad_input_count) * 2 {
            let idx = current_byte as usize;
            let input = &input_data[idx..idx + 2];
            dpad_inputs.push(DPadInput::try_from(input)?);
            current_byte += 2;
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
            face_input_count,
            stick_input_count,
            dpad_input_count,
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

            if duration == u32::MAX {
                // if all streams exhausted
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

            if let Some(face) = face
                && face_offset >= face.frame_duration()
            {
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

    pub fn face_input_count(&self) -> u16 {
        self.face_input_count
    }

    pub fn stick_input_count(&self) -> u16 {
        self.stick_input_count
    }

    pub fn dpad_input_count(&self) -> u16 {
        self.dpad_input_count
    }
}

/// Decompress YAZ1-compressed input data
/// Adapted from https://github.com/AtishaRibeiro/InputDisplay/blob/master/InputDisplay/Core/Yaz1dec.cs
fn yaz1_decompress(data: &[u8]) -> Option<Vec<u8>> {
    // YAZ1 files start with "Yaz1" magic header
    if data.len() < 16 || &data[0..4] != b"Yaz1" {
        return None;
    }

    let uncompressed_size = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;

    let mut result = Vec::with_capacity(uncompressed_size);

    let decompressed = decompress_block(
        data,
        16, // Start after 16-byte header
        uncompressed_size,
    );

    if let Some(mut dec) = decompressed {
        result.append(&mut dec);
    }

    if result.len() == uncompressed_size {
        Some(result)
    } else {
        None
    }
}

fn decompress_block(src: &[u8], offset: usize, uncompressed_size: usize) -> Option<Vec<u8>> {
    let mut dst = Vec::with_capacity(uncompressed_size);
    let mut src_pos = offset;

    let mut valid_bit_count = 0; // number of valid bits left in "code" byte
    let mut curr_code_byte = 0u8;

    while dst.len() < uncompressed_size {
        // Read new "code" byte if the current one is used up
        if valid_bit_count == 0 {
            if src_pos >= src.len() {
                return None;
            }
            curr_code_byte = src[src_pos];
            src_pos += 1;
            valid_bit_count = 8;
        }

        if (curr_code_byte & 0x80) != 0 {
            // Straight copy
            if src_pos >= src.len() {
                return None;
            }
            dst.push(src[src_pos]);
            src_pos += 1;
        } else {
            // RLE part
            if src_pos + 1 >= src.len() {
                return None;
            }

            let byte1 = src[src_pos];
            src_pos += 1;
            let byte2 = src[src_pos];
            src_pos += 1;

            let dist = (((byte1 & 0xF) as usize) << 8) | (byte2 as usize);
            let copy_source = dst.len().wrapping_sub(dist + 1);

            let mut num_bytes = (byte1 >> 4) as usize;
            if num_bytes == 0 {
                if src_pos >= src.len() {
                    return None;
                }
                num_bytes = src[src_pos] as usize + 0x12;
                src_pos += 1;
            } else {
                num_bytes += 2;
            }

            // Copy run - must handle overlapping copies
            for i in 0..num_bytes {
                if copy_source + i >= dst.len() {
                    return None;
                }
                let byte = dst[copy_source + i];
                dst.push(byte);
            }
        }

        // Use next bit from "code" byte
        curr_code_byte <<= 1;
        valid_bit_count -= 1;
    }

    Some(dst)
}