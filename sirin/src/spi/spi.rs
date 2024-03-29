use core::future::Future;
use core::ops::{Deref, DerefMut};

use embassy_stm32::spi::{self as em_spi};

use embedded_hal_async::spi::{SpiBus, ErrorType};
use embedded_hal_async::spi as hal_spi;
use sirin_macros::SpiError;

use super::config::{SpiConfig, SpiConfigStruct};
//use super::spi_interface::SpiInterface;

pub type EmbassySpi<S, Tx, Rx> = embassy_stm32::spi::Spi<'static, S, Tx, Rx>;

#[derive(SpiError)]
pub struct SpiInstance<S: SpiConfig> {
    embassy_spi: EmbassySpi<S::Spi, S::TxDma, S::RxDma>
}

pub trait Spi: ErrorType<Error = SpiError> + SpiBus {}

impl <S: SpiConfig> Spi for SpiInstance<S> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiError {
    Overrun,
    ModeFault,
    FrameFormat,
    ChipSelectFault,
    Other,
}

impl SpiError {
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

impl <S: SpiConfig> SpiInstance<S> {
    pub fn new(config: SpiConfigStruct<S>) -> Self {
        Self {
            embassy_spi: EmbassySpi::new(
                config.spi,
                config.sck,
                config.mosi,
                config.miso,
                config.dma_tx,
                config.dma_rx,
                em_spi::Config::default()
            )
        }
    }
}

impl <S: SpiConfig> SpiBus for SpiInstance<S> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.embassy_spi.read(words).await.map_err(SpiError::from)
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.embassy_spi.write(words).await.map_err(SpiError::from)
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.embassy_spi.transfer(read, write).await.map_err(SpiError::from)
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.embassy_spi.transfer_in_place(words).await.map_err(SpiError::from)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        <EmbassySpi<S::Spi, S::TxDma, S::RxDma> as SpiBus<u8>>::flush(&mut self.embassy_spi)
            .await.map_err(SpiError::from)
    }
}