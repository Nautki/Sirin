use embassy_stm32::spi::{self as em_spi};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::{ Mutex, MutexGuard };

use super::config::{SpiConfig, SpiConfigStruct};

pub type EmbassySpi<S, Tx, Rx> = embassy_stm32::spi::Spi<'static, S, Tx, Rx>;
pub type MutexSpi<S> = Mutex<CriticalSectionRawMutex, SpiInstance<S>>;
pub type MutexGuardSpi<'a, S> = MutexGuard<'a, CriticalSectionRawMutex, SpiInstance<S>>;

pub struct Spi<S: SpiConfig> {
    mutex: MutexSpi<S>
}

impl <S: SpiConfig> Spi<S> {
    pub fn new(config: SpiConfigStruct<S>) -> Self {
        Self {
            mutex: MutexSpi::new(SpiInstance::new(config))
        }
    }

    pub async fn borrow(&self) -> MutexGuardSpi<S> {
        self.mutex.lock().await
    }
}

pub struct SpiInstance<S: SpiConfig> {
    embassy_spi: EmbassySpi<S::Spi, S::TxDma, S::RxDma>
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