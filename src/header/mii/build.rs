use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("Weight is invalid")]
    WeightInvalid,
    #[error("Height is invalid")]
    HeightInvalid,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

#[derive(Clone, Copy)]
pub struct Build {
    height: u8,
    weight: u8,
}

impl Build {
    #[inline(always)]
    fn new(height: u8, weight: u8) -> Result<Self, BuildError> {
        if height > 127 {
            return Err(BuildError::HeightInvalid);
        }
        if weight > 127 {
            return Err(BuildError::WeightInvalid);
        }
        Ok(Self { height, weight })
    }

    pub fn height(&self)->u8{self.height}
    pub fn weight(&self)->u8{self.weight}
}

impl FromByteHandler for Build {
    type Err = BuildError;

    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        let handler = handler.try_into()?;

        Self::new(handler.copy_byte(0), handler.copy_byte(1))
    }
}
