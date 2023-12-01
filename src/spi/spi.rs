use embassy_stm32::dma::Channel;
use embassy_stm32::peripherals::{ DMA1_CH1, DMA1_CH2 };
use embassy_stm32::spi as em_spi;
use em_spi::Instance as SpiInstance;
use embassy_sync::channel::Channel;

use super::config::SpiConfig;

pub type EmbassySpi<S: SpiInstance> = embassy_stm32::spi::Spi<'static, S, DMA1_CH1, DMA1_CH2>;

pub type SpiChannel = Channel<TODO>;

pub struct Spi<S: SpiConfig> {
    embassy_spi: EmbassySpi<S::Spi>
}

impl <S: SpiConfig> Spi<S> {
    pub fn new(config: S) -> Self {
        Self {
            embassy_spi: EmbassySpi::<S>::new(
                config.spi(),
                config.sck(),
                config.mosi(),
                config.miso(),
                config.dma_tx(),
                config.dma_rx(),
                em_spi::Config::default()
            )
        }
    }
}