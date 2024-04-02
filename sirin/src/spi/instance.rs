use embassy_stm32::spi::{ self as em_spi, SckPin, MisoPin, MosiPin, RxDma, TxDma, Instance as EmSpiInstance, Spi as EmSpi };
use embedded_hal_async::spi::SpiBus;
use sirin_macros::SpiError;

use super::SpiError;

pub trait SpiConfig {
    type Spi: EmSpiInstance;
    type Sck: SckPin<Self::Spi>;
    type Miso: MisoPin<Self::Spi>;
    type Mosi: MosiPin<Self::Spi>;
    type TxDma: TxDma<Self::Spi>;
    type RxDma: RxDma<Self::Spi>;
}

/// TODO: better name
pub struct SpiConfigStruct<S: SpiConfig> {
    pub spi: S::Spi,
    pub sck: S::Sck,
    pub miso: S::Miso,
    pub mosi: S::Mosi,
    pub dma_tx: S::TxDma,
    pub dma_rx: S::RxDma,
}

#[derive(SpiError)]
pub struct SpiInstance<S: SpiConfig> {
    embassy_spi: EmSpi<'static, S::Spi, S::TxDma, S::RxDma>
}

impl <S: SpiConfig> SpiInstance<S> {
    pub fn new(config: SpiConfigStruct<S>) -> Self {
        Self {
            embassy_spi: EmSpi::new(
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
        SpiBus::<u8>::flush(&mut self.embassy_spi).await.map_err(SpiError::from)
    }
}