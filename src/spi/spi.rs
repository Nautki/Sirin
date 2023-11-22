use embassy_stm32::Peripheral;
use embassy_stm32::peripherals::*;
use embassy_stm32::spi as em_spi;

pub type EmbassySpi<S: Peripheral> = embassy_stm32::spi::Spi<'static, S, DMA1_CH1, DMA1_CH2>;

pub struct SpiPins<S: Peripheral> {
    pub spi: S,
    pub sck: PA5,
    pub miso: PA6,
    pub mosi: PA7,
    pub dma_tx: DMA1_CH1,
    pub dma_rx: DMA1_CH2,
}

pub struct Spi<S: Peripheral> {
    embassy_spi: EmbassySpi<S>
}

impl <S: Peripheral> Spi<S> {
    pub fn new(pins: SpiPins<S>) -> Self {
        Self::new_from_config(pins, em_spi::Config::default())
    }

    pub fn new_from_config(pins: SpiPins<S>, config: em_spi::Config) -> Self {
        Self {
            embassy_spi: EmbassySpi::new(
                pins.spi,
                pins.sck,
                pins.mosi,
                pins.miso,
                pins.dma_tx,
                pins.dma_rx,
                em_spi::Config::default()
            )
        }
    }
}