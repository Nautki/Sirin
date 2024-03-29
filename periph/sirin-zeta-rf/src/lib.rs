use sirin::spi::{ config::SpiConfig, spi_interface::SpiInterface };

pub struct ZetaRf<'a, S: SpiConfig> {
    spi: SpiInterface<'a, S>
}

impl <'a, S: SpiConfig> ZetaRf<'a, S> {
    async fn cts_cmd(&self) -> {
        
    }
    pub async fn part_info(&self) -> {

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