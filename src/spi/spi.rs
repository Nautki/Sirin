use embassy_stm32::peripherals::*;
use embassy_stm32::spi as em_spi;

pub type EmbassySpi = embassy_stm32::spi::Spi<'static, SPI1, DMA1_CH1, DMA1_CH2>;

pub struct SpiPins {
    pub spi: SPI1,
    pub sck: PA5,
    pub miso: PA6,
    pub mosi: PA7,
    pub dma_tx: DMA1_CH1,
    pub dma_rx: DMA1_CH2,
}

pub struct Spi {
    embassy_spi: EmbassySpi
}

impl Spi {
    pub fn new(pins: SpiPins) -> Self {
        Self::new_from_config(pins, em_spi::Config::default())
    }

    pub fn new_from_config(pins: SpiPins, config: em_spi::Config) -> Self {
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