#![no_std]

use sirin::{spi::{SpiError, SpiHandle}, sync::{Mutex, MutexGuard}};
use embedded_hal_async::spi::SpiBus;

struct CmdBufOwnership;

pub struct ZetaRf<S: SpiHandle> {
    spi: S,
    /// Flag which controls whether or not someone has data which will be showing
    /// up in the `CMD_BUF` soon.
    cmd_mutex: Mutex<CmdBufOwnership>
}


impl <S: SpiHandle> ZetaRf<S> {
    async fn cts_cmd<const N: usize>(&mut self) -> Result<[u8; N], SpiError> {
        self.cmd_mutex.lock().await;
        let mut spi = self.spi.select().await;

        // TODO: check this/make constants or smth
        spi.write(&[0x44]).await?;
        let mut out = [0u8; N];
        spi.read(&mut out).await?;
        Ok(out)
    }

    pub async fn part_info(&mut self) -> Result<PartInfo, SpiError> {
        let mut spi = self.spi.select().await;
        spi.write([0x01]).await?;

        let reply = self.cts_cmd::<10>().await?;
    }
}

#[repr(C)]
pub struct PartInfo {
    /// Chip Mask Revision
    chip_rev: u8,
    /// Part Number (e.g., si4461 will return - 0x4461).
    part: u16,
    /// Part Build.
    pbuild: u8,
    id: u16,
    customer: u8,
    rom_id: u8
}