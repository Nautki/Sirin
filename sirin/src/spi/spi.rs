use core::{future::Future, marker::PhantomData};

use derive_new::new;
use embassy_stm32::gpio::Output;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex};
use embedded_hal_async::spi::{self as hal_spi, ErrorType};
use sirin_macros::SpiError;

use crate::sync::{Mutex, MutexGuard};

use super::SpiError;

pub trait SpiBus: ErrorType<Error = SpiError> + hal_spi::SpiBus {}
impl <S: ErrorType<Error = SpiError> + hal_spi::SpiBus> SpiBus for S {}

#[derive(SpiError, new)]
pub struct SpiHandleMutex<'a, S: SpiBus> {
    spi: &'a Mutex<S>,
    cs: Output<'a>
}

pub trait SpiHandle {
    fn select<'a>(&'a mut self) -> impl Future<Output = impl SpiBus + 'a>;
}

impl <'a, S: SpiBus> SpiHandle for SpiHandleMutex<'a, S>
where Self: 'a {
    async fn select<'b>(&'b mut self) -> impl SpiBus + 'b {
        let spi = self.spi.lock().await;
        self.cs.set_low();
        SpiGuard::new(spi, &mut self.cs)
    }
}

#[derive(SpiError, new)]
pub struct SpiGuard<'a, S: SpiBus> {
    spi_mutex: MutexGuard<'a, S>,
    cs: *mut Output<'a>,
}

impl <'a, S: SpiBus> SpiGuard<'a, S> {
    // drops it lol
    pub fn deselect(self) {}
}

impl <S: SpiBus> Drop for SpiGuard<'_, S> {
    fn drop(&mut self) {
        // SAFETY: pin has lifetime 'a
        unsafe {
            (*self.cs).set_high();
        }
    }
}

impl <'a, S: SpiBus> hal_spi::SpiBus for SpiGuard<'a, S> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi_mutex.read(words).await.map_err(SpiError::from_kind)
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.spi_mutex.write(words).await.map_err(SpiError::from_kind)
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.spi_mutex.transfer(read, write).await.map_err(SpiError::from_kind)
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi_mutex.transfer_in_place(words).await.map_err(SpiError::from_kind)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.spi_mutex.flush().await.map_err(SpiError::from_kind)
    }
}