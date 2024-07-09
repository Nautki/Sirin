#![no_std]

use core::future::Future;
use embedded_hal_async::spi::SpiBus;

pub trait SpiHandle {
    /// The awaited return type MUST implement [Drop] to deselect the SPI bus.
    /// 
    /// The future resolves once the SPI device has been selected on the bus. 
    /// This method is infallible. It should not panic or fail. If the 
    /// device cannot be selected, the future should wait until it can
    /// be selected, then it should select it and resolve.
    /// 
    /// The returned [SpiBus] must guarantee exclusive access.
    fn select<'a>(&'a mut self) -> impl Future<Output = impl SpiBus + 'a>;
}