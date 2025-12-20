use crate::{
    byte_handler::{ByteHandler, FromByteHandler},
    header::{
        combo::{Combo, ComboError},
        controller::{Controller, ControllerError},
        date::{Date, DateError},
        ghost_type::{GhostType, GhostTypeError},
        in_game_time::{InGameTime, InGameTimeError},
        location::country::{Country, CountryError},
        mii::{Mii, MiiError},
        slot_id::{SlotId, SlotIdError},
    },
};

use std::io::Read;

pub mod combo;
pub mod controller;
pub mod date;
pub mod ghost_type;
pub mod in_game_time;
pub mod location;
pub mod mii;
pub mod slot_id;

#[derive(thiserror::Error, Debug)]
pub enum HeaderError {
    #[error("File is not RKGD")]
    NotRKGD,
    #[error("Data passed is not correct size (0x88)")]
    NotCorrectSize,
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
    #[error("Country Error: {0}")]
    CountryError(#[from] CountryError),
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
    lap_split_times: [InGameTime; 10],
    country: Country,
    subregion: u8,
    location_code: u16,
    mii_bytes: [u8; 0x4A],
    mii: Mii,
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
        let is_automatic_drift = ByteHandler::from(header_data[0x0D]).read_bool(1);
        let decompressed_input_data_length = ByteHandler::try_from(&header_data[0x0E..=0x0F])
            .unwrap()
            .copy_word(1);

        let lap_count = header_data[0x10];
        let mut lap_split_times: [InGameTime; 10] = [Default::default(); 10];
        for index in 0..lap_count {
            let start = (0x11 + index * 3) as usize;
            lap_split_times[index as usize] =
                InGameTime::from_byte_handler(&header_data[start..start + 3])?;
        }

        let codes = ByteHandler::try_from(&header_data[0x34..=0x37]).unwrap();
        let country = Country::try_from(codes.copy_byte(0))?;
        let subregion = codes.copy_byte(1);
        let location_code = codes.copy_word(1);

        let mut mii_bytes = [0_u8; 0x4A];
        for (index, byte) in header_data[0x3C..0x3C + 0x4A].iter().enumerate() {
            mii_bytes[index] = *byte;
        }
        let mii = Mii::new(mii_bytes)?;

        let mii_crc16 = ByteHandler::try_from(&header_data[0x86..=0x87])
            .unwrap()
            .copy_word(1);

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
            country,
            subregion,
            location_code,
            mii_bytes,
            mii,
            mii_crc16,
        })
    }

    /// Returns true if Mii CRC16 is correct (i.e. Mii data not illegally tampered with)
    pub fn verify_mii_crc16(&self) -> bool {
        crc16(&self.mii_bytes) == self.mii_crc16()
    }

    /// Recalculates and updates Mii CRC16
    pub fn fix_mii_crc16(&mut self) {
        self.mii_crc16 = crc16(&self.mii_bytes);
    }

    pub fn finish_time(&self) -> &InGameTime {
        &self.finish_time
    }

    pub fn set_finish_time(&mut self, finish_time: InGameTime) {
        self.finish_time = finish_time;
    }

    pub fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    pub fn set_slot_id(&mut self, slot_id: SlotId) {
        self.slot_id = slot_id;
    }

    pub fn combo(&self) -> &Combo {
        &self.combo
    }

    pub fn set_combo(&mut self, combo: Combo) {
        self.combo = combo;
    }

    pub fn date_set(&self) -> &Date {
        &self.date_set
    }

    pub fn set_date_set(&mut self, date_set: Date) {
        self.date_set = date_set;
    }

    pub fn controller(&self) -> Controller {
        self.controller
    }

    pub fn set_controller(&mut self, controller: Controller) {
        self.controller = controller;
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub fn ghost_type(&self) -> GhostType {
        self.ghost_type
    }

    pub fn set_ghost_type(&mut self, ghost_type: GhostType) {
        self.ghost_type = ghost_type;
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
        &self.lap_split_times[0..self.lap_count as usize]
    }

    pub fn set_lap_split_times(&mut self, lap_split_times: [InGameTime; 10]) {
        self.lap_split_times = lap_split_times;
    }

    pub fn country(&self) -> Country {
        self.country
    }

    pub fn subregion(&self) -> u8 {
        self.subregion
    }

    pub fn location_code(&self) -> u16 {
        self.location_code
    }

    pub fn mii(&self) -> &Mii {
        &self.mii
    }

    pub fn mii_crc16(&self) -> u16 {
        self.mii_crc16
    }
}

fn crc16(value: &[u8]) -> u16 {
    let mut crc: u16 = 0x0000; // Initial value for XModem variant
    let polynomial: u16 = 0x1021; // Standard CCITT polynomial

    for &byte in value.iter() {
        crc ^= (byte as u16) << 8; // XOR current byte with the high byte of CRC

        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ polynomial;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}
