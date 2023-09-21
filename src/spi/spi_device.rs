use core::{future, fmt};
use core::error::Error;
use std::fmt::Display;
use async_trait::async_trait;

#[derive(Debug)]
pub enum SelectIoError<SelectError: Error, WriteError: Error> {
    SelectError(SelectError),
    WriteError(WriteError),
}

impl <SelectError: Error, WriteError: Error> Display for SelectIoError<SelectError, WriteError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelectError(e) => write!(f, "SelectError: {}", e),
            Self::WriteError(e) => write!(f, "WriteError: {}", e),
        }
    }
}

impl <SelectError: Error, WriteError: Error> Error for SelectIoError<SelectError, WriteError> {}

#[async_trait]
pub trait SpiDeviceConn {
    type Future<R>: future::Future<Output = R>;
    type WriteError: Error;
    type ReadError: Error;
    type SelectError: Error;
    type Selected: SelectedSpiDeviceConn<SpiDeviceConn = Self>;
    
    /// Automatically selects the device and then swaps the bytes.
    async fn write(&mut self, data: impl Iterator<Item = u8>)
        -> Self::Future<Result<(), Self::WriteError>> {
        let selected = self.select().await.map_err(SelectIoError::SelectError)?;
    }

    /// Automatically selects this device, then reads `num_bytes` bytes from the device.
    fn read<const num_bytes: usize>(&mut self)
        -> Self::Future<Result<[u8; num_bytes], Self::ReadError>>;

    /// Automatically selects this device, then swaps the bytes in the `data` buffer. 
    /// So each byte in the buffer is
    /// sent to the device, and the buffer is modified in-place with the returned bytes.
    fn swap<const num_bytes: usize>(&mut self, data: &[u8; num_bytes])
        -> Self::Future<Result<(), Self::WriteError>>;

    fn select(self) -> Self::Future<Result<Self::Selected, Self::SelectError>>;
}

pub trait SelectedSpiDeviceConn {
    type SpiDeviceConn: SpiDeviceConn;
    fn deselect(self)
        -> <Self::SpiDeviceConn as SpiDeviceConn>
        ::Future<
            Result<
                Self::SpiDeviceConn,
                <Self::SpiDeviceConn as SpiDeviceConn>::SelectError
            >
        >;

    fn write_byte(&mut self, byte: u8)
        -> <Self::SpiDeviceConn as SpiDeviceConn>
        ::Future<
            Result<
                (),
                <Self::SpiDeviceConn as SpiDeviceConn>::WriteError
            >
        >;

    fn read_byte(&mut self)
        -> <Self::SpiDeviceConn as SpiDeviceConn>
        ::Future<
            Result<
                u8,
                <Self::SpiDeviceConn as SpiDeviceConn>::ReadError
            >
        >;
}