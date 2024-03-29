use embassy_stm32::spi::{ SckPin, MisoPin, MosiPin, RxDma, TxDma, Instance as SpiInstance };

pub trait SpiConfig {
    type Spi: SpiInstance;
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