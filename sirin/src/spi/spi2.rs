use core::{future::Future, marker::PhantomData};

use derive_new::new;
use embassy_stm32::gpio::Output;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embedded_hal_async::spi::SpiBus;

type Mutex<V> = embassy_sync::mutex::Mutex<CriticalSectionRawMutex, V>;
type MutexGuard<'a, V> = embassy_sync::mutex::MutexGuard<'a, CriticalSectionRawMutex, V>;

#[derive(Debug, Clone, new)]
pub struct SpiHandleMutex<'a, S: SpiBus> {
    spi: Mutex<S>,
    cs: Output<'a>
}

pub trait SpiHandle {
    fn select(&self) -> impl Future<Output = impl SpiBus>;
}

impl <'a, S: SpiBus> SpiHandle for SpiHandleMutex<'a, S> {
    async fn select(&self) -> SpiGuard<S> {
        let spi = self.spi.lock().await;
        self.cs.set_low();
        SpiGuard::new(spi, &mut self.cs)
    }
}

#[derive(new)]
pub struct SpiGuard<'a, S: SpiBus> {
    _phantom: PhantomData<&'a ()>,
    spi_mutex: MutexGuard<'a, S>,
    cs: *mut Output<'a>,
}

impl <'a, S: SpiBus> SpiGuard<'a, S> {
    // drops it lol
    fn deselect(self) {}
}

impl <S: SpiBus> Drop for SpiGuard<'_, S> {
    fn drop(&mut self) {
        // SAFETY: pin has lifetime 'a
        unsafe {
            (*self.cs).set_high();
        }
    }
}

impl <'a, S: SpiBus> SpiBus for SpiGuard<'a, S> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi_mutex.read(words).await.map_err(SpiError::from)
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.spi_mutex.write(words).await.map_err(SpiError::from)
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.spi_mutex.transfer(read, write).await.map_err(SpiError::from)
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi_mutex.transfer_in_place(words).await.map_err(SpiError::from)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        <EmbassySpi<S::Spi, S::TxDma, S::RxDma> as SpiBus<u8>>::flush(&mut self.spi_mutex)
            .await.map_err(SpiError::from)
    }
}