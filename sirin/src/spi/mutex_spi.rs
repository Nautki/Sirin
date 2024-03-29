//! Mutex implementation of [Spi]. This is currently
//! the only implementation, but we may add more in the future.

use core::future::Future;

use embassy_stm32::gpio::Output;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::{ Mutex, MutexGuard };
use embedded_hal_async::spi::{ErrorType, Operation, SpiBus, SpiDevice};
use sirin_macros::SpiError;

use crate::delay::delay_ns;

use super::config::{SpiConfig, SpiConfigStruct};
use super::spi::{Spi, SpiError, SpiInterface, SpiInstance};

pub type MutexWithSpi<S> = Mutex<CriticalSectionRawMutex, SpiInstance<S>>;
//pub type MutexGuardSpi<'a, S> = MutexGuard<'a, CriticalSectionRawMutex, SpiInstance<S>>;

pub struct MutexSpi<S: SpiConfig> {
    mutex: MutexWithSpi<S>
}

impl <'a, S: SpiConfig + 'a> MutexSpi<S> {
    pub fn new(config: SpiConfigStruct<S>) -> Self {
        Self {
            mutex: MutexWithSpi::new(SpiInstance::new(config))
        }
    }

    pub async fn borrow(&self) -> MutexGuardSpi<S> {
        MutexGuardSpi {
            guard: self.mutex.lock().await
        }
    }
}

/*
impl <'a, S: SpiConfig> SpiInterface for &'a MutexSpi<S> {
    type Spi = MutexGuardSpi<'a, S>;
    fn select(&self) -> impl Future<Output = Self::Spi> {
        MutexSpi::borrow(&self)
    }
}

#[derive(SpiError)]
pub struct MutexGuardSpi<'a, S: SpiConfig + 'a> {
    guard: MutexGuard<'a, CriticalSectionRawMutex, SpiInstance<S>>
}

impl <'a, S: SpiConfig + 'a> Spi for MutexGuardSpi<'a, S> {}

impl <'a, S: SpiConfig + 'a> SpiBus for MutexGuardSpi<'a, S> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.guard.read(words).await
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.guard.write(words).await
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.guard.transfer(read, write).await
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.guard.transfer_in_place(words).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.guard.flush().await
    }
}*/