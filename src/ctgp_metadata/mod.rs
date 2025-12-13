use crate::ctgp_metadata::category::Category;
use bitreader::BitReader;
use chrono::{Duration, TimeDelta, prelude::*};

pub mod category;

#[derive(thiserror::Error, Debug)]
pub enum CTGPMetadataError {
    #[error("Ghost is not CKGD")]
    NotCKGD,
    #[error("Category Error: {0}")]
    CategoryError(#[from] category::CategoryError),
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

pub struct CTGPMetadata {
    security_data: [u8; 0x54],
    track_sha1: [u8; 0x14],
    player_id: u64,
    true_time_subtraction: f32,
    ctgp_version: u32,
    unknown: [u8; 0x20],
    true_lap_time_subtractions: [f32; 7],
    rtc_race_end: NaiveDateTime,
    rtc_race_begins: NaiveDateTime,
    rtc_time_paused: TimeDelta,
    my_stuff_enabled: bool,
    my_stuff_used: bool,
    usb_gamecube_enabled: bool,
    final_lap_dubious_intersection: bool,
    shroomstrat: [u8; 8],
    shortcut_used: bool,
    cannoned: bool,
    went_oob: bool,
    has_slowdown: bool,
    has_rapidfire: bool,
    dubious_ghost: bool,
    has_mii_data_replaced: bool,
    has_name_replaced: bool, // Hi Korben
    respawns: bool,
    category: Category,
    footer_version: u8,
    metadata_length: u32,
}

impl CTGPMetadata {
    pub fn new(metadata: &[u8]) -> Result<Self, CTGPMetadataError> {
        // TODO: handle older footer versions

        if metadata[metadata.len() - 0x04..] != [0x43, 0x4B, 0x47, 0x44] {
            return Err(CTGPMetadataError::NotCKGD);
        }

        let mut metadata_reader = BitReader::new(metadata);
        let mut security_data = [0_u8; 0x54];
        for byte in &mut security_data {
            *byte = metadata_reader.read_u8(8)?;
        }

        let mut track_sha1: [u8; 0x14] = [0; 0x14];
        for byte in &mut track_sha1 {
            *byte = metadata_reader.read_u8(8)?;
        }

        let player_id = metadata_reader.read_u64(64)?;
        let true_time_subtraction = f32::from_bits(metadata_reader.read_u32(32)?);
        let ctgp_version = metadata_reader.read_u32(32)?;

        let mut unknown = [0_u8; 0x20];
        for byte in &mut unknown {
            *byte = metadata_reader.read_u8(8)?;
        }

        let mut true_lap_time_subtractions = [0.0; 7];
        for time in &mut true_lap_time_subtractions {
            *time = f32::from_bits(metadata_reader.read_u32(32)?);
        }

        let rtc_race_end = datetime_from_timestamp(metadata_reader.read_u64(64)?);
        let rtc_race_begins = datetime_from_timestamp(metadata_reader.read_u64(64)?);
        let rtc_time_paused = duration_from_ticks(metadata_reader.read_u64(64)?);
        metadata_reader.skip(4)?; // padding
        let my_stuff_enabled = metadata_reader.read_bool()?;
        let my_stuff_used = metadata_reader.read_bool()?;
        let usb_gamecube_enabled = metadata_reader.read_bool()?;
        let final_lap_dubious_intersection = metadata_reader.read_bool()?;

        let mut shroomstrat: [u8; 8] = [0; 8];
        for _ in 0..3 {
            let lap = metadata_reader.read_u8(8)?;
            if lap != 0 {
                shroomstrat[(lap - 1) as usize] += 1;
            }
        }
        let shortcut_used = metadata_reader.read_u8(8)? > 0;
        let cannoned = metadata_reader.read_bool()?;
        let went_oob = metadata_reader.read_bool()?;
        let has_slowdown = metadata_reader.read_bool()?;
        let has_rapidfire = metadata_reader.read_bool()?;
        let dubious_ghost = metadata_reader.read_bool()?;
        let has_mii_data_replaced = metadata_reader.read_bool()?;
        let has_name_replaced = metadata_reader.read_bool()?;
        let respawns = metadata_reader.read_bool()?;
        let category = Category::try_from(&mut metadata_reader)?;
        let footer_version = metadata_reader.read_u8(8)?;
        let metadata_length = metadata_reader.read_u32(32)?;

        Ok(Self {
            security_data,
            track_sha1,
            player_id,
            true_time_subtraction,
            ctgp_version,
            unknown,
            true_lap_time_subtractions,
            rtc_race_end,
            rtc_race_begins,
            rtc_time_paused,
            my_stuff_enabled,
            my_stuff_used,
            usb_gamecube_enabled,
            final_lap_dubious_intersection,
            shroomstrat,
            shortcut_used,
            cannoned,
            went_oob,
            has_slowdown,
            has_rapidfire,
            dubious_ghost,
            has_mii_data_replaced,
            has_name_replaced,
            respawns,
            category,
            footer_version,
            metadata_length,
        })
    }

    pub fn security_data(&self) -> &[u8] {
        &self.security_data
    }

    pub fn track_sha1(&self) -> &[u8] {
        &self.track_sha1
    }

    pub fn player_id(&self) -> u64 {
        self.player_id
    }

    pub fn true_time_subtraction(&self) -> f32 {
        self.true_time_subtraction
    }

    pub fn ctgp_version(&self) -> u32 {
        self.ctgp_version
    }

    pub fn unknown(&self) -> &[u8] {
        &self.unknown
    }

    pub fn true_lap_time_subtractions(&self) -> &[f32] {
        &self.true_lap_time_subtractions
    }

    pub fn rtc_race_end(&self) -> NaiveDateTime {
        self.rtc_race_end
    }

    pub fn rtc_race_begins(&self) -> NaiveDateTime {
        self.rtc_race_begins
    }

    pub fn rtc_time_paused(&self) -> TimeDelta {
        self.rtc_time_paused
    }

    pub fn my_stuff_enabled(&self) -> bool {
        self.my_stuff_enabled
    }

    pub fn my_stuff_used(&self) -> bool {
        self.my_stuff_used
    }

    pub fn usb_gamecube_enabled(&self) -> bool {
        self.usb_gamecube_enabled
    }

    pub fn final_lap_dubious_intersection(&self) -> bool {
        self.final_lap_dubious_intersection
    }

    pub fn shroomstrat(&self) -> &[u8] {
        &self.shroomstrat
    }

    pub fn shortcut_used(&self) -> bool {
        self.shortcut_used
    }

    pub fn cannoned(&self) -> bool {
        self.cannoned
    }

    pub fn went_oob(&self) -> bool {
        self.went_oob
    }

    pub fn has_slowdown(&self) -> bool {
        self.has_slowdown
    }

    pub fn has_rapidfire(&self) -> bool {
        self.has_rapidfire
    }

    pub fn dubious_ghost(&self) -> bool {
        self.dubious_ghost
    }

    pub fn has_mii_data_replaced(&self) -> bool {
        self.has_mii_data_replaced
    }

    pub fn has_name_replaced(&self) -> bool {
        self.has_name_replaced
    }

    pub fn respawns(&self) -> bool {
        self.respawns
    }

    pub fn category(&self) -> Category {
        self.category
    }

    pub fn footer_version(&self) -> u8 {
        self.footer_version
    }

    pub fn metadata_length(&self) -> u32 {
        self.metadata_length
    }
}

fn datetime_from_timestamp(tick_count: u64) -> NaiveDateTime {
    let clock_rate = 60_750_000.0; // 60.75 MHz tick speed
    let epoch_shift = 946_684_800; // Shifts epoch from 1970-01-01 to 2000-01-01 (which is what the Wii uses)
    let total_seconds = tick_count as f64 / clock_rate;
    let total_nanoseconds = (total_seconds * 1_000_000_000.0) as i64;

    let duration = Duration::nanoseconds(total_nanoseconds);
    let epoch = DateTime::from_timestamp(epoch_shift, 0).unwrap();

    epoch.naive_utc() + duration
}

fn duration_from_ticks(tick_count: u64) -> TimeDelta {
    let clock_rate = 60_750_000.0; // 60.75 MHz tick speed
    let total_seconds = tick_count as f64 / clock_rate;
    let total_milliseconds = (total_seconds * 1_000.0) as i64;

    Duration::milliseconds(total_milliseconds)
}
