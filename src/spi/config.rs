use embassy_stm32::spi::{ SckPin, MisoPin, MosiPin, RxDma, TxDma, Instance as SpiInstance };
use embassy_stm32::peripherals::*;

/// TODO: better name
pub trait SpiConfig {
    type Spi: SpiInstance;
    type Sck: SckPin<Self::Spi>;
    type Miso: MisoPin<Self::Spi>;
    type Mosi: MosiPin<Self::Spi>;
    type TxDma: RxDma<Self::Spi>;
    type RxDma: TxDma<Self::Spi>;

    fn spi(&self) -> Self::Spi;
    fn sck(&self) -> Self::Sck;
    fn miso(&self) -> Self::Miso;
    fn mosi(&self) -> Self::Mosi;
    fn dma_tx(&self) -> Self::TxDma;
    fn dma_rx(&self) -> Self::RxDma;
}

pub struct BasicSpiConfig<Spi, Sck, Miso, Mosi, TTxDma, TRxDma> where
    Spi: SpiInstance, 
    Sck: SckPin<Spi>,
    Miso: MisoPin<Spi>,
    Mosi: MosiPin<Spi>,
    TTxDma: TxDma<Spi>,
    TRxDma: RxDma<Spi>
{
    pub spi: Spi,
    pub sck: Sck,
    pub miso: Miso,
    pub mosi: Mosi,
    pub dma_tx: TTxDma,
    pub dma_rx: TRxDma,
}

impl <Spi, Sck, Miso, Mosi, TTxDma, TRxDma> SpiConfig for BasicSpiConfig<Spi, Sck, Miso, Mosi, TTxDma, TRxDma> where
    Spi: SpiInstance, 
    Sck: SckPin<Spi>,
    Miso: MisoPin<Spi>,
    Mosi: MosiPin<Spi>,
    TTxDma: TxDma<Spi>,
    TRxDma: RxDma<Spi>
{
    type Spi = Spi;
    type Sck = Sck;
    type Miso = Miso;
    type Mosi = Mosi;
    type TxDma = TTxDma;
    type RxDma = TRxDma;

    fn spi(&self) -> Self::Spi {
        self.spi
    }

    fn sck(&self) -> Self::Sck {
        self.sck
    }

    fn miso(&self) -> Self::Miso {
        self.miso
    }

    fn mosi(&self) -> Self::Mosi {
        self.mosi
    }

    fn dma_tx(&self) -> Self::TxDma {
        self.dma_tx
    }

    fn dma_rx(&self) -> Self::RxDma {
        self.dma_rx
    }
}