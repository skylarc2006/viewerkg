use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(thiserror::Error, Debug)]
pub enum BirthdayError {
    #[error("Month is invalid")]
    MonthInvalid,
    #[error("Day is invalid")]
    DayInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

#[derive(Clone, Copy)]
pub struct Birthday {
    month: Option<u8>,
    day: Option<u8>,
}

impl Birthday {
    pub fn new(month: u8, day: u8) -> Result<Self, BirthdayError> {
        match month {
            0 => Ok(Self {
                month: None,
                day: None,
            }),
            month if day == 0 => Ok(Self {
                month: Some(month),
                day: None,
            }),
            1 | 3 | 5 | 7 | 8 | 10 | 12 if day > 31 => Err(BirthdayError::DayInvalid),
            4 | 6 | 9 | 11 if day > 30 => Err(BirthdayError::DayInvalid),
            2 if day > 29 => Err(BirthdayError::DayInvalid),
            1..=12 => Ok(Self {
                month: Some(month),
                day: Some(day),
            }),
            _ => Err(BirthdayError::MonthInvalid),
        }
    }

    pub fn month(&self) -> Option<u8> {
        self.month
    }

    pub fn day(&self) -> Option<u8> {
        self.day
    }
}

impl FromByteHandler for Birthday {
    type Err = BirthdayError;
    /// Expects 0x00..=0x01
    /// Where M = Month and D = Day
    /// 1. XXMMMMDD
    /// 2. DDDXXXXX
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let mut handler = handler.try_into()?;
        handler.shift_right(2);
        Self::new(handler.copy_byte(0) & 0x0F, handler.copy_byte(1) >> 3)
    }
}
