//#![feature(error_in_core)]
//#![feature(associated_type_defaults)]
#![no_std]

use core::{mem::MaybeUninit, panic, ptr::addr_of_mut};

use bmp3::Bmp3;
use embassy_executor::{Executor, Spawner};
use embassy_stm32::{ gpio::{Level, Output, Speed}, spi as em_spi, time::mhz, Config, Peripherals };
use gpio::GpioPins;
use rfm9x::Rfm9x;
use w25q::W25Q;
use spi::{Spi, SpiConfig, SpiConfigStruct, SpiDev, SpiInstance, WithSpiHandle};

pub mod spi;
pub mod delay;
pub mod gpio;
pub mod sync;
pub mod triplet;

pub struct Sirin {
    //pub imu: Lsm6Dso,
    pub radio: Rfm9x<SpiDev>,
    //pub gps: S1315F8,
    pub spawner: Spawner,
    pub spi1: SpiInstance,
    pub spi2: SpiInstance,
    pub gpio: GpioPins,
    pub baro: Bmp3<SpiDev>,
    pub flash: W25Q<SpiDev>
}

impl Sirin {
    /// Initializing Sirin is a PITA bc it is a self-referential struct
    #[inline]
    pub async fn init(
        sirin: &'static mut MaybeUninit<Sirin>,
        spawner: Spawner
    ) -> &'static mut Sirin {
        unsafe {
            macro_rules! ptr {
                ($field: ident) => {
                    addr_of_mut!((*sirin.as_mut_ptr()).$field)
                };
            }

            *ptr!(spawner) = spawner;

            let mut config = Config::default();
            {
                use embassy_stm32::rcc::*;
                config.rcc.hsi = Some(HSIPrescaler::DIV1);
                config.rcc.csi = true;
                config.rcc.pll1 = Some(Pll {
                    source: PllSource::HSI,
                    prediv: PllPreDiv::DIV4,
                    mul: PllMul::MUL50,
                    divp: Some(PllDiv::DIV2),
                    divq: Some(PllDiv::DIV8), // used by SPI3. 100Mhz.
                    divr: None,
                });
                config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
                config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
                config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
                config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
                config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
                config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
                config.rcc.voltage_scale = VoltageScale::Scale1;
            }
            let p = embassy_stm32::init(config);

            let mut spi_config = em_spi::Config::default();
            spi_config.frequency = mhz(1);

            let spi1: *mut SpiInstance = ptr!(spi1);
            spi1.write(spi::SpiInstance::new(SpiConfigStruct {
                spi: p.SPI1,
                sck: p.PA5,
                miso: p.PA6,
                mosi: p.PA7,
                dma_tx: p.DMA1_CH3,
                dma_rx: p.DMA1_CH2,
                config: spi_config
            }));

            let spi2: *mut SpiInstance = ptr!(spi2);
            spi2.write(spi::SpiInstance::new(SpiConfigStruct {
                spi: p.SPI2,
                sck: p.PB13,
                miso: p.PB14,
                mosi: p.PB15,
                dma_tx: p.DMA1_CH5,
                dma_rx: p.DMA1_CH4,
                config: spi_config
            }));

            let gpio: *mut GpioPins = ptr!(gpio);
            gpio.write(GpioPins {
                p1: p.PA4,
                p2: p.PC4,
                p3: p.PC5,
                p4: p.PB0,
                p5: p.PB1,
                p6: p.PB2,
                p7: p.PE7,
                p8: p.PE8,
                p9: p.PE9,
                p10: p.PE10,
                p11: p.PD7,
                p12: p.PD6,
                p13: p.PD5,
                p14: p.PD4,
                p15: p.PD3,
                p16: p.PD1,
                p17: p.PD0,
                p18: p.PC12,
                p19: p.PC11,
                p20: p.PC10,
            });

            
            let baro: *mut Bmp3<SpiDev> = ptr!(baro);
            let baro_cs = Output::new(p.PA2, Level::High, Speed::High);
            let baro_future = Bmp3::new((*spi1).handle(baro_cs));

            let radio: *mut Rfm9x<SpiDev> = ptr!(radio);
            let radio_cs = Output::new(p.PC8, Level::High, Speed::High);
            radio.write(Rfm9x::new((*spi2).handle(radio_cs)));

            let flash: *mut W25Q<SpiDev> = ptr!(flash);
            let flash_cs = Output::new(p.PD2, Level::High, Speed::High);
            flash.write(W25Q::new((*spi2).handle(flash_cs)));

            // TODO: JOIN FUTURES, AWAIT
            baro.write(baro_future.await.unwrap());

            let sirin: &'static mut _ = sirin.assume_init_mut();
            sirin
        }
    }
}