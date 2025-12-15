use crate::byte_handler::FromByteHandler;

#[derive(thiserror::Error, Debug)]
pub enum DateError {
    #[error("Year is invalid")]
    YearInvalid,
    #[error("Month is invalid")]
    MonthInvalid,
    #[error("Day is invalid")]
    DayInvalid,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Debug)]
pub struct Date {
    year: u8,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, DateError> {
        let year = (year - 2000) as u8;

        if year > 35 {
            return Err(DateError::YearInvalid);
        }

        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 if day > 31 => Err(DateError::DayInvalid),
            4 | 6 | 9 | 11 if day > 30 => Err(DateError::DayInvalid),
            2 if year.is_multiple_of(4) && day > 29 => Err(DateError::DayInvalid),
            2 if day > 28 => Err(DateError::DayInvalid),
            1..=12 => Ok(Self { year, month, day }),
            _ => Err(DateError::MonthInvalid),
        }
    }

    pub fn year(&self) -> u16 {
        (self.year as u16) + 2000
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }
}

impl FromByteHandler for Date {
    type Err = DateError;
    fn from_byte_handler<T: TryInto<crate::byte_handler::ByteHandler>>(
        handler: T,
    ) -> Result<Self, Self::Err> {
        let mut handler = handler.try_into().map_err(|_|()).expect("TODO: handle this");

        handler.shift_right(1);
        let day_num = handler.copy_bytes()[2] & 0x0F;

        handler.shift_right(3);

        Self::new(
            (handler.copy_words()[1] >> 9) + 2000,
            day_num,
            handler.copy_bytes()[3] & 0x1F,
        )
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year
    }
}
