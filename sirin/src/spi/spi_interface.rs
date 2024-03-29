use core::future::Future;

use embassy_stm32::{gpio::Output, spi::Spi as EmbassySpi};
use embedded_hal_async::spi::{SpiDevice, ErrorType, Operation, SpiBus};

use crate::delay::delay_ns;
use super::{config::SpiConfig, spi::{Spi, SpiError, SpiInterface}};

// I have found it easier to manage chip select manually so far,
// but it may be worth it to experiment with using [Drop] to force the
// user to assert CS (and then automatically de-assert).
pub trait SpiInterface<'a>: SpiDevice {
    type Spi: Spi + 'a;
    fn spi(&mut self) -> impl Future<Output = Self::Spi>;
    fn select(&mut self) -> impl Future<Output = Result<(), SpiError>>;
    fn deselect(&mut self) -> impl Future<Output = Result<(), SpiError>>;
}

impl <'a, S: SpiInterface> SpiInterface<'a> for SpiInterfaceStruct<'a, S> {
    type Spi = S::Spi;
    fn spi(&mut self) -> impl Future<Output = S::Spi> {
        self.spi.select()
    }

    async fn select(&mut self) -> Result<(), SpiError> {
        self.cs.set_low();
        Ok(())
    }

    async fn deselect(&mut self) -> Result<(), SpiError> {
        self.cs.set_high();
        Ok(())
    }
}

// bad name
pub struct SpiInterfaceStruct<'a, S: SpiInterface>  {
    spi: &'a S,
    cs: Output<'a>,
}

impl <S: SpiInterface> ErrorType for SpiInterfaceStruct<'_, S> {
    type Error = SpiError;
}

impl <S: SpiInterface> SpiDevice for SpiInterfaceStruct<'_, S> {
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, u8>]
    ) -> Result<(), Self::Error> {
        let mut spi = self.spi.select().await;
        self.cs.set_low();

        for op in operations {
            match op {
                Operation::Read(buf) => spi.read(buf).await?,
                Operation::Write(buf) => spi.write(buf).await?,
                Operation::Transfer(read, write) => spi.transfer(read, write).await?,
                Operation::TransferInPlace(buf) => spi.transfer_in_place(buf).await?,
                Operation::DelayNs(time_ns) => delay_ns(*time_ns).await,
            }
        }

        self.cs.set_high();

        Ok(())
    }
}