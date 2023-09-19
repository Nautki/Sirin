use core::panic;

use embassy_executor::Executor;
use embassy_stm32::{ Config };

pub mod spi;

pub struct Peripherals {
    pub spi: EmbassySpi,
}

impl Peripherals {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {

        Self {
            spi: EmbassySpi::new(),
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
            peripherals: ,
            executor: Executor::new(),
        }
    }
}