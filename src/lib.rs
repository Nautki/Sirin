use std::future::IntoFuture;

use embedded_hal_async::spi::ErrorKind;
use spi_handle::SpiHandle;
use embedded_hal_async::spi::SpiBus;

pub struct W25Q<S: SpiHandle>{
    spi: S
}

impl <S: SpiHandle> W25Q<S>{
    pub async fn write_enable(&mut self) -> Result<(), ErrorKind> {
        let mut spi = self.spi.select().await;
        spi.write(&[0x06]).await?;
        Ok(())
    }

    pub async fn write(&mut self, addr: u32, words: &[u8]) -> Result<(), ErrorKind> {
        // TODO: write `words` to `addr`
        let mut spi = self.spi.select().await;
        spi.write(&[0x02]).await?;
        spi.write(&[((addr >> 16) & 0xFF) as u8, ((addr >> 8) & 0xFF) as u8, ((addr & 0xFF) as u8)]).await?;
        spi.write(&[((addr >> 16) & 0xFF) as u8, ((addr >> 8) & 0xFF) as u8, ((addr & 0xFF) as u8)]).await?;
        spi.write(words).await?;
        Ok(())
    }//
}