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

        for op in operations {
            match op {
                Operation::Read(buf) => spi.read(buf).await?,
                Operation::Write(buf) => spi.write(buf).await?,
                Operation::Transfer(read, write) => spi.transfer(read, write).await?,
                Operation::TransferInPlace(buf) => spi.transfer_in_place(buf).await?,
                Operation::DelayNs(time_ns) => delay_ns(*time_ns).await,
            }
        }

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

#[macro_export]
macro_rules! transact {
    ($device: ident, $($tx: tt)*) => {{
        let val = {
            let inst = $device.spi.borrow().await;
            $device.select();
            $device.
            $($tx)*
        }
        $device.deselect();
        val
    }};
}