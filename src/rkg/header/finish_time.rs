use bitreader::BitReader;

pub struct FinishTime {
    minutes: u8,       // 7 bits, offset 0x00
    seconds: u8,       // 7 bits, offset 0x00.7
    milliseconds: u16, // 10 bits, offset 0x01.6
    string: String,    // Total time as string
}

impl FinishTime {
    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    pub fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    pub fn string(&self) -> &str {
        &self.string
    }

    pub fn from_reader(rkg_reader: &mut BitReader<'_>) -> Self {
        // Get finish time fields
        let minutes: u8 = rkg_reader
            .read_u8(7)
            .expect("Failed to read minutes of finish time");

        let seconds: u8 = rkg_reader
            .read_u8(7)
            .expect("Failed to read seconds of finish time");

        let milliseconds: u16 = rkg_reader
            .read_u16(10)
            .expect("Failed to read milliseconds of finish time");

        Self {
            minutes,
            seconds,
            milliseconds,
            string: format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds),
        }
    }

    pub fn new(minutes: u8, seconds: u8, milliseconds: u16) -> Self {
        Self {
            minutes,
            seconds,
            milliseconds,
            string: format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds),
        }
    }
}
