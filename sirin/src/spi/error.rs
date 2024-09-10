use core::future::Future;
use core::ops::{Deref, DerefMut};

use embassy_stm32::spi::{self as em_spi};

use embedded_hal_async::spi::{SpiBus, ErrorType};
use embedded_hal_async::spi as hal_spi;
use sirin_macros::SpiError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiError {
    Overrun,
    ModeFault,
    FrameFormat,
    /// This error is likely impossible
    ChipSelectFault,
    Other,
}

impl SpiError {
    // no `From` impl bc of conflicting impl in core
    pub fn from_kind(err: impl hal_spi::Error) -> Self {
        match err.kind() {
            hal_spi::ErrorKind::Overrun => SpiError::Overrun,
            hal_spi::ErrorKind::ModeFault => SpiError::ModeFault,
            hal_spi::ErrorKind::FrameFormat => SpiError::FrameFormat,
            hal_spi::ErrorKind::ChipSelectFault => SpiError::ChipSelectFault,
            _ => SpiError::Other,
        }
    }
}

impl From<em_spi::Error> for SpiError {
    fn from(err: em_spi::Error) -> Self {
        match err {
            em_spi::Error::Overrun => SpiError::Overrun,
            em_spi::Error::ModeFault => SpiError::ModeFault,
            em_spi::Error::Framing => SpiError::FrameFormat,
            em_spi::Error::Crc => SpiError::Other,
        }
    }
}

impl hal_spi::Error for SpiError {
    fn kind(&self) -> hal_spi::ErrorKind {
        match self {
            SpiError::Overrun => hal_spi::ErrorKind::Overrun,
            SpiError::ModeFault => hal_spi::ErrorKind::ModeFault,
            SpiError::FrameFormat => hal_spi::ErrorKind::FrameFormat,
            SpiError::ChipSelectFault => hal_spi::ErrorKind::ChipSelectFault,
            SpiError::Other => hal_spi::ErrorKind::Other,
        }
    }
}