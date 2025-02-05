#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use core::{fmt::Debug, mem};
use defmt::info;
use dev_csr::dev_csr;
use embassy_futures::yield_now;
use embedded_hal::spi::{ErrorKind as SpiError, ErrorType};
use embedded_hal_async::spi::SpiBus;
use spi_handle::SpiHandle;
use uunit::{Milliamperes, Milliamps, WithUnits};
const _: u8 = 0;
const _: u8 = 0;
pub struct RegFifo;
impl RegFifo {
    pub const ADDR: u8 = 0x00;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegFifo {
    fn as_addr(&self) -> u8 {
        0x00
    }
}
impl ReadableAddr for RegFifo {}
impl WritableAddr for RegFifo {}
pub struct RegOpMode;
impl RegOpMode {
    pub const ADDR: u8 = 0x01;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegOpMode {
    fn as_addr(&self) -> u8 {
        0x01
    }
}
impl ReadableAddr for RegOpMode {}
impl WritableAddr for RegOpMode {}
pub struct RegFrMsb;
impl RegFrMsb {
    pub const ADDR: u8 = 0x06;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegFrMsb {
    fn as_addr(&self) -> u8 {
        0x06
    }
}
impl ReadableAddr for RegFrMsb {}
impl WritableAddr for RegFrMsb {}
pub struct RegFrMid;
impl RegFrMid {
    pub const ADDR: u8 = 0x07;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegFrMid {
    fn as_addr(&self) -> u8 {
        0x07
    }
}
impl ReadableAddr for RegFrMid {}
impl WritableAddr for RegFrMid {}
pub struct RegFrLsb;
impl RegFrLsb {
    pub const ADDR: u8 = 0x08;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegFrLsb {
    fn as_addr(&self) -> u8 {
        0x08
    }
}
impl ReadableAddr for RegFrLsb {}
impl WritableAddr for RegFrLsb {}
pub struct RegPaConfig;
impl RegPaConfig {
    pub const ADDR: u8 = 0x09;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegPaConfig {
    fn as_addr(&self) -> u8 {
        0x09
    }
}
impl ReadableAddr for RegPaConfig {}
impl WritableAddr for RegPaConfig {}
pub struct RegPaRamp;
impl RegPaRamp {
    pub const ADDR: u8 = 0x0A;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegPaRamp {
    fn as_addr(&self) -> u8 {
        0x0A
    }
}
impl ReadableAddr for RegPaRamp {}
impl WritableAddr for RegPaRamp {}
pub struct RegOcp;
impl RegOcp {
    pub const ADDR: u8 = 0x0B;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegOcp {
    fn as_addr(&self) -> u8 {
        0x0B
    }
}
impl ReadableAddr for RegOcp {}
impl WritableAddr for RegOcp {}
pub struct RegLna;
impl RegLna {
    pub const ADDR: u8 = 0x0C;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegLna {
    fn as_addr(&self) -> u8 {
        0x0C
    }
}
impl ReadableAddr for RegLna {}
impl WritableAddr for RegLna {}
pub struct RegFifoAddrPtr;
impl RegFifoAddrPtr {
    pub const ADDR: u8 = 0x0D;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegFifoAddrPtr {
    fn as_addr(&self) -> u8 {
        0x0D
    }
}
impl ReadableAddr for RegFifoAddrPtr {}
impl WritableAddr for RegFifoAddrPtr {}
pub struct RegFifoTxBaseAddr;
impl RegFifoTxBaseAddr {
    pub const ADDR: u8 = 0x0E;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegFifoTxBaseAddr {
    fn as_addr(&self) -> u8 {
        0x0E
    }
}
impl ReadableAddr for RegFifoTxBaseAddr {}
impl WritableAddr for RegFifoTxBaseAddr {}
pub struct RegFifoRxBaseAddr;
impl RegFifoRxBaseAddr {
    pub const ADDR: u8 = 0x0F;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegFifoRxBaseAddr {
    fn as_addr(&self) -> u8 {
        0x0F
    }
}
impl ReadableAddr for RegFifoRxBaseAddr {}
impl WritableAddr for RegFifoRxBaseAddr {}
pub struct RegFifoRxCurrentAddr;
impl RegFifoRxCurrentAddr {
    pub const ADDR: u8 = 0x10;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegFifoRxCurrentAddr {
    fn as_addr(&self) -> u8 {
        0x10
    }
}
impl ReadableAddr for RegFifoRxCurrentAddr {}
pub struct RegIrqFlagsMask;
impl RegIrqFlagsMask {
    pub const ADDR: u8 = 0x11;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegIrqFlagsMask {
    fn as_addr(&self) -> u8 {
        0x11
    }
}
impl ReadableAddr for RegIrqFlagsMask {}
impl WritableAddr for RegIrqFlagsMask {}
pub struct RegIrqFlags;
impl RegIrqFlags {
    pub const ADDR: u8 = 0x12;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegIrqFlags {
    fn as_addr(&self) -> u8 {
        0x12
    }
}
impl ReadableAddr for RegIrqFlags {}
impl WritableAddr for RegIrqFlags {}
pub struct RegRxNbBytes;
impl RegRxNbBytes {
    pub const ADDR: u8 = 0x13;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegRxNbBytes {
    fn as_addr(&self) -> u8 {
        0x13
    }
}
impl ReadableAddr for RegRxNbBytes {}
pub struct RegRxHeaderCntValueMsb;
impl RegRxHeaderCntValueMsb {
    pub const ADDR: u8 = 0x14;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegRxHeaderCntValueMsb {
    fn as_addr(&self) -> u8 {
        0x14
    }
}
impl ReadableAddr for RegRxHeaderCntValueMsb {}
pub struct RegRxHeaderCntValueLsb;
impl RegRxHeaderCntValueLsb {
    pub const ADDR: u8 = 0x15;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegRxHeaderCntValueLsb {
    fn as_addr(&self) -> u8 {
        0x15
    }
}
impl ReadableAddr for RegRxHeaderCntValueLsb {}
pub struct RegRxPacketCntValueMsb;
impl RegRxPacketCntValueMsb {
    pub const ADDR: u8 = 0x16;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegRxPacketCntValueMsb {
    fn as_addr(&self) -> u8 {
        0x16
    }
}
impl ReadableAddr for RegRxPacketCntValueMsb {}
impl WritableAddr for RegRxPacketCntValueMsb {}
pub struct RegRxPacketCntValueLsb;
impl RegRxPacketCntValueLsb {
    pub const ADDR: u8 = 0x17;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegRxPacketCntValueLsb {
    fn as_addr(&self) -> u8 {
        0x17
    }
}
impl ReadableAddr for RegRxPacketCntValueLsb {}
pub struct RegModemStat;
impl RegModemStat {
    pub const ADDR: u8 = 0x18;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegModemStat {
    fn as_addr(&self) -> u8 {
        0x18
    }
}
impl ReadableAddr for RegModemStat {}
pub struct RegPktSnrValue;
impl RegPktSnrValue {
    pub const ADDR: u8 = 0x19;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegPktSnrValue {
    fn as_addr(&self) -> u8 {
        0x19
    }
}
impl ReadableAddr for RegPktSnrValue {}
pub struct RegPktRssiValue;
impl RegPktRssiValue {
    pub const ADDR: u8 = 0x1A;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegPktRssiValue {
    fn as_addr(&self) -> u8 {
        0x1A
    }
}
impl ReadableAddr for RegPktRssiValue {}
pub struct RegRssiValue;
impl RegRssiValue {
    pub const ADDR: u8 = 0x1B;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegRssiValue {
    fn as_addr(&self) -> u8 {
        0x1B
    }
}
impl ReadableAddr for RegRssiValue {}
pub struct RegHopChannel;
impl RegHopChannel {
    pub const ADDR: u8 = 0x1C;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegHopChannel {
    fn as_addr(&self) -> u8 {
        0x1C
    }
}
impl ReadableAddr for RegHopChannel {}
pub struct RegModemConfig1;
impl RegModemConfig1 {
    pub const ADDR: u8 = 0x1D;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegModemConfig1 {
    fn as_addr(&self) -> u8 {
        0x1D
    }
}
impl ReadableAddr for RegModemConfig1 {}
impl WritableAddr for RegModemConfig1 {}
pub struct RegModemConfig2;
impl RegModemConfig2 {
    pub const ADDR: u8 = 0x1E;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegModemConfig2 {
    fn as_addr(&self) -> u8 {
        0x1E
    }
}
impl ReadableAddr for RegModemConfig2 {}
impl WritableAddr for RegModemConfig2 {}
pub struct RegSymbTimeoutLsb;
impl RegSymbTimeoutLsb {
    pub const ADDR: u8 = 0x1F;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegSymbTimeoutLsb {
    fn as_addr(&self) -> u8 {
        0x1F
    }
}
impl ReadableAddr for RegSymbTimeoutLsb {}
impl WritableAddr for RegSymbTimeoutLsb {}
pub struct RegPreambleMsb;
impl RegPreambleMsb {
    pub const ADDR: u8 = 0x20;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegPreambleMsb {
    fn as_addr(&self) -> u8 {
        0x20
    }
}
impl ReadableAddr for RegPreambleMsb {}
impl WritableAddr for RegPreambleMsb {}
pub struct RegPreambleLsb;
impl RegPreambleLsb {
    pub const ADDR: u8 = 0x21;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegPreambleLsb {
    fn as_addr(&self) -> u8 {
        0x21
    }
}
impl ReadableAddr for RegPreambleLsb {}
impl WritableAddr for RegPreambleLsb {}
pub struct RegPayloadLength;
impl RegPayloadLength {
    pub const ADDR: u8 = 0x22;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegPayloadLength {
    fn as_addr(&self) -> u8 {
        0x22
    }
}
impl ReadableAddr for RegPayloadLength {}
impl WritableAddr for RegPayloadLength {}
pub struct RegMaxPayloadLength;
impl RegMaxPayloadLength {
    pub const ADDR: u8 = 0x23;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegMaxPayloadLength {
    fn as_addr(&self) -> u8 {
        0x23
    }
}
impl ReadableAddr for RegMaxPayloadLength {}
impl WritableAddr for RegMaxPayloadLength {}
pub struct RegHopPeriod;
impl RegHopPeriod {
    pub const ADDR: u8 = 0x24;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegHopPeriod {
    fn as_addr(&self) -> u8 {
        0x24
    }
}
impl ReadableAddr for RegHopPeriod {}
impl WritableAddr for RegHopPeriod {}
pub struct RegFifoRxByteAddr;
impl RegFifoRxByteAddr {
    pub const ADDR: u8 = 0x25;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegFifoRxByteAddr {
    fn as_addr(&self) -> u8 {
        0x25
    }
}
impl ReadableAddr for RegFifoRxByteAddr {}
pub struct RegModemConfig3;
impl RegModemConfig3 {
    pub const ADDR: u8 = 0x26;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegModemConfig3 {
    fn as_addr(&self) -> u8 {
        0x26
    }
}
impl ReadableAddr for RegModemConfig3 {}
impl WritableAddr for RegModemConfig3 {}
pub struct RegPpmCorrection;
impl RegPpmCorrection {
    pub const ADDR: u8 = 0x27;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegPpmCorrection {
    fn as_addr(&self) -> u8 {
        0x27
    }
}
impl ReadableAddr for RegPpmCorrection {}
impl WritableAddr for RegPpmCorrection {}
pub struct RegFeiMsb;
impl RegFeiMsb {
    pub const ADDR: u8 = 0x28;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegFeiMsb {
    fn as_addr(&self) -> u8 {
        0x28
    }
}
impl ReadableAddr for RegFeiMsb {}
pub struct RegFeiMid;
impl RegFeiMid {
    pub const ADDR: u8 = 0x29;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegFeiMid {
    fn as_addr(&self) -> u8 {
        0x29
    }
}
impl ReadableAddr for RegFeiMid {}
pub struct RegFeiLsb;
impl RegFeiLsb {
    pub const ADDR: u8 = 0x2A;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegFeiLsb {
    fn as_addr(&self) -> u8 {
        0x2A
    }
}
impl ReadableAddr for RegFeiLsb {}
pub struct RegRssiWideBand;
impl RegRssiWideBand {
    pub const ADDR: u8 = 0x2C;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegRssiWideBand {
    fn as_addr(&self) -> u8 {
        0x2C
    }
}
impl ReadableAddr for RegRssiWideBand {}
pub struct RegIfFreq2;
impl RegIfFreq2 {
    pub const ADDR: u8 = 0x2F;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegIfFreq2 {
    fn as_addr(&self) -> u8 {
        0x2F
    }
}
impl ReadableAddr for RegIfFreq2 {}
impl WritableAddr for RegIfFreq2 {}
pub struct RegVersion;
impl RegVersion {
    pub const ADDR: u8 = 0x42;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsRfm9xAddr for RegVersion {
    fn as_addr(&self) -> u8 {
        0x42
    }
}
impl ReadableAddr for RegVersion {}
pub struct RegPaDac;
impl RegPaDac {
    pub const ADDR: u8 = 0x4D;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsRfm9xAddr for RegPaDac {
    fn as_addr(&self) -> u8 {
        0x4D
    }
}
impl ReadableAddr for RegPaDac {}
impl WritableAddr for RegPaDac {}
pub trait AsRfm9xAddr {
    fn as_addr(&self) -> u8;
}
pub trait ReadableAddr: AsRfm9xAddr {}
pub trait WritableAddr: AsRfm9xAddr {}
pub trait ReadableValue<T> {
    fn from_value(value: T) -> Self;
}
pub trait WritableValue<T> {
    fn into_value(self) -> T;
}
impl AsRfm9xAddr for u8 {
    fn as_addr(&self) -> u8 {
        *self
    }
}
impl ReadableAddr for u8 {}
impl WritableAddr for u8 {}
pub trait ReadRfm9x {
    type Error;
    fn read_contiguous_regs(
        &mut self,
        addr: impl ReadableAddr,
        out: &mut [u8],
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>>;
    fn read_reg(
        &mut self,
        addr: impl ReadableAddr,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let mut out = [0];
            match self.read_contiguous_regs(addr, &mut out).await {
                Ok(_) => Ok(out[0]),
                Err(err) => Err(err),
            }
        }
    }
    /// Error coding rate
    /// 001 -> 4/5
    /// 010 -> 4/6
    /// 011 -> 4/7
    /// 100 -> 4/8
    /// All other values -> reserved
    /// In implicit header mode should be set on receiver todetermineexpected coding rate. See 4.1.1.3
    fn coding_rate(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegModemConfig1).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (3 + 1))) as u8) >> 1) << 0;
            Ok(_0)
        }
    }
    /// Number of valid headers received since last transition intoRx mode, LSB(7:0). Header and packet counters are reseted in Sleep mode.
    /// Number of valid headers received since last transition intoRx mode, MSB(15:8). Header and packet counters are reseted in Sleep mode
    fn valid_header_cnt(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegRxHeaderCntValueLsb).await?;
            let _2 = self.read_reg(RegRxHeaderCntValueMsb).await?;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    /// Current value of RX databuffer pointer (address of last byte written by Lora receiver)
    fn fifo_rx_byte_addr_ptr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegFifoRxByteAddr)
    }
    /// LoRa base-band FIFO data input/output. FIFO is cleared an not
    /// accessible when device is in SLEEPmode
    fn fifo(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegFifo)
    }
    /// Trimming of OCP current:
    /// Imax = 45 + 5 * OcpTrim[mA] if OcpTrim <= 15 (120mA)/
    /// Imax = -30 + 10 * OcpTrim[mA] if 15 < OcpTrim <= 27 (130 to 240mA)
    /// Imax = 240mA for higher settings
    /// Default Imax=100mA
    fn ocp_on(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegOcp).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Number of payload bytes of latest packet received
    fn fifo_rx_nb_bytes(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegRxNbBytes).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    ///Start address (in data buffer) of last packet received
    fn fifo_rx_current_addr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoRxCurrentAddr).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    /// 0 -> LNA gain set by register LnaGain
    /// 1 -> LNA gain set by the internal AGC loo
    fn acg_auto_on(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemConfig3).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// 2 RX on-going
    fn rx_on_going(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemStat).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    fn frf_15_8(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFrMid).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    /// 0 -> Disabled
    /// 1 -> Enabled; mandated for when the symbol lengthexceeds16ms
    fn low_data_rate_optimize(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemConfig3).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Packet reception complete interrupt: writing a 1 clears the IRQ
    fn rx_done(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlags).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Version code of the chip. Bits 7-4 give the full revision number;
    /// bits 3-0 give the metal mask revision number.
    fn version(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegVersion)
    }
    /// FIFO Payload transmission complete interrupt: writing a 1 clears the IRQ
    fn tx_done(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlags).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// FHSS change channel interrupt: writing a 1 clears the IR
    fn fhss_change_channel(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlags).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Signal detected
    fn signal_detected(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemStat).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Current RSSI value (dBm)
    /// RSSI[dBm] = -157 + Rssi (using HF outputport)
    /// or RSSI[dBm] = -164 + Rssi (using LF outputport)
    /// (see section 5.5.5 for details*/
    fn rssi(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegRssiValue).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    ///read base address in FIFO data buffer for RX demodulator
    fn fifo_rx_base_addr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoRxBaseAddr).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    /// Symbol periods between frequency hops. (0 = disabled). 1st hop always happen after the 1st header symbol
    fn frew_hopping_period(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegHopPeriod).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    ///Packet reception complete interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
    fn rx_done_mask(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlagsMask).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// 3 Header info valid
    fn header_info_valid(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemStat).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Select max output power.
    /// PMax = 10.8 + 0.6*MaxPower [dBm]
    fn max_power(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegPaConfig).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (6 + 1))) as u8) >> 4) << 0;
            Ok(_0)
        }
    }
    /// PLL failed to lock while attempting a TX/RX/CAD operation
    /// 1 -> PLL did not lock
    /// 0 -> PLL did lock
    fn pll_time_out(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegHopChannel).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// 4 Modem clear
    fn modem_clear(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemStat).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// LSB of RF Frequency Error
    /// Middle byte of RF FrequencyError
    /// Estimated frequency error from modem
    /// MSB of RF Frequency Error
    /// F_error = (FreqError * 2^24)/F_xtal * BW[kHz]/500
    fn freq_error(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u32, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFeiLsb).await?;
            let _2 = self.read_reg(RegFeiMid).await?;
            let _3 = self.read_reg(RegFeiMsb).await?;
            let mut _0: u32 = 0;
            _0 += (_1 as u32) << 0;
            _0 += (_2 as u32) << 8;
            _0 += (((_3 % (1 << (3 + 1))) as u32) >> 0) << 16;
            Ok(_0)
        }
    }
    /// Estimation of SNR on last packet received.In two’s compliment format multiplied by 4. SNR[dB] = PacketSnr[twos complement]/4
    fn packet_snr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegPktSnrValue).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    /// Payload length in bytes. The register needs to be set in implicit header mode for the expected packet length. A 0 value is not permitted
    fn payload_length(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegPayloadLength).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    /// Number of valid packets received since last transition intoRx mode, LSB(7:0). Header and packet counters are reseted in Sleep mode
    /// Number of valid packets received since last transition into Rx mode, MSB(15:8). Header and packet counters are reseted in Sleep mode.
    fn valid_packet_cnt(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegRxPacketCntValueLsb).await?;
            let _2 = self.read_reg(RegRxPacketCntValueMsb).await?;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    /// Current value of frequency hopping channel inuse.
    fn fhss_present_channel(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegHopChannel).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (5 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    /// Selects PA output pin
    /// 0: RFO pin. Output power is limited to +14dBm
    /// 1: PA_BOOST pin. Output power is limited to +20 dBm
    fn pa_select(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegPaConfig).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Wideband RSSI measurement used to locally generate a random number
    fn rssi_wide_band(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegRssiWideBand)
    }
    /// Signal bandwidth:
    /// 0000 -> 7.8 kHz
    /// 0001 -> 10.4 kHz
    /// 0010 -> 15.6 kHz
    /// 0011 -> 20.8kHz
    /// 0100 -> 31.25 kHz
    /// 0101 -> 41.7 kHz
    /// 0110 -> 62.5 kHz
    /// 0111 -> 125 kHz
    /// 1000 -> 250 kHz
    /// 1001 -> 500 kHz
    /// other values -> reserved
    /// In the lower band (169MHz), signal bandwidths 8&9 are not supported)
    fn bw(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegModemConfig1).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (7 + 1))) as u8) >> 4) << 0;
            Ok(_0)
        }
    }
    /// Timeout interrupt: writing a 1 clears the IRQ
    fn rx_time_out(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlags).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Maximum payload length; if header payload length exceeds value a header CRC error is generated. Allows filtering of packet with a bad size.
    fn payload_max_length(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegMaxPayloadLength).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    /// LSB or RF carrier frequency
    fn frf_7_0(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFrLsb).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    fn ocp_trim(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegOcp).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (4 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    fn pa_dac(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegPaDac)
    }
    /// 1 Signal synchronized
    fn signal_synchronized(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemStat).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Valid header received in Rx mask: setting this bit masks the corresponding IRQ in RegIrqFlags
    fn valid_header_mask(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlagsMask).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// 0 is FSK Mode, 1 is LoRA mode
    /// Can only be modified in sleep mode
    /// Write opperation on other devices is ignored
    fn long_range_mode(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegOpMode).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// POut = PMax - (15-OutputPower) if PaSelect = 0 (RFO pin)
    /// POut = 17 - (15-OutputPower) if PaSelect = 1 (PA_BOOST pin)
    fn output_power(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegPaConfig).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (3 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///FIFO Payload transmission complete interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
    fn tx_done_mask(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlagsMask).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Access Low Frequency Mode registers
    /// 0: High Frequency Mode (access to HF test registers)
    /// 1: Low Frequency Mode(access to LF test registers)
    fn low_freq_mode_on(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegOpMode).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Cad Detected Interrupt Mask: setting this bit masks the corresponding IRQ in RegIrqFlags
    fn cad_detected_mask(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlagsMask).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///High Frequency (RFI_HF) LNA currentadjustment, 00 -> Default LNA current
    ///11 -> Boost on, 150% LNA current
    fn lna_boost_hf(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegLna).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (1 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///LNA gain setting:
    ///000 -> not used
    ///001 -> G1 = maximum gain 010 G2
    ///011 -> G 3
    ///100 -> G4
    ///101 -> G5
    ///110 -> G6 = minimum gain
    ///111  -> not used
    fn lna_gain(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegLna).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (7 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    /// Device modes
    /// 000 : SLEEP
    /// 001 : STDBY
    /// 010 : Frequency synthesis TX(FSTX)
    /// 011 : Transmit(TX)
    /// 100 : Frequency synthesis RX(FSRX)
    /// 101 : Receive continuous(RXCONTINUOUS)
    /// 110 : receive single(RXSINGLE)
    /// 111 : Channel activity detection(CAD)
    fn mode(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegOpMode).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (2 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///write base address in FIFO data buffer for TX modulator
    fn fifo_tx_base_addr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoTxBaseAddr).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    /// Enable CRC generation and check onpayload:
    /// 0 -> CRC disable
    /// 1 -> CRC enable
    /// If CRC is needed, RxPayloadCrcOn should beset:
    /// - in Implicit header mode: on Tx and Rx side
    /// - in Explicit header mode: on the Tx side alone (recoveredfromtheheader in Rx side)
    fn rx_payload_crc_on(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemConfig2).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// CAD complete: write to clear: writing a 1 clears the IRQ
    fn cad_done(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlags).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///CAD complete interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
    fn cad_done_mask(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlagsMask).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Coding rate of last headerreceived
    fn rx_coding_rate(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegModemStat).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (7 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    /// Valid header received in Rx: writing a 1 clears theIRQ
    fn valid_header(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlags).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Rise/Fall time of ramp up/down in FSK
    /// 0000 -> 3.4ms
    /// 0001 -> 2ms
    /// 0010 -> 1ms
    /// 0011 -> 500us
    /// 0100 -> 250us
    /// 0101 -> 125us
    /// 0110 -> 100us
    /// 0111 -> 62us
    /// 1000 -> 50us
    /// 1001 -> 40us
    /// 1010 -> 31us
    /// 1011 -> 25us
    /// 1100 -> 20us
    /// 1101 -> 15us
    /// 1110 -> 12us
    /// 1111 -> 10us
    fn pa_ramp(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegPaRamp)
    }
    /// RX Time-Out LSB
    /// RX operation time-out value expressed as number of symbols:
    /// TimeOut = SymbTimeout * Ts
    /// RX Time-Out MSB
    fn symb_timeout(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegSymbTimeoutLsb).await?;
            let _2 = self.read_reg(RegModemConfig2).await?;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (((_2 % (1 << (1 + 1))) as u16) >> 0) << 8;
            Ok(_0)
        }
    }
    ///FHSS change channel interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
    fn fhss_change_channel_mask(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlagsMask).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///SPI interface address pointer in FIFO data buffer.
    fn fifo_addr_ptr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoAddrPtr).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    ///Timeout interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
    fn rx_timeout_mask(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlagsMask).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// 0 -> normal mode, a single packet is sent
    /// 1 -> continuous mode, send multiple packets across the FIFO(used for spectral analysis)
    fn tx_continuous_mode(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemConfig2).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Low Frequency (RFI_LF) LNA current adjustment, 00 -> Default LNA current
    ///Other -> Reserved
    fn lna_boost_lf(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegLna).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (4 + 1))) as u8) >> 3) << 0;
            Ok(_0)
        }
    }
    /// MSB or FR carrier frequency
    fn frf_23_16(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFrMsb).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    /// Valid Lora signal detected during CAD operation: writing a 1clears the IR
    fn cad_detected(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlags).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// 0 -> Explicit Header mode
    /// 1 -> Implicit Header mode
    fn implicit_header_mode_on(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegModemConfig1).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Payload CRC error interrupt: writing a 1 clears the IR
    fn payload_crc_err(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlags).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// Preamble Length LSB
    /// Preamble length MSB, = PreambleLength + 4.25Symbols
    /// See 4.1.1 for more details
    fn preamble_length(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegPreambleLsb).await?;
            let _2 = self.read_reg(RegPreambleMsb).await?;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    /// Data rate offset value, used in conjunction with AFC
    fn ppm_correction(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegPpmCorrection)
    }
    /// CRC Information extracted from the received packetheader
    /// (Explicit header mode only)
    /// 0 -> Header indicates CRC off
    /// 1 -> Header indicates CRC on
    fn crc_on_payload(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegHopChannel).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// RSSI of the latest packet received (dBm):
    ///RSSI[dBm] = -157 + Rssi (using HF output port, SNR >=0)
    ///or RSSI[dBm] = -164 + Rssi (using LF output port, SNR >= 0)
    /// (see section 5.5.5 for details)
    fn packet_rssi(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegPktRssiValue).await?;
            let mut _0: u8 = 0;
            _0 += (_1 as u8) << 0;
            Ok(_0)
        }
    }
    /// This bit operates when device is in LoRa mode;
    /// if set it allows access to FSK registers page
    /// located in address space (0x0D:0x3F) while in LoRa mode
    /// 0: Access LoRa registers page 0x0D:0x3F
    /// 1: Access FSK registers page(in mode LoRa)0x0D:0x3F
    fn access_shared_reg(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegOpMode).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// SF rate (expressed as a base-2logarithm)
    /// 6 -> 64 chips / symbol
    /// 7 -> 128 chips / symbol
    /// 8 -> 256 chips / symbol
    /// 9 -> 512 chips / symbol
    /// 10 -> 1024 chips / symbol
    /// 11 -> 2048 chips / symbol
    /// 12 -> 4096 chips / symbol
    /// other values reserved.
    fn spreading_factor(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegModemConfig2).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 % (1 << (7 + 1))) as u8) >> 4) << 0;
            Ok(_0)
        }
    }
    ///Payload CRC error interrupt mask: setting this bit masks the corresponding IRQ in RegIrqFlags
    fn payload_crc_error_mask(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegIrqFlagsMask).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    /// See errata note
    fn if_freq_2(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegIfFreq2)
    }
}
pub trait WriteRfm9x {
    type Error;
    /// Write consecutive words to the peripheral. Most chips have it so
    /// that when writing more than just one word, the next word goes into the
    /// next address.
    fn write_contiguous_regs(
        &mut self,
        addr: impl WritableAddr,
        values: &[u8],
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>>;
    /// Write one word to the peripheral. Default calls `write_contiguous_regs` with length `1`.
    fn write_reg(
        &mut self,
        addr: impl WritableAddr,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let words = [value];
            self.write_contiguous_regs(addr, &words).await
        }
    }
    /// LoRa base-band FIFO data input/output. FIFO is cleared an not
    /// accessible when device is in SLEEPmode
    fn set_fifo(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        self.write_reg(RegFifo, value)
    }
    fn set_frf_15_8(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            self.write_reg(RegFrMid, _1).await?;
            Ok(())
        }
    }
    ///read base address in FIFO data buffer for RX demodulator
    fn set_fifo_rx_base_addr(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            self.write_reg(RegFifoRxBaseAddr, _1).await?;
            Ok(())
        }
    }
    /// Symbol periods between frequency hops. (0 = disabled). 1st hop always happen after the 1st header symbol
    fn set_frew_hopping_period(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            self.write_reg(RegHopPeriod, _1).await?;
            Ok(())
        }
    }
    /// Payload length in bytes. The register needs to be set in implicit header mode for the expected packet length. A 0 value is not permitted
    fn set_payload_length(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            self.write_reg(RegPayloadLength, _1).await?;
            Ok(())
        }
    }
    /// Maximum payload length; if header payload length exceeds value a header CRC error is generated. Allows filtering of packet with a bad size.
    fn set_payload_max_length(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            self.write_reg(RegMaxPayloadLength, _1).await?;
            Ok(())
        }
    }
    /// LSB or RF carrier frequency
    fn set_frf_7_0(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            self.write_reg(RegFrLsb, _1).await?;
            Ok(())
        }
    }
    fn set_pa_dac(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        self.write_reg(RegPaDac, value)
    }
    ///write base address in FIFO data buffer for TX modulator
    fn set_fifo_tx_base_addr(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            self.write_reg(RegFifoTxBaseAddr, _1).await?;
            Ok(())
        }
    }
    /// Rise/Fall time of ramp up/down in FSK
    /// 0000 -> 3.4ms
    /// 0001 -> 2ms
    /// 0010 -> 1ms
    /// 0011 -> 500us
    /// 0100 -> 250us
    /// 0101 -> 125us
    /// 0110 -> 100us
    /// 0111 -> 62us
    /// 1000 -> 50us
    /// 1001 -> 40us
    /// 1010 -> 31us
    /// 1011 -> 25us
    /// 1100 -> 20us
    /// 1101 -> 15us
    /// 1110 -> 12us
    /// 1111 -> 10us
    fn set_pa_ramp(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        self.write_reg(RegPaRamp, value)
    }
    ///SPI interface address pointer in FIFO data buffer.
    fn set_fifo_addr_ptr(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            self.write_reg(RegFifoAddrPtr, _1).await?;
            Ok(())
        }
    }
    /// MSB or FR carrier frequency
    fn set_frf_23_16(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            self.write_reg(RegFrMsb, _1).await?;
            Ok(())
        }
    }
    /// Preamble Length LSB
    /// Preamble length MSB, = PreambleLength + 4.25Symbols
    /// See 4.1.1 for more details
    fn set_preamble_length(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        async move {
            let mut _0 = value;
            let _1 = ((_0 % (1 << (7 + 1))) >> 0);
            let _2 = ((_0 % (1 << (15 + 1))) >> 8);
            self.write_reg(RegPreambleLsb, _1).await?;
            self.write_reg(RegPreambleMsb, _2).await?;
            Ok(())
        }
    }
    /// Data rate offset value, used in conjunction with AFC
    fn set_ppm_correction(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        self.write_reg(RegPpmCorrection, value)
    }
    /// See errata note
    fn set_if_freq_2(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        self.write_reg(RegIfFreq2, value)
    }
}
pub enum ErrorCodingRate {
    FourFifths = 0b001,
    FourSixths = 0b010,
    FourSevenths = 0b011,
    FourEighths = 0b100,
}
pub enum SignalBandwidth {
    Cr7_8 = 0b0000,
    Cr10_4 = 0b0001,
    Cr15_6 = 0b0010,
    Cr20_8 = 0b0011,
    Cr31_25 = 0b0100,
    Cr41_7 = 0b0101,
    Cr62_5 = 0b0110,
    Cr125 = 0b0111,
    Cr250 = 0b1000,
    Cr500 = 0b1001,
}
#[automatically_derived]
impl ::core::fmt::Debug for SignalBandwidth {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                SignalBandwidth::Cr7_8 => "Cr7_8",
                SignalBandwidth::Cr10_4 => "Cr10_4",
                SignalBandwidth::Cr15_6 => "Cr15_6",
                SignalBandwidth::Cr20_8 => "Cr20_8",
                SignalBandwidth::Cr31_25 => "Cr31_25",
                SignalBandwidth::Cr41_7 => "Cr41_7",
                SignalBandwidth::Cr62_5 => "Cr62_5",
                SignalBandwidth::Cr125 => "Cr125",
                SignalBandwidth::Cr250 => "Cr250",
                SignalBandwidth::Cr500 => "Cr500",
            },
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for SignalBandwidth {}
#[automatically_derived]
impl ::core::clone::Clone for SignalBandwidth {
    #[inline]
    fn clone(&self) -> SignalBandwidth {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for SignalBandwidth {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SignalBandwidth {}
#[automatically_derived]
impl ::core::cmp::PartialEq for SignalBandwidth {
    #[inline]
    fn eq(&self, other: &SignalBandwidth) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for SignalBandwidth {
    #[inline]
    fn cmp(&self, other: &SignalBandwidth) -> ::core::cmp::Ordering {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for SignalBandwidth {
    #[inline]
    fn partial_cmp(
        &self,
        other: &SignalBandwidth,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
    }
}
#[automatically_derived]
impl ::core::hash::Hash for SignalBandwidth {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state)
    }
}
#[repr(u8)]
pub enum Mode {
    Sleep = 0b000,
    Stdby = 0b001,
    Fstx = 0b010,
    Tx = 0b011,
    Fsrx = 0b100,
    RxContinuous = 0b101,
    RxSingle = 0b110,
    Cad = 0b111,
}
#[automatically_derived]
impl ::core::fmt::Debug for Mode {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Mode::Sleep => "Sleep",
                Mode::Stdby => "Stdby",
                Mode::Fstx => "Fstx",
                Mode::Tx => "Tx",
                Mode::Fsrx => "Fsrx",
                Mode::RxContinuous => "RxContinuous",
                Mode::RxSingle => "RxSingle",
                Mode::Cad => "Cad",
            },
        )
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Mode {}
#[automatically_derived]
impl ::core::clone::Clone for Mode {
    #[inline]
    fn clone(&self) -> Mode {
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Mode {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Mode {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Mode {
    #[inline]
    fn eq(&self, other: &Mode) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Mode {
    #[inline]
    fn cmp(&self, other: &Mode) -> ::core::cmp::Ordering {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Mode {
    #[inline]
    fn partial_cmp(
        &self,
        other: &Mode,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Mode {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state)
    }
}
pub struct Rfm9x<S: SpiHandle> {
    spi: S,
}
pub enum Rfm9xError {
    Spi(SpiError),
    Crc,
    Timeout,
}
#[automatically_derived]
impl ::core::fmt::Debug for Rfm9xError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Rfm9xError::Spi(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Spi", &__self_0)
            }
            Rfm9xError::Crc => ::core::fmt::Formatter::write_str(f, "Crc"),
            Rfm9xError::Timeout => ::core::fmt::Formatter::write_str(f, "Timeout"),
        }
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Rfm9xError {}
#[automatically_derived]
impl ::core::clone::Clone for Rfm9xError {
    #[inline]
    fn clone(&self) -> Rfm9xError {
        let _: ::core::clone::AssertParamIsClone<SpiError>;
        *self
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Rfm9xError {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<SpiError>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Rfm9xError {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Rfm9xError {
    #[inline]
    fn eq(&self, other: &Rfm9xError) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Rfm9xError::Spi(__self_0), Rfm9xError::Spi(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                _ => true,
            }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Rfm9xError {
    #[inline]
    fn cmp(&self, other: &Rfm9xError) -> ::core::cmp::Ordering {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
            ::core::cmp::Ordering::Equal => {
                match (self, other) {
                    (Rfm9xError::Spi(__self_0), Rfm9xError::Spi(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::Ordering::Equal,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Rfm9xError {
    #[inline]
    fn partial_cmp(
        &self,
        other: &Rfm9xError,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (Rfm9xError::Spi(__self_0), Rfm9xError::Spi(__arg1_0)) => {
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
            }
            _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Rfm9xError {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state);
        match self {
            Rfm9xError::Spi(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            _ => {}
        }
    }
}
impl From<SpiError> for Rfm9xError {
    fn from(value: SpiError) -> Self {
        Rfm9xError::Spi(value)
    }
}
type Error = Rfm9xError;
impl<S: SpiHandle> Rfm9x<S> {
    pub fn new(spi: S) -> Self {
        Self { spi }
    }
    pub async fn init(&mut self) -> Result<(), Error> {
        self.set_mode(Mode::Sleep).await?;
        self.set_fifo_tx_base_addr(0).await?;
        self.set_fifo_rx_base_addr(0).await?;
        let lna = self.read_reg(RegLna).await?;
        self.write_reg(RegLna, lna | 0x03).await?;
        self.write_reg(RegModemConfig3, 0x04).await?;
        self.set_mode(Mode::Stdby).await?;
        Ok(())
    }
    pub async fn mode(&mut self) -> Result<Mode, Error> {
        let mode: Mode = unsafe {
            mem::transmute(self.read_reg(RegOpMode).await? & 0b0000_0111)
        };
        Ok(mode)
    }
    pub async fn set_mode(&mut self, mode: Mode) -> Result<(), Error> {
        self.write_reg(RegOpMode, 0b1000_0000 | mode as u8).await?;
        Ok(())
    }
    pub async fn set_ocp(&mut self, current_limit: Milliamps<u8>) -> Result<(), Error> {
        let current_limit = current_limit.value;
        let trim = if current_limit < 45 {
            0
        } else if current_limit < 130 {
            (current_limit - 45) / 5
        } else if current_limit < 240 {
            (current_limit + 30) / 10
        } else {
            27
        };
        self.write_reg(RegOcp, 0b10_0000 + trim).await?;
        Ok(())
    }
    pub async fn use_high_power(&mut self) -> Result<(), Error> {
        self.set_pa_dac(0x87).await?;
        self.set_ocp(140u8.with_units()).await?;
        self.write_reg(RegPaConfig, 0b1_100_1111).await?;
        Ok(())
    }
    pub async fn use_explicit_headers(&mut self) -> Result<(), Error> {
        self.write_reg(RegModemConfig1, 0x72).await?;
        Ok(())
    }
    pub async fn transmit(&mut self, data: &[u8]) -> Result<(), Error> {
        let len: u8 = data.len().try_into().unwrap();
        self.set_mode(Mode::Stdby).await?;
        self.use_explicit_headers().await?;
        self.write_reg(RegIrqFlags, 0).await?;
        self.set_fifo_addr_ptr(0x00).await?;
        self.set_payload_length(0).await?;
        for byte in data {
            self.write_reg(RegFifo, *byte).await?;
        }
        self.write_reg(RegPayloadLength, len).await?;
        match (&(len)) {
            _ => {}
        };
        self.set_mode(Mode::Tx).await?;
        Ok(())
    }
    pub async fn recieve(&mut self, data: &mut [u8]) -> Result<u8, Error> {
        self.set_mode(Mode::RxSingle).await?;
        if self.payload_crc_err().await? {
            self.set_mode(Mode::Stdby).await?;
            return Err(Rfm9xError::Crc);
        }
        let rx_cur_addr: u8 = self.fifo_rx_current_addr().await?;
        self.set_fifo_addr_ptr(rx_cur_addr).await?;
        let len: u8 = self.fifo_rx_nb_bytes().await?;
        self.read_contiguous_regs(RegFifo, data).await?;
        self.set_mode(Mode::Stdby).await?;
        Ok(len)
    }
}
impl<S: SpiHandle> ReadRfm9x for Rfm9x<S> {
    type Error = <S::Bus as ErrorType>::Error;
    async fn read_contiguous_regs(
        &mut self,
        addr: impl ReadableAddr,
        out: &mut [u8],
    ) -> Result<(), Self::Error> {
        let mut bus = self.spi.select().await;
        let addr: u8 = addr.as_addr() & 0b0111_1111;
        bus.write(&[addr]).await?;
        bus.transfer_in_place(out).await?;
        Ok(())
    }
}
impl<S: SpiHandle> WriteRfm9x for Rfm9x<S> {
    type Error = <S::Bus as ErrorType>::Error;
    async fn write_contiguous_regs(
        &mut self,
        addr: impl WritableAddr,
        values: &[u8],
    ) -> Result<(), Self::Error> {
        let mut bus = self.spi.select().await;
        let addr: u8 = addr.as_addr() | 0b1000_0000;
        bus.write(&[addr.as_addr()]).await?;
        bus.write(values).await?;
        Ok(())
    }
}
