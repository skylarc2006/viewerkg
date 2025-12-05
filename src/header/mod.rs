use bitreader::BitReader;

use crate::header::{finish_time::FinishTime, mii::Mii, slot_id::SlotId};

pub mod finish_time;
pub mod mii;
pub mod slot_id;

pub struct Header {
    rkgd: String,                        // 0x04, offset 0x00
    finish_time: FinishTime,             // 0x03, offset 0x04
    slot_id: SlotId,                     // 6 bits, offset 0x07
    unknown1: u8,                        // 2 bits, offset 0x07.6, likely padding
    vehicle_id: u8,                      // 6 bits, offset 0x08
    character_id: u8,                    // 6 bits, offset 0x08.6
    year_set: u16, // 7 bits, offset 0x09.4 (Stores year relative to 2000 but will be stored as actual year here)
    month_set: u8, // 4 bits, offset 0x0A.3
    day_set: u8,   // 5 bits, offset 0x0A.7
    controller_id: u8, // 4 bits, offset 0x0B.4
    unknown2: u8,  // 4 bits, offset 0x0C, always 0?
    is_compressed: bool, // 1 bit, offset 0xC.4
    unknown3: u8,  // 2 bits, offset 0x0C.5, always 0?
    ghost_type: u8, // 7 bits, offset 0x0C.7
    is_automatic_drift: bool, // 1 bit, offset 0x0D.6
    unknown4: bool, // 1 bit, offset 0x0D.7, likely padding
    decompressed_input_data_length: u16, // 0x02, offset 0x0E
    lap_count: u8, // 0x01, offset 0x10
    lap_split_times: Vec<FinishTime>, // 0x0F, offset 0x11, first 5 laps
    // 0x14, offset 0x20, vanilla game attempts to store laps greater than 5 but fails.
    country_code: u8,   // 0x01, offset 0x34
    state_code: u8,     // 0x01, offset 0x35
    location_code: u16, // 0x02, offset 0x36
    unknown6: u32,      // 0x04, offset 0x38, typically 0
    mii_data: Mii,      // 0x4A, offset 0x3C
    mii_crc16: u16,     // 0x02, offset 0x86
}

impl Header {
    pub fn new(rkg_data: &[u8]) -> Self {
        let mut rkg_reader: BitReader<'_> = BitReader::new(rkg_data);

        let rkgd = get_rkgd(&mut rkg_reader);
        let finish_time = FinishTime::from(&mut rkg_reader);
        let slot_id = SlotId::try_from(&mut rkg_reader).expect("Non Existent Slot ID");
        let unknown1 = rkg_reader.read_u8(2).expect("Failed to read unknown1");
        let vehicle_id = rkg_reader.read_u8(6).expect("Failed to read vehicle ID");
        let character_id = rkg_reader.read_u8(6).expect("Failed to read character ID");
        let year_set = rkg_reader.read_u16(7).expect("Failed to read year set") + 2000;
        let month_set = rkg_reader.read_u8(4).expect("Failed to read month set");
        let day_set = rkg_reader.read_u8(5).expect("Failed to read day set");
        let controller_id = rkg_reader.read_u8(4).expect("Failed to read controller ID");
        let unknown2 = rkg_reader.read_u8(4).expect("Failed to read unknown2");

        let is_compressed: bool = rkg_reader
            .read_bool()
            .expect("Failed to read is_compressed");

        let unknown3: u8 = rkg_reader.read_u8(2).expect("Failed to read unknown3");
        let ghost_type: u8 = rkg_reader.read_u8(7).expect("Failed to read ghost type");

        let is_automatic_drift: bool = rkg_reader
            .read_bool()
            .expect("Failed to read is_automatic_drift");

        let unknown4: bool = rkg_reader.read_bool().expect("Failed to read unknown4");

        let decompressed_input_data_length: u16 = rkg_reader
            .read_u16(16)
            .expect("Failed to read decompressed input data length");

        let lap_count: u8 = rkg_reader.read_u8(8).expect("Failed to read lap count");

        let mut lap_split_times: Vec<FinishTime> = Vec::new();
        for _ in 1..=9 {
            lap_split_times.push(FinishTime::from(&mut rkg_reader));
        }

        // Skip garbage RAM data
        rkg_reader.skip(64).expect("Failed to skip garbage data");

        let country_code: u8 = rkg_reader.read_u8(8).expect("Failed to read country code");
        let state_code: u8 = rkg_reader.read_u8(8).expect("Failed to read state code");

        let location_code: u16 = rkg_reader
            .read_u16(16)
            .expect("Failed to read location code");

        let unknown6: u32 = rkg_reader.read_u32(32).expect("Failed to read unknown6");
        let mii_data: Mii = Mii::new(&rkg_data[0x3C..0x86]);

        // Skip current reader over mii data (Mii constructor uses its own reader)
        for _ in 1..=74 {
            rkg_reader.skip(8).expect("Failed to skip Mii data");
        }

        let mii_crc16: u16 = rkg_reader.read_u16(16).expect("Failed to read Mii CRC16");

        Self {
            rkgd,
            finish_time,
            slot_id,
            unknown1,
            vehicle_id,
            character_id,
            year_set,
            month_set,
            day_set,
            controller_id,
            unknown2,
            is_compressed,
            unknown3,
            ghost_type,
            is_automatic_drift,
            unknown4,
            decompressed_input_data_length,
            lap_count,
            lap_split_times,
            country_code,
            state_code,
            location_code,
            unknown6,
            mii_data,
            mii_crc16,
        }
    }

    pub fn rkgd(&self) -> &str {
        &self.rkgd
    }

    pub fn finish_time(&self) -> &FinishTime {
        &self.finish_time
    }

    pub fn slot_id(&self) -> SlotId {
        self.slot_id
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

    pub fn year_set(&self) -> u16 {
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

fn get_rkgd(rkg_reader: &mut BitReader) -> String {
    let rkgd_bytes: [u8; _] = rkg_reader
        .read_u32(32)
        .expect("Failed to read rkgd")
        .to_be_bytes();

    // TODO: rewrite this and probably return Option<String>?
    // Convert the byte array to a String
    match String::from_utf8(rkgd_bytes.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to convert bytes to UTF-8 string: {}", e);
            let s: &str = "FUCK";
            s.to_string()
        }
    }
}
