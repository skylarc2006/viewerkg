use crate::rkg::header::{finish_time::FinishTime, mii::Mii};

pub mod finish_time;
pub mod mii;

#[allow(dead_code)]
pub struct Header {
    rkgd: String, 				            // 0x04, offset 0x00
    finish_time: FinishTime, 	   			// 0x03, offset 0x04
    track_id: u8,				            // 6 bits, offset 0x07
    unknown1: u8,				            // 2 bits, offset 0x07.6, likely padding
    vehicle_id: u8,				            // 6 bits, offset 0x08
    character_id: u8,			        	// 6 bits, offset 0x08.6
    year_set: u8,				            // 7 bits, offset 0x09.4
    month_set: u8,				            // 4 bits, offset 0x0A.3
    day_set: u8,				            // 5 bits, offset 0x0A.7
    controller_id: u8,				        // 4 bits, offset 0x0B.4
    unknown2: u8,				            // 4 bits, offset 0x0C, always 0?
    is_compressed: bool,				    // 1 bit, offset 0xC.4
    unknown3: u8,				            // 2 bits, offset 0x0C.5, always 0?
    ghost_type: u8,				            // 7 bits, offset 0x0C.7
    is_automatic_drift: bool,				// 1 bit, offset 0x0D.6
    unknown4: bool,					        // 1 bit, offset 0x0D.7, likely padding
    decompressed_input_data_length: u16,    // 0x02, offset 0x0E	
    lap_count: u8,				            // 0x01, offset 0x10
    lap_split_times: Vec<FinishTime>,	    // 0x0F, offset 0x11, first 5 laps
    // 0x14, offset 0x20, vanilla game attempts to store laps greater than 5 but fails.
    country_code: u8,				        // 0x01, offset 0x34
    state_code: u8,				            // 0x01, offset 0x35
    location_code: u16,				        // 0x02, offset 0x36
    unknown6: u32,				            // 0x04, offset 0x38, typically 0
    mii_data: Mii,				            // 0x4A, offset 0x3C
    mii_crc16: u16,				            // 0x02, offset 0x86
}

impl Header {
    // TODO: learn how to read file data and start doing cool stuff
    /*
    pub fn new(ghost file data) {
        read all values...
    }
     */

    pub fn rkgd(&self) -> &str {
        &self.rkgd
    }

    pub fn finish_time(&self) -> &FinishTime {
        &self.finish_time
    }

    pub fn track_id(&self) -> u8 {
        self.track_id
    }

    pub fn unknown1(&self) -> u8 {
        self.unknown1
    }

    pub fn vehicle_id(&self) -> u8 {
        self.vehicle_id
    }

    pub fn character_id(&self) -> u8 {
        self.character_id
    }

    pub fn year_set(&self) -> u8 {
        self.year_set
    }

    pub fn month_set(&self) -> u8 {
        self.month_set
    }

    pub fn day_set(&self) -> u8 {
        self.day_set
    }

    pub fn controller_id(&self) -> u8 {
        self.controller_id
    }

    pub fn unknown2(&self) -> u8 {
        self.unknown2
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }

    pub fn unknown3(&self) -> u8 {
        self.unknown3
    }

    pub fn ghost_type(&self) -> u8 {
        self.ghost_type
    }

    pub fn is_automatic_drift(&self) -> bool {
        self.is_automatic_drift
    }

    pub fn unknown4(&self) -> bool {
        self.unknown4
    }

    pub fn decompressed_input_data_length(&self) -> u16 {
        self.decompressed_input_data_length
    }

    pub fn lap_count(&self) -> u8 {
        self.lap_count
    }

    pub fn lap_split_times(&self) -> &[FinishTime] {
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

    pub fn unknown6(&self) -> u32 {
        self.unknown6
    }

    pub fn mii_data(&self) -> &Mii {
        &self.mii_data
    }

    pub fn mii_crc16(&self) -> u16 {
        self.mii_crc16
    }
}
