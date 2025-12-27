use std::convert::Infallible;

use crate::byte_handler::{ByteHandlerError, FromByteHandler};

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("Nonexistent Controller ID")]
    NonexistentControllerID,
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
    #[error("")]
    Infallible(#[from] Infallible),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Controller {
    WiiWheel,
    Nunchuk,
    Classic,
    Gamecube,
}

impl FromByteHandler for Controller {
    type Err = ControllerError;
    /// Expects Header 0x0B
    fn from_byte_handler<T>(handler: T) -> Result<Self, Self::Err>
    where
        T: TryInto<crate::byte_handler::ByteHandler>,
        Self::Err: From<T::Error>,
    {
        (handler.try_into()?.copy_byte(0) & 0x0F).try_into()
    }
}

impl TryFrom<u8> for Controller {
    type Error = ControllerError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::WiiWheel),
            1 => Ok(Self::Nunchuk),
            2 => Ok(Self::Classic),
            3 => Ok(Self::Gamecube),
            _ => Err(ControllerError::NonexistentControllerID),
        }
    }
}

impl From<Controller> for u8 {
    fn from(value: Controller) -> Self {
        match value {
            Controller::WiiWheel => 0,
            Controller::Nunchuk => 1,
            Controller::Classic => 2,
            Controller::Gamecube => 3,
        }
    }
}
