use crate::byte_handler::FromByteHandler;
use crate::ctgp_metadata::exact_finish_time::ExactFinishTime;
use crate::ctgp_metadata::{category::Category, ctgp_version::CTGPVersion};
use crate::header::in_game_time::InGameTime;
use crate::{byte_handler::ByteHandler, input_data::yaz1_decompress};
use chrono::{Duration, TimeDelta, prelude::*};

pub mod category;
pub mod ctgp_version;
pub mod exact_finish_time;

#[derive(thiserror::Error, Debug)]
pub enum CTGPMetadataError {
    #[error("Ghost is not CKGD")]
    NotCKGD,
    #[error("Invalid CTGP metadata version")]
    InvalidMetadataVersion,
    #[error("Try From Slice Error: {0}")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
    #[error("Category Error: {0}")]
    CategoryError(#[from] category::CategoryError),
    #[error("In Game Time Error")]
    InGameTimeError(#[from] crate::header::in_game_time::InGameTimeError),
}

pub struct CTGPMetadata {
    security_data: Vec<u8>,
    track_sha1: [u8; 0x14],
    player_id: u64,
    exact_finish_time: ExactFinishTime,
    ctgp_version: Option<CTGPVersion>,
    lap_split_dubious_intersections: Option<[bool; 10]>,
    exact_lap_times: [ExactFinishTime; 10],
    rtc_race_end: NaiveDateTime,
    rtc_race_begins: NaiveDateTime,
    rtc_time_paused: TimeDelta,
    pause_frames: Vec<u32>,
    my_stuff_enabled: bool,
    my_stuff_used: bool,
    usb_gamecube_enabled: bool,
    final_lap_dubious_intersection: bool,
    shroomstrat: [u8; 10],
    cannoned: bool,
    went_oob: bool,
    has_slowdown: bool,
    has_rapidfire: bool,
    dubious_ghost: bool,
    has_mii_data_replaced: bool,
    has_name_replaced: bool, // Hi Korben
    respawns: bool,
    category: Category,
    metadata_version: u8,
    metadata_size: u32,
    lap_count: u8,
}

impl CTGPMetadata {
    /// Expects full rkg data
    pub fn new(data: &[u8]) -> Result<Self, CTGPMetadataError> {
        if data[data.len() - 0x08..data.len() - 0x04] != [0x43, 0x4B, 0x47, 0x44] {
            return Err(CTGPMetadataError::NotCKGD);
        }

        let metadata_size =
            u32::from_be_bytes(data[data.len() - 0x0C..data.len() - 0x08].try_into()?);

        let metadata_version = data[data.len() - 0x0D];
        let security_data_size = if metadata_version < 7 { 0x44 } else { 0x54 };

        let header_data = &data[..0x88];
        let input_data = &data[0x88..data.len() - metadata_size as usize];
        let metadata = &data[data.len() - metadata_size as usize..];
        let mut current_offset = 0usize;

        let security_data = Vec::from(&metadata[..security_data_size]);
        current_offset += security_data_size;

        let mut track_sha1 = [0; 0x14];
        let track_sha1_offset = current_offset;
        for (index, byte) in metadata[track_sha1_offset..track_sha1_offset + 0x14]
            .iter()
            .enumerate()
        {
            track_sha1[index] = *byte;
            current_offset += 0x01;
        }

        let player_id =
            u64::from_be_bytes(metadata[current_offset..current_offset + 0x08].try_into()?);
        current_offset += 0x08;

        let finish_time = InGameTime::from_byte_handler(&header_data[0x04..0x07])?;
        let true_time_subtraction =
            (f32::from_be_bytes(metadata[current_offset..current_offset + 0x04].try_into()?)
                * 1e+9).floor() as i64;
        let exact_finish_time = ExactFinishTime::new(
            finish_time.minutes(),
            finish_time.seconds(),
            (finish_time.milliseconds() as i64 * 1e+9 as i64 + true_time_subtraction) as u64,
        );
        current_offset += 0x04;

        let ctgp_version;
        let mut lap_split_dubious_intersections = Some([false; 10]);

        if metadata_version >= 2 {
            ctgp_version = Some(CTGPVersion::new(
                metadata[current_offset..current_offset + 0x04].try_into()?,
            ));
            current_offset += 0x04;

            let laps_handler = ByteHandler::try_from(&metadata[current_offset..current_offset + 2])
                .expect("ByteHandler try_from() failed");

            if let Some(mut array) = lap_split_dubious_intersections {
                for (index, intersection) in array.iter_mut().enumerate() {
                    *intersection = laps_handler.read_bool(index as u8 + 6);
                }
            }
            current_offset -= 0x04;
        } else {
            ctgp_version = None;
            lap_split_dubious_intersections = None;
        }

        current_offset += 0x3C;

        // Exact lap split calculation
        let mut previous_subtractions = 0i64;
        let mut exact_lap_times = [ExactFinishTime::default(); 10];
        let lap_count = header_data[0x10];
        let mut in_game_time_offset = 0x11usize;
        let mut subtraction_ps = 0i64;

        for index in 0..lap_count as usize {
            let mut true_time_subtraction =
                ((f32::from_be_bytes(metadata[current_offset..current_offset + 0x04].try_into()?) as f64)
                    * 1e+9).floor() as i64;

            let lap_time = InGameTime::from_byte_handler(
                &header_data[in_game_time_offset..in_game_time_offset + 0x03],
            )?;

            // subtract the sum of the previous laps' difference because the lap differences add up to
            // have its decimal portion be equal to the total time
            true_time_subtraction -= previous_subtractions;
            if true_time_subtraction > 1e+9 as i64 {
                true_time_subtraction -= subtraction_ps;
                subtraction_ps = if subtraction_ps == 0 { 1e+9 as i64 } else { 0 };
            }
            previous_subtractions += true_time_subtraction;
            exact_lap_times[index] = ExactFinishTime::new(
                lap_time.minutes(),
                lap_time.seconds(),
                (lap_time.milliseconds() as i64 * 1e+9 as i64 + true_time_subtraction) as u64,
            );
            in_game_time_offset += 0x03;
            current_offset -= 0x04;
        }

        current_offset += 0x04 * (lap_count as usize + 1);

        let rtc_race_end = datetime_from_timestamp(u64::from_be_bytes(
            metadata[current_offset..current_offset + 0x08].try_into()?,
        ));
        current_offset += 0x08;

        let rtc_race_begins = datetime_from_timestamp(u64::from_be_bytes(
            metadata[current_offset..current_offset + 0x08].try_into()?,
        ));
        current_offset += 0x08;

        let rtc_time_paused = duration_from_ticks(u64::from_be_bytes(
            metadata[current_offset..current_offset + 0x08].try_into()?,
        ));
        current_offset += 0x08;

        // Pause frame times
        let mut pause_frames = Vec::new();
        let input_data = if input_data[4..8] == [0x59, 0x61, 0x7A, 0x31] {
            // YAZ1 header, decompress
            yaz1_decompress(&input_data[4..]).unwrap()
        } else {
            Vec::from(input_data)
        };

        let face_input_count = u16::from_be_bytes([input_data[0], input_data[1]]);

        let mut current_input_byte = 8;
        let mut elapsed_frames = 1u32;
        while current_input_byte < 8 + face_input_count * 2 {
            let idx = current_input_byte as usize;
            let input = &input_data[idx..idx + 2];

            if input[0] & 0x40 != 0 {
                pause_frames.push(elapsed_frames);
            }

            elapsed_frames += input[1] as u32;
            current_input_byte += 2;
        }

        let bool_handler = ByteHandler::from(metadata[current_offset]);
        let my_stuff_enabled = bool_handler.read_bool(3);
        let my_stuff_used = bool_handler.read_bool(2);
        let usb_gamecube_enabled = bool_handler.read_bool(1);
        let final_lap_dubious_intersection = bool_handler.read_bool(0);
        current_offset += 0x01;

        let mut shroomstrat: [u8; 10] = [0; 10];
        for _ in 0..3 {
            let lap = metadata[current_offset];
            if lap != 0 {
                shroomstrat[(lap - 1) as usize] += 1;
            }
            current_offset += 0x01;
        }

        let category = Category::try_from(metadata[current_offset + 2], metadata[current_offset])?;
        current_offset += 0x01;
        let bool_handler = ByteHandler::from(metadata[current_offset]);
        let cannoned = bool_handler.read_bool(7);
        let went_oob = bool_handler.read_bool(6);
        let has_slowdown = bool_handler.read_bool(5);
        let has_rapidfire = bool_handler.read_bool(4);
        let dubious_ghost = bool_handler.read_bool(3);
        let has_mii_data_replaced = bool_handler.read_bool(2);
        let has_name_replaced = bool_handler.read_bool(1);
        let respawns = bool_handler.read_bool(0);

        Ok(Self {
            security_data,
            track_sha1,
            player_id,
            exact_finish_time,
            ctgp_version,
            lap_split_dubious_intersections,
            exact_lap_times,
            rtc_race_end,
            rtc_race_begins,
            rtc_time_paused,
            pause_frames,
            my_stuff_enabled,
            my_stuff_used,
            usb_gamecube_enabled,
            final_lap_dubious_intersection,
            shroomstrat,
            cannoned,
            went_oob,
            has_slowdown,
            has_rapidfire,
            dubious_ghost,
            has_mii_data_replaced,
            has_name_replaced,
            respawns,
            category,
            metadata_version,
            metadata_size,
            lap_count,
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

    pub fn exact_finish_time(&self) -> ExactFinishTime {
        self.exact_finish_time
    }

    pub fn ctgp_version(&self) -> Option<CTGPVersion> {
        self.ctgp_version
    }

    pub fn lap_split_dubious_intersections(&self) -> Option<&[bool]> {
        if let Some(intersections) = &self.lap_split_dubious_intersections {
            return Some(&intersections[0..self.lap_count as usize]);
        }
        None
    }

    pub fn exact_lap_times(&self) -> &[ExactFinishTime] {
        &self.exact_lap_times[0..self.lap_count as usize]
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

    pub fn pause_frames(&self) -> &Vec<u32> {
        &self.pause_frames
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
        &self.shroomstrat[0..self.lap_count as usize]
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

    pub fn metadata_version(&self) -> u8 {
        self.metadata_version
    }

    pub fn metadata_size(&self) -> u32 {
        self.metadata_size
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

/// Used with a face button byte
fn contains_ctgp_pause(buttons: u8) -> bool {
    buttons & 0x40 != 0
}
