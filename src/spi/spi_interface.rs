use embassy_stm32::gpio::{AnyPin, Output};
use embedded_hal_async::spi::{SpiDevice, ErrorType, Operation, SpiBus};

use crate::delay::delay_ns;
use super::{spi::{Spi, SpiError}, config::SpiConfig};

pub trait SpiInterfaceTrait<'a, S: SpiConfig + 'a>: SpiDevice {
    async fn spi(&mut self) -> &'a Spi<S>;
    async fn select(&mut self) -> Result<(), SpiError>;
    async fn deselect(&mut self) -> Result<(), SpiError>;
}

pub struct SpiInterface<'a, S: SpiConfig>  {
    spi: &'a Spi<S>,
    pin: Output<'a, AnyPin>,
}

impl <S: SpiConfig> ErrorType for SpiInterface<'_, S> {
    type Error = SpiError;
}

impl <S: SpiConfig> SpiDevice for SpiInterface<'_, S> {
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, u8>]
    ) -> Result<(), Self::Error> {
        let mut spi = self.spi.borrow().await;
        self.pin.set_low();

        for op in operations {
            match op {
                Operation::Read(buf) => spi.read(buf).await?,
                Operation::Write(buf) => spi.write(buf).await?,
                Operation::Transfer(read, write) => spi.transfer(read, write).await?,
                Operation::TransferInPlace(buf) => spi.transfer_in_place(buf).await?,
                Operation::DelayNs(time_ns) => delay_ns(*time_ns).await,
            }
        }

        self.pin.set_high();

        Ok(())
    }
}

impl <'a, S: SpiConfig> SpiInterfaceTrait<'a, S> for SpiInterface<'a, S> {
    async fn spi(&mut self) -> &'a Spi<S> {
        self.spi
    }

    async fn select(&mut self) -> Result<(), SpiError> {
        self.pin.set_low();
        Ok(())
    }

    async fn deselect(&mut self) -> Result<(), SpiError> {
        self.pin.set_high();
        Ok(())
    }
}

/// Macro to simplify transactions. Requires that there is a variable `self` with property `spi_interface`.
macro_rules! t {
    ($()*) => {
        {
            let slf = $crate::unhygenic!(self)
        }
    };
}