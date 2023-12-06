//#![feature(error_in_core)]
//#![feature(associated_type_defaults)]
#![no_std]

use core::panic;

use embassy_executor::Executor;
use embassy_stm32::{ Config };
use spi::spi::{ EmbassySpi, SpiInstance };

pub mod spi;

pub struct Peripherals {
    //pub spi: SpiInstance,
}

impl Peripherals {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            /*spi: SpiInstance::new(SpiPinsTrait {
                spi: p.SPI1,
                sck: p.PA5,
                miso: p.PA6,
                mosi: p.PA7,
                dma_tx: p.DMA1_CH1,
                dma_rx: p.DMA1_CH2,
            })*/
        }
    }
}

pub struct Sirin {
    pub peripherals: Peripherals,
    pub executor: Executor,
}

impl Sirin {
    pub fn new() -> Self {
        Self::from_config(Config::default())
    }

    pub fn from_config(config: Config) -> Self {
        let embassy_peripherals = embassy_stm32::init(config);

        Self {
            peripherals: Peripherals::new(embassy_peripherals),
            executor: Executor::new(),
        }
    }
}