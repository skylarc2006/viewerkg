use crate::{byte_handler::{ByteHandler, FromByteHandler}, header::{
    combo::{Combo, ComboError},
    controller::{Controller, ControllerError},
    date::{Date, DateError},
    ghost_type::{GhostType, GhostTypeError},
    in_game_time::{InGameTime, InGameTimeError},
    mii::{Mii, MiiError},
    slot_id::{SlotId, SlotIdError},
}};

use std::io::Read;

pub mod combo;
pub mod controller;
pub mod date;
pub mod ghost_type;
pub mod in_game_time;
pub mod slot_id;
pub mod mii;

#[derive(thiserror::Error, Debug)]
pub enum HeaderError {
    #[error("File is not RKGD")]
    NotRKGD,
    #[error("Data passed is not correct size (0x88)")]
    NotCorrectSize,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
    #[error("In Game Time Error: {0}")]
    InGameTimeError(#[from] InGameTimeError),
    #[error("Slot ID Error: {0}")]
    SlotIdError(#[from] SlotIdError),
    #[error("Combo Error: {0}")]
    ComboError(#[from] ComboError),
    #[error("Date Error: {0}")]
    DateError(#[from] DateError),
    #[error("Controller Error: {0}")]
    ControllerError(#[from] ControllerError),
    #[error("Ghost Type Error: {0}")]
    GhostTypeError(#[from] GhostTypeError),
    #[error("Mii Error: {0}")]
    MiiError(#[from] MiiError),
    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error),
}

/// All the data in the Header of an RKGD
/// https://wiki.tockdom.com/wiki/RKG_(File_Format)#File_Header
pub struct Header {
    finish_time: InGameTime,
    slot_id: SlotId,
    combo: Combo,
    date_set: Date,
    controller: Controller,
    is_compressed: bool,
    ghost_type: GhostType,
    is_automatic_drift: bool,
    decompressed_input_data_length: u16,
    lap_count: u8,
    lap_split_times: [InGameTime; 8],
    country_code: u8,
    state_code: u8,
    location_code: u16,
    mii_data: Mii,
    mii_crc16: u16,
}

impl Header {
    /// Reads header from a file at the path
    pub fn new_from_path<P: AsRef<std::path::Path>>(p: P) -> Result<Self, HeaderError> {
        let mut rkg_data = [0u8; 0x88];
        std::fs::File::open(p)?.read_exact(&mut rkg_data)?;
        Self::new(&rkg_data)
    }

    /// Reads header from slice
    pub fn new(header_data: &[u8]) -> Result<Self, HeaderError> {
        if header_data.len() != 0x88 {
            return Err(HeaderError::NotCorrectSize);
        }
        if header_data[0..4] != [0x52, 0x4B, 0x47, 0x44] {
            return Err(HeaderError::NotRKGD);
        }

        let finish_time = InGameTime::from_byte_handler(&header_data[4..7])?;
        let slot_id = SlotId::from_byte_handler(header_data[7])?;
        let combo = Combo::from_byte_handler(&header_data[0x08..0x0A])?;
        let date_set = Date::from_byte_handler(&header_data[0x09..=0x0B])?;
        let controller = Controller::from_byte_handler(header_data[0x0B])?;
        let is_compressed = ByteHandler::from(header_data[0x0C]).read_bool(3);
        let ghost_type = GhostType::from_byte_handler(&header_data[0x0C..=0x0D])?;
        let is_automatic_drift = true; ByteHandler::from(header_data[0x0D]).read_bool(0);
        let decompressed_input_data_length = ByteHandler::try_from(&header_data[0x0E..=0x0F]).unwrap().copy_words()[1];

        let lap_count = header_data[0x10];
        let mut lap_split_times: [InGameTime; 8] = [Default::default(); 8];
        for index in 0..lap_count {
            let start = (0x11 + index*3) as usize;
            lap_split_times[index as usize] = InGameTime::from_byte_handler(&header_data[start..start+3])?;
        }

        let codes = ByteHandler::try_from(&header_data[0x34..=0x37]).unwrap();
        let country_code = codes.copy_bytes()[0];
        let state_code = codes.copy_bytes()[1];
        let location_code = codes.copy_words()[1];

        let mii_data = Mii::new(&header_data[0x3C..0x3C+0x4A])?;

        // TODO: Use CRC for its intended purpose and error out if wrong OR report mismatch
        let mii_crc16 = ByteHandler::try_from(&header_data[0x86..=0x87]).unwrap().copy_words()[1];

        Ok(Self {
            finish_time,
            slot_id,
            combo,
            date_set,
            controller,
            is_compressed,
            ghost_type,
            is_automatic_drift,
            decompressed_input_data_length,
            lap_count,
            lap_split_times,
            country_code,
            state_code,
            location_code,
            mii_data,
            mii_crc16,
        })
    }

    pub fn finish_time(&self) -> &InGameTime {
        &self.finish_time
    }

    pub fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    pub fn combo(&self) -> &Combo {
        &self.combo
    }

    pub fn date_set(&self) -> &Date {
        &self.date_set
    }

    pub fn controller(&self) -> Controller {
        self.controller
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub fn ghost_type(&self) -> GhostType {
        self.ghost_type
    }

    pub fn is_automatic_drift(&self) -> bool {
        self.is_automatic_drift
    }

    pub fn decompressed_input_data_length(&self) -> u16 {
        self.decompressed_input_data_length
    }

    pub fn lap_count(&self) -> u8 {
        self.lap_count
    }

    pub fn lap_split_times(&self) -> &[InGameTime] {
        &self.lap_split_times
    }

    pub fn country_code(&self) -> u8 {
        self.country_code
    }

    pub fn state_code(&self) -> u8 {
        self.state_code
    }

    pub fn location_code(&self) -> u16 {
        self.location_code
    }

    pub fn mii_data(&self) -> &Mii {
        &self.mii_data
    }

    pub fn mii_crc16(&self) -> u16 {
        self.mii_crc16
    }
}
