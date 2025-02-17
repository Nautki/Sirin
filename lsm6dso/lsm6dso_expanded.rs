#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use core::{fmt::Debug, mem};
use dev_csr::dev_csr;
use embedded_hal::spi::{ErrorKind as SpiError, ErrorType};
use embedded_hal_async::spi::SpiBus;
use spi_handle::SpiHandle;
const _: u8 = 0;
const _: u8 = 0;
pub struct RegFuncCfgAccess;
impl RegFuncCfgAccess {
    pub const ADDR: u8 = 0x01;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegFuncCfgAccess {
    fn as_addr(&self) -> u8 {
        0x01
    }
}
impl ReadableAddr for RegFuncCfgAccess {}
impl WritableAddr for RegFuncCfgAccess {}
pub struct RegPinCtrl;
impl RegPinCtrl {
    pub const ADDR: u8 = 0x02;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegPinCtrl {
    fn as_addr(&self) -> u8 {
        0x02
    }
}
impl ReadableAddr for RegPinCtrl {}
impl WritableAddr for RegPinCtrl {}
pub struct RegFifoCtrl1;
impl RegFifoCtrl1 {
    pub const ADDR: u8 = 0x07;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegFifoCtrl1 {
    fn as_addr(&self) -> u8 {
        0x07
    }
}
impl ReadableAddr for RegFifoCtrl1 {}
impl WritableAddr for RegFifoCtrl1 {}
pub struct RegFifoCtrl2;
impl RegFifoCtrl2 {
    pub const ADDR: u8 = 0x08;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegFifoCtrl2 {
    fn as_addr(&self) -> u8 {
        0x08
    }
}
impl ReadableAddr for RegFifoCtrl2 {}
impl WritableAddr for RegFifoCtrl2 {}
pub struct RegFifoCtrl3;
impl RegFifoCtrl3 {
    pub const ADDR: u8 = 0x09;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegFifoCtrl3 {
    fn as_addr(&self) -> u8 {
        0x09
    }
}
impl ReadableAddr for RegFifoCtrl3 {}
impl WritableAddr for RegFifoCtrl3 {}
pub struct RegFifoCtrl4;
impl RegFifoCtrl4 {
    pub const ADDR: u8 = 0x0A;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegFifoCtrl4 {
    fn as_addr(&self) -> u8 {
        0x0A
    }
}
impl ReadableAddr for RegFifoCtrl4 {}
impl WritableAddr for RegFifoCtrl4 {}
pub struct RegCounterBdrReg1;
impl RegCounterBdrReg1 {
    pub const ADDR: u8 = 0x0B;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCounterBdrReg1 {
    fn as_addr(&self) -> u8 {
        0x0B
    }
}
impl ReadableAddr for RegCounterBdrReg1 {}
impl WritableAddr for RegCounterBdrReg1 {}
pub struct RegCounterBdrReg2;
impl RegCounterBdrReg2 {
    pub const ADDR: u8 = 0x0C;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCounterBdrReg2 {
    fn as_addr(&self) -> u8 {
        0x0C
    }
}
impl ReadableAddr for RegCounterBdrReg2 {}
impl WritableAddr for RegCounterBdrReg2 {}
pub struct RegInt1Ctrl;
impl RegInt1Ctrl {
    pub const ADDR: u8 = 0x0D;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegInt1Ctrl {
    fn as_addr(&self) -> u8 {
        0x0D
    }
}
impl ReadableAddr for RegInt1Ctrl {}
impl WritableAddr for RegInt1Ctrl {}
pub struct RegInt2Ctrl;
impl RegInt2Ctrl {
    pub const ADDR: u8 = 0x0E;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegInt2Ctrl {
    fn as_addr(&self) -> u8 {
        0x0E
    }
}
impl ReadableAddr for RegInt2Ctrl {}
impl WritableAddr for RegInt2Ctrl {}
pub struct RegWhoAmI;
impl RegWhoAmI {
    pub const ADDR: u8 = 0x0F;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegWhoAmI {
    fn as_addr(&self) -> u8 {
        0x0F
    }
}
impl ReadableAddr for RegWhoAmI {}
pub struct RegCtrl1Xl;
impl RegCtrl1Xl {
    pub const ADDR: u8 = 0x10;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl1Xl {
    fn as_addr(&self) -> u8 {
        0x10
    }
}
impl ReadableAddr for RegCtrl1Xl {}
impl WritableAddr for RegCtrl1Xl {}
pub struct RegCtrl2G;
impl RegCtrl2G {
    pub const ADDR: u8 = 0x11;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl2G {
    fn as_addr(&self) -> u8 {
        0x11
    }
}
impl ReadableAddr for RegCtrl2G {}
impl WritableAddr for RegCtrl2G {}
pub struct RegCtrl3C;
impl RegCtrl3C {
    pub const ADDR: u8 = 0x12;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl3C {
    fn as_addr(&self) -> u8 {
        0x12
    }
}
impl ReadableAddr for RegCtrl3C {}
impl WritableAddr for RegCtrl3C {}
pub struct RegCtrl4;
impl RegCtrl4 {
    pub const ADDR: u8 = 0x13;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl4 {
    fn as_addr(&self) -> u8 {
        0x13
    }
}
impl ReadableAddr for RegCtrl4 {}
impl WritableAddr for RegCtrl4 {}
pub struct RegCtrl5;
impl RegCtrl5 {
    pub const ADDR: u8 = 0x14;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl5 {
    fn as_addr(&self) -> u8 {
        0x14
    }
}
impl ReadableAddr for RegCtrl5 {}
impl WritableAddr for RegCtrl5 {}
pub struct RegCtrl6C;
impl RegCtrl6C {
    pub const ADDR: u8 = 0x15;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl6C {
    fn as_addr(&self) -> u8 {
        0x15
    }
}
impl ReadableAddr for RegCtrl6C {}
impl WritableAddr for RegCtrl6C {}
pub struct RegCtrl7G;
impl RegCtrl7G {
    pub const ADDR: u8 = 0x16;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl7G {
    fn as_addr(&self) -> u8 {
        0x16
    }
}
impl ReadableAddr for RegCtrl7G {}
impl WritableAddr for RegCtrl7G {}
pub struct RegCtrl8Xl;
impl RegCtrl8Xl {
    pub const ADDR: u8 = 0x17;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl8Xl {
    fn as_addr(&self) -> u8 {
        0x17
    }
}
impl ReadableAddr for RegCtrl8Xl {}
impl WritableAddr for RegCtrl8Xl {}
pub struct RegCtrl9Xl;
impl RegCtrl9Xl {
    pub const ADDR: u8 = 0x18;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl9Xl {
    fn as_addr(&self) -> u8 {
        0x18
    }
}
impl ReadableAddr for RegCtrl9Xl {}
impl WritableAddr for RegCtrl9Xl {}
pub struct RegCtrl10C;
impl RegCtrl10C {
    pub const ADDR: u8 = 0x19;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegCtrl10C {
    fn as_addr(&self) -> u8 {
        0x19
    }
}
impl ReadableAddr for RegCtrl10C {}
impl WritableAddr for RegCtrl10C {}
pub struct RegAllIntSrc;
impl RegAllIntSrc {
    pub const ADDR: u8 = 0x1A;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegAllIntSrc {
    fn as_addr(&self) -> u8 {
        0x1A
    }
}
impl ReadableAddr for RegAllIntSrc {}
pub struct RegWakeUpSrc;
impl RegWakeUpSrc {
    pub const ADDR: u8 = 0x1B;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegWakeUpSrc {
    fn as_addr(&self) -> u8 {
        0x1B
    }
}
impl ReadableAddr for RegWakeUpSrc {}
pub struct RegTapSrc;
impl RegTapSrc {
    pub const ADDR: u8 = 0x1C;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegTapSrc {
    fn as_addr(&self) -> u8 {
        0x1C
    }
}
impl ReadableAddr for RegTapSrc {}
pub struct RegD6dSrc;
impl RegD6dSrc {
    pub const ADDR: u8 = 0x1D;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegD6dSrc {
    fn as_addr(&self) -> u8 {
        0x1D
    }
}
impl ReadableAddr for RegD6dSrc {}
pub struct RegStatusReg;
impl RegStatusReg {
    pub const ADDR: u8 = 0x1E;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegStatusReg {
    fn as_addr(&self) -> u8 {
        0x1E
    }
}
impl ReadableAddr for RegStatusReg {}
pub struct RegOutTempL;
impl RegOutTempL {
    pub const ADDR: u8 = 0x20;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutTempL {
    fn as_addr(&self) -> u8 {
        0x20
    }
}
impl ReadableAddr for RegOutTempL {}
pub struct RegOutTempH;
impl RegOutTempH {
    pub const ADDR: u8 = 0x21;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutTempH {
    fn as_addr(&self) -> u8 {
        0x21
    }
}
impl ReadableAddr for RegOutTempH {}
pub struct RegOutxLG;
impl RegOutxLG {
    pub const ADDR: u8 = 0x22;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutxLG {
    fn as_addr(&self) -> u8 {
        0x22
    }
}
impl ReadableAddr for RegOutxLG {}
pub struct RegOutxHG;
impl RegOutxHG {
    pub const ADDR: u8 = 0x23;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutxHG {
    fn as_addr(&self) -> u8 {
        0x23
    }
}
impl ReadableAddr for RegOutxHG {}
pub struct RegOutyLG;
impl RegOutyLG {
    pub const ADDR: u8 = 0x24;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutyLG {
    fn as_addr(&self) -> u8 {
        0x24
    }
}
impl ReadableAddr for RegOutyLG {}
pub struct RegOutyHG;
impl RegOutyHG {
    pub const ADDR: u8 = 0x25;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutyHG {
    fn as_addr(&self) -> u8 {
        0x25
    }
}
impl ReadableAddr for RegOutyHG {}
pub struct RegOutzLG;
impl RegOutzLG {
    pub const ADDR: u8 = 0x26;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutzLG {
    fn as_addr(&self) -> u8 {
        0x26
    }
}
impl ReadableAddr for RegOutzLG {}
pub struct RegOutzHG;
impl RegOutzHG {
    pub const ADDR: u8 = 0x27;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutzHG {
    fn as_addr(&self) -> u8 {
        0x27
    }
}
impl ReadableAddr for RegOutzHG {}
pub struct RegOutxLA;
impl RegOutxLA {
    pub const ADDR: u8 = 0x28;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutxLA {
    fn as_addr(&self) -> u8 {
        0x28
    }
}
impl ReadableAddr for RegOutxLA {}
pub struct RegOutxHA;
impl RegOutxHA {
    pub const ADDR: u8 = 0x29;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutxHA {
    fn as_addr(&self) -> u8 {
        0x29
    }
}
impl ReadableAddr for RegOutxHA {}
pub struct RegOutyLA;
impl RegOutyLA {
    pub const ADDR: u8 = 0x2A;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutyLA {
    fn as_addr(&self) -> u8 {
        0x2A
    }
}
impl ReadableAddr for RegOutyLA {}
pub struct RegOutyHA;
impl RegOutyHA {
    pub const ADDR: u8 = 0x2B;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutyHA {
    fn as_addr(&self) -> u8 {
        0x2B
    }
}
impl ReadableAddr for RegOutyHA {}
pub struct RegOutzLA;
impl RegOutzLA {
    pub const ADDR: u8 = 0x2C;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutzLA {
    fn as_addr(&self) -> u8 {
        0x2C
    }
}
impl ReadableAddr for RegOutzLA {}
pub struct RegOutzHA;
impl RegOutzHA {
    pub const ADDR: u8 = 0x2D;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegOutzHA {
    fn as_addr(&self) -> u8 {
        0x2D
    }
}
impl ReadableAddr for RegOutzHA {}
pub struct RegUiOutxLGOisEis;
impl RegUiOutxLGOisEis {
    pub const ADDR: u8 = 0x2E;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutxLGOisEis {
    fn as_addr(&self) -> u8 {
        0x2E
    }
}
impl ReadableAddr for RegUiOutxLGOisEis {}
pub struct RegUiOutxHGOisEis;
impl RegUiOutxHGOisEis {
    pub const ADDR: u8 = 0x2F;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutxHGOisEis {
    fn as_addr(&self) -> u8 {
        0x2F
    }
}
impl ReadableAddr for RegUiOutxHGOisEis {}
pub struct RegUiOutyLGOisEis;
impl RegUiOutyLGOisEis {
    pub const ADDR: u8 = 0x30;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutyLGOisEis {
    fn as_addr(&self) -> u8 {
        0x30
    }
}
impl ReadableAddr for RegUiOutyLGOisEis {}
pub struct RegUiOutyHGOisEis;
impl RegUiOutyHGOisEis {
    pub const ADDR: u8 = 0x31;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutyHGOisEis {
    fn as_addr(&self) -> u8 {
        0x31
    }
}
impl ReadableAddr for RegUiOutyHGOisEis {}
pub struct RegUiOutzLGOisEis;
impl RegUiOutzLGOisEis {
    pub const ADDR: u8 = 0x32;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutzLGOisEis {
    fn as_addr(&self) -> u8 {
        0x32
    }
}
impl ReadableAddr for RegUiOutzLGOisEis {}
pub struct RegUiOutzHGOisEis;
impl RegUiOutzHGOisEis {
    pub const ADDR: u8 = 0x33;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutzHGOisEis {
    fn as_addr(&self) -> u8 {
        0x33
    }
}
impl ReadableAddr for RegUiOutzHGOisEis {}
pub struct RegUiOutxLAOisDualC;
impl RegUiOutxLAOisDualC {
    pub const ADDR: u8 = 0x34;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutxLAOisDualC {
    fn as_addr(&self) -> u8 {
        0x34
    }
}
impl ReadableAddr for RegUiOutxLAOisDualC {}
pub struct RegUiOutxHAOisDualC;
impl RegUiOutxHAOisDualC {
    pub const ADDR: u8 = 0x35;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutxHAOisDualC {
    fn as_addr(&self) -> u8 {
        0x35
    }
}
impl ReadableAddr for RegUiOutxHAOisDualC {}
pub struct RegUiOutyLAOisDualC;
impl RegUiOutyLAOisDualC {
    pub const ADDR: u8 = 0x36;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutyLAOisDualC {
    fn as_addr(&self) -> u8 {
        0x36
    }
}
impl ReadableAddr for RegUiOutyLAOisDualC {}
pub struct RegUiOutyHAOisDualC;
impl RegUiOutyHAOisDualC {
    pub const ADDR: u8 = 0x37;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutyHAOisDualC {
    fn as_addr(&self) -> u8 {
        0x37
    }
}
impl ReadableAddr for RegUiOutyHAOisDualC {}
pub struct RegUiOutzLAOisDualC;
impl RegUiOutzLAOisDualC {
    pub const ADDR: u8 = 0x38;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutzLAOisDualC {
    fn as_addr(&self) -> u8 {
        0x38
    }
}
impl ReadableAddr for RegUiOutzLAOisDualC {}
pub struct RegUiOutzHAOisDualC;
impl RegUiOutzHAOisDualC {
    pub const ADDR: u8 = 0x39;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegUiOutzHAOisDualC {
    fn as_addr(&self) -> u8 {
        0x39
    }
}
impl ReadableAddr for RegUiOutzHAOisDualC {}
pub struct RegTimestamp0;
impl RegTimestamp0 {
    pub const ADDR: u8 = 0x40;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegTimestamp0 {
    fn as_addr(&self) -> u8 {
        0x40
    }
}
impl ReadableAddr for RegTimestamp0 {}
pub struct RegTimestamp1;
impl RegTimestamp1 {
    pub const ADDR: u8 = 0x41;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegTimestamp1 {
    fn as_addr(&self) -> u8 {
        0x41
    }
}
impl ReadableAddr for RegTimestamp1 {}
pub struct RegTimestamp2;
impl RegTimestamp2 {
    pub const ADDR: u8 = 0x42;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegTimestamp2 {
    fn as_addr(&self) -> u8 {
        0x42
    }
}
impl ReadableAddr for RegTimestamp2 {}
pub struct RegTimestamp3;
impl RegTimestamp3 {
    pub const ADDR: u8 = 0x43;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegTimestamp3 {
    fn as_addr(&self) -> u8 {
        0x43
    }
}
impl ReadableAddr for RegTimestamp3 {}
pub struct RegTapCfg0;
impl RegTapCfg0 {
    pub const ADDR: u8 = 0x56;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegTapCfg0 {
    fn as_addr(&self) -> u8 {
        0x56
    }
}
impl ReadableAddr for RegTapCfg0 {}
impl WritableAddr for RegTapCfg0 {}
pub struct RegTapCfg1;
impl RegTapCfg1 {
    pub const ADDR: u8 = 0x57;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegTapCfg1 {
    fn as_addr(&self) -> u8 {
        0x57
    }
}
impl ReadableAddr for RegTapCfg1 {}
impl WritableAddr for RegTapCfg1 {}
pub struct RegTapCfg2;
impl RegTapCfg2 {
    pub const ADDR: u8 = 0x58;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegTapCfg2 {
    fn as_addr(&self) -> u8 {
        0x58
    }
}
impl ReadableAddr for RegTapCfg2 {}
impl WritableAddr for RegTapCfg2 {}
pub struct RegTapThs6d;
impl RegTapThs6d {
    pub const ADDR: u8 = 0x59;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegTapThs6d {
    fn as_addr(&self) -> u8 {
        0x59
    }
}
impl ReadableAddr for RegTapThs6d {}
impl WritableAddr for RegTapThs6d {}
pub struct RegTapDur;
impl RegTapDur {
    pub const ADDR: u8 = 0x5A;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegTapDur {
    fn as_addr(&self) -> u8 {
        0x5A
    }
}
impl ReadableAddr for RegTapDur {}
impl WritableAddr for RegTapDur {}
pub struct RegWakeUpThs;
impl RegWakeUpThs {
    pub const ADDR: u8 = 0x5B;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegWakeUpThs {
    fn as_addr(&self) -> u8 {
        0x5B
    }
}
impl ReadableAddr for RegWakeUpThs {}
impl WritableAddr for RegWakeUpThs {}
pub struct RegWakeUpDur;
impl RegWakeUpDur {
    pub const ADDR: u8 = 0x5C;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegWakeUpDur {
    fn as_addr(&self) -> u8 {
        0x5C
    }
}
impl ReadableAddr for RegWakeUpDur {}
impl WritableAddr for RegWakeUpDur {}
pub struct RegFreeFall;
impl RegFreeFall {
    pub const ADDR: u8 = 0x5D;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegFreeFall {
    fn as_addr(&self) -> u8 {
        0x5D
    }
}
impl ReadableAddr for RegFreeFall {}
impl WritableAddr for RegFreeFall {}
pub struct RegMd1Cfg;
impl RegMd1Cfg {
    pub const ADDR: u8 = 0x5E;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegMd1Cfg {
    fn as_addr(&self) -> u8 {
        0x5E
    }
}
impl ReadableAddr for RegMd1Cfg {}
impl WritableAddr for RegMd1Cfg {}
pub struct RegMd2Cfg;
impl RegMd2Cfg {
    pub const ADDR: u8 = 0x5F;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegMd2Cfg {
    fn as_addr(&self) -> u8 {
        0x5F
    }
}
impl ReadableAddr for RegMd2Cfg {}
impl WritableAddr for RegMd2Cfg {}
pub struct RegI3cBusAvb;
impl RegI3cBusAvb {
    pub const ADDR: u8 = 0x62;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegI3cBusAvb {
    fn as_addr(&self) -> u8 {
        0x62
    }
}
impl ReadableAddr for RegI3cBusAvb {}
impl WritableAddr for RegI3cBusAvb {}
pub struct RegInternalFreqFine;
impl RegInternalFreqFine {
    pub const ADDR: u8 = 0x63;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegInternalFreqFine {
    fn as_addr(&self) -> u8 {
        0x63
    }
}
impl ReadableAddr for RegInternalFreqFine {}
impl WritableAddr for RegInternalFreqFine {}
pub struct RegXOfsUsr;
impl RegXOfsUsr {
    pub const ADDR: u8 = 0x73;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegXOfsUsr {
    fn as_addr(&self) -> u8 {
        0x73
    }
}
impl ReadableAddr for RegXOfsUsr {}
impl WritableAddr for RegXOfsUsr {}
pub struct RegYOfsUsr;
impl RegYOfsUsr {
    pub const ADDR: u8 = 0x74;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegYOfsUsr {
    fn as_addr(&self) -> u8 {
        0x74
    }
}
impl ReadableAddr for RegYOfsUsr {}
impl WritableAddr for RegYOfsUsr {}
pub struct RegZOfsUsr;
impl RegZOfsUsr {
    pub const ADDR: u8 = 0x75;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = true;
}
impl AsLsm6dsoAddr for RegZOfsUsr {
    fn as_addr(&self) -> u8 {
        0x75
    }
}
impl ReadableAddr for RegZOfsUsr {}
impl WritableAddr for RegZOfsUsr {}
pub struct RegFifoDataOutTag;
impl RegFifoDataOutTag {
    pub const ADDR: u8 = 0x78;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegFifoDataOutTag {
    fn as_addr(&self) -> u8 {
        0x78
    }
}
impl ReadableAddr for RegFifoDataOutTag {}
pub struct RegFifoDataOutXL;
impl RegFifoDataOutXL {
    pub const ADDR: u8 = 0x79;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegFifoDataOutXL {
    fn as_addr(&self) -> u8 {
        0x79
    }
}
impl ReadableAddr for RegFifoDataOutXL {}
pub struct RegFifoDataOutXH;
impl RegFifoDataOutXH {
    pub const ADDR: u8 = 0x7A;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegFifoDataOutXH {
    fn as_addr(&self) -> u8 {
        0x7A
    }
}
impl ReadableAddr for RegFifoDataOutXH {}
pub struct RegFifoDataOutYL;
impl RegFifoDataOutYL {
    pub const ADDR: u8 = 0x7B;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegFifoDataOutYL {
    fn as_addr(&self) -> u8 {
        0x7B
    }
}
impl ReadableAddr for RegFifoDataOutYL {}
pub struct RegFifoDataOutYH;
impl RegFifoDataOutYH {
    pub const ADDR: u8 = 0x7C;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegFifoDataOutYH {
    fn as_addr(&self) -> u8 {
        0x7C
    }
}
impl ReadableAddr for RegFifoDataOutYH {}
pub struct RegFifoDataOutZL;
impl RegFifoDataOutZL {
    pub const ADDR: u8 = 0x7D;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegFifoDataOutZL {
    fn as_addr(&self) -> u8 {
        0x7D
    }
}
impl ReadableAddr for RegFifoDataOutZL {}
pub struct RegFifoDataOutZH;
impl RegFifoDataOutZH {
    pub const ADDR: u8 = 0x7E;
    pub const READABLE: bool = true;
    pub const WRITABLE: bool = false;
}
impl AsLsm6dsoAddr for RegFifoDataOutZH {
    fn as_addr(&self) -> u8 {
        0x7E
    }
}
impl ReadableAddr for RegFifoDataOutZH {}
pub trait AsLsm6dsoAddr {
    fn as_addr(&self) -> u8;
}
pub trait ReadableAddr: AsLsm6dsoAddr {}
pub trait WritableAddr: AsLsm6dsoAddr {}
pub trait ReadableValue<T> {
    fn from_value(value: T) -> Self;
}
pub trait WritableValue<T> {
    fn into_value(self) -> T;
}
impl AsLsm6dsoAddr for u8 {
    fn as_addr(&self) -> u8 {
        *self
    }
}
impl ReadableAddr for u8 {}
impl WritableAddr for u8 {}
pub trait ReadLsm6dso {
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
    ///Accel ODR selection.
    ///When XL_HM_MODE=1 in CTRL6_C:
    ///0000: Power-down
    ///1011: 1.6Hz (low power only)
    ///0001: 12.5Hz (low power)
    ///0010: 26Hz (low power)
    ///0011: 52Hz (low power)
    ///0100: 104Hz (normal mode)
    ///0101: 208Hz (normal mode)
    ///0110: 416Hz (high performance)
    ///0111: 833Hz (high performance)
    ///1000: 1.66kHz (high performance)
    ///1001: 3.33kHz (high performance)
    ///1010: 6.66kHz (high performance)
    ///11xx: reserved
    ///Wnen XL_HM_MODE=0 in CTRL6_C:
    ///0000: Power-down
    ///1011: 12.5Hz (high performance)
    ///0001: 12.5Hz (high performance)
    ///0010: 26Hz (high performance)
    ///0011: 52Hz (high performance)
    ///0100: 104Hz (high performance)
    ///0101: 208Hz (high performance)
    ///0110: 416Hz (high performance)
    ///0111: 833Hz (high performance)
    ///1000: 1.66kHz (high performance)
    ///1001: 3.33kHz (high performance)
    ///1010: 6.66kHz (high performance)
    ///11xx: Reserved
    fn accel_odr_mode(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl1Xl).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 4) << 0;
            Ok(_0)
        }
    }
    ///Parity check of TAG content
    fn tag_parity(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegFifoDataOutTag).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///whoami value. Read-only, fixed at 0x6C.
    fn whoami(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegWhoAmI)
    }
    ///Timestamp output. 1LSB = 21.75u
    fn timestamp(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u32, Self::Error>> {
        async move {
            let mut read = [0u8; 4usize];
            self.read_contiguous_regs(RegTimestamp0, &mut read).await?;
            let [_1, _2, _3, _4] = read;
            let mut _0: u32 = 0;
            _0 += (_1 as u32) << 0;
            _0 += (_2 as u32) << 8;
            _0 += (_3 as u32) << 16;
            _0 += (_4 as u32) << 24;
            Ok(_0)
        }
    }
    ///Difference in percentage of the effective ODR (and timestamp rate) with respect to the typical. Step: 0.15%.
    ///8-bit format, two's complement.
    fn freq_fine(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegInternalFreqFine).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Gyro yaw axis angular rate
    fn gyro_yaw_rate(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegOutzLG, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Block data update. Default value: 0
    ///0: continuous update
    ///1: output registers are not updated until MSB and LSB have been read
    fn block_update(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl3C).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Detects change event in activity/inactivity status. Default value: 0
    fn sleep_change_ia(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegAllIntSrc).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Accel x output
    fn accel_x(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegOutxLA, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Wake-up event detection status on Z-axis. Default value: 0
    fn wake_z(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegWakeUpSrc).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Gyroscope UI chain full-scale selection
    ///00: ┬▒250 dp
    ///01: ┬▒500 dp
    ///10: ┬▒1000 dp
    ///11: ┬▒2000 dp
    fn gyro_fs_select(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl2G).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (3 + 1))) as u8) >> 2) << 0;
            Ok(_0)
        }
    }
    ///Enables accel data-ready interrupt on INT2 pin.
    fn int2_accel_rdy(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt2Ctrl).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables gyroscope sleep mode. Default value: 0
    fn gyro_sleep(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl4).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Routing of sensor hub communication concluded event on INT1.
    fn int1_shub(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd1Cfg).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Select batch data rate for accelerometer. Same as gyro.
    fn fifo_accel_bdr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoCtrl3).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (3 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Alerts timestamp overflow within 6.4 ms
    fn timestamp_endct(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegAllIntSrc).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Selection of axis priority for TAP detection (see Table 119)
    ///TAP_PRIORITY_[2:0] | Max. priority | Mid. priority | Min. priority
    ///000 | X | Y | Z
    ///001 | Y | X | Z
    ///010 | X | Z | Y
    ///011 | Z | Y | X
    ///100 | X | Y | Z
    ///101 | Y | Z | X
    ///110 | Z | X | Y
    ///111 | Z | Y | X
    fn tap_priority(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegTapCfg1).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    ///Enable activity/inactivity (sleep) function. Default value: 00
    ///(00: stationary/motion-only interrupts generated, XL and gyro do not change;
    ///01: sets accelerometer ODR to 12.5 Hz (low-power mode), gyro does not change;
    ///10: sets accelerometer ODR to 12.5 Hz (low-power mode), gyro to sleep mode;
    ///11: sets accelerometer ODR to 12.5 Hz (low-power mode), gyro to power-down mode)
    fn inact_en(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegTapCfg2).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (6 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    ///HPF or SLOPE filter selection on wake-up and Activity/Inactivity functions.
    ///Default value: 0 (0: SLOPE filter applied; 1: HPF applied)
    fn slope_fds(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapCfg0).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Y-axis tap recognition threshold. Default value: 0
    ///1 LSB = FS_XL / (2^5)
    fn tap_ths_y(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegTapCfg2).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (4 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Free-fall duration event. Default: 0
    ///For the complete configuration of the free fall duration, refer to FF_DUR5 in
    ///WAKE_UP_DUR (5Ch) configuration
    fn ff_dur(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFreeFall).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 3) << 0;
            Ok(_0)
        }
    }
    ///x axis OIS/EIS
    fn x_ois_eis(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegUiOutxLGOisEis, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Free fall threshold setting. Default: 000
    ///FF_THS[2:0] | Threshold value
    ///000 | 312 mg
    ///001 | 438 mg
    ///010 | 500 mg
    ///011 | Reserved
    ///100 | Reserved
    ///101 | Reserved
    ///110 | Reserved
    ///111 | Reserved
    fn ff_ths(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFreeFall).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (2 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Routing of wakeup event on INT2. Default value: 0
    fn int2_wu(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd2Cfg).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Duration of maximum time gap for double tap recognition. Default: 0000
    ///When double tap recognition is enabled, this register expresses the maximum time
    ///between two consecutive detected taps to determine a double tap event. The default
    ///value of these bits is 0000b which corresponds to 16*ODR_XL time. If the DUR[3:0]
    ///bits are set to a different value, 1LSB corresponds to 32*ODR_XL time.
    fn dur(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegTapDur).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 4) << 0;
            Ok(_0)
        }
    }
    ///Weight of the accelerometer user offset bits of registers X_OFS_USR (73h), Y_OFS_USR (74h), Z_OFS_USR (75h)
    ///0: 2^-10 g/LSB
    ///1: 2^-6 g/LSB
    fn accel_offset_weight(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl6C).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables data available
    ///0: disabled
    ///1: mask DRDY on pin (both accelerometer and gyroscope) until filter settling ends (accelerometer and gyroscope independently masked)
    fn drdy_mask(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl4).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Register address automatically incremented during a multiple byte access with a serial interface (I┬▓C or SPI). Default value: 1
    ///0: disabled
    ///1: enabled
    fn inc_addrs(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl3C).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///4D orientation detection enable. Z-axis position detection is disabled.
    fn d4d_en(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapThs6d).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables temperature data-ready interrupt on INT2 pin.
    fn int2_temp_rdy(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt2Ctrl).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Wake up duration event. Default: 00
    ///1LSB = 1 ODR_time
    fn wake_dur(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegWakeUpDur).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (6 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    ///This bit allows disabling the INT1 pull-down.
    ///PD_DIS_INT1
    ///INT1
    ///(0: Pull-down on INT1 enabled (pull-down is effectively connected only when no interrupts are routed
    ///to the INT1 pin or when the MIPI I3CSM dynamic address is assigned);
    ///1: Pull-down on INT1 disabled (pull-down not connected)
    fn pd_dis_int1(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegI3cBusAvb).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Limit FIFO depth to threshold level
    fn stop_fifo_on_wtm(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegFifoCtrl2).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Whether or not interrupts are active low, default 0
    fn interrupts_active_low(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl3C).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Gyroscope digital HP filter cutoff selection. Default: 00
    ///00: 16 mHz
    ///01: 65 mHz
    ///10: 260 mHz
    ///11: 1.04 Hz
    fn gyro_hpf_cutoff(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl7G).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (5 + 1))) as u8) >> 4) << 0;
            Ok(_0)
        }
    }
    ///z axis OIS/EIS
    fn z_ois_eis(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegUiOutzLGOisEis, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///x axis OIS/DualC
    fn x_ois_dc(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegUiOutxLAOisDualC, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Wake-up event status. Default value: 0
    fn wake_ia(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegAllIntSrc).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///FIFO tag. Identifies sensor used for FIFO data.
    ///Value     Sensor
    ///0x01      Gyroscope NC
    ///0x02      Accelerometer NC
    ///0x03      Temperature
    ///0x04      Timestamp
    ///0x05      CFG_Change
    ///0x06      Accelerometer NC_T_2
    ///0x07      Accelerometer NC_T_1
    ///0x08      Accelerometer 2xC
    ///0x09      Accelerometer 3xC
    ///0x0A      Gyroscope NC_T_2
    ///0x0B      Gyroscope NC_T_1
    ///0x0C      Gyroscope 2xC
    ///0x0D      Gyroscope 3xC
    ///0x0E      Sensor hub slave 0
    ///0x0F      Sensor hub slave 1
    ///0x10      Sensor hub slave 2
    ///0x11      Sensor hub slave 3
    ///0x12      Step counter
    ///0x19      Sensor hub nack
    fn tag_sensor(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoDataOutTag).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 3) << 0;
            Ok(_0)
        }
    }
    ///Interrupt activation level. Default value: 0
    ///0: interrupt output pins active high
    ///1: interrupt output pins active low
    fn pp_od(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl3C).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables gyro data-ready interrupt on INT2 pin.
    fn int2_gyro_rdy(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt2Ctrl).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Tap event detection status on Y-axis. Default value: 0
    fn tap_y(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapSrc).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Accel y output
    fn accel_y(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegOutyLA, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Single/double-tap event enable. Default: 0
    ///(0: only single-tap event enabled;
    ///1: both single and double-tap events enabled)
    fn single_double_tap(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegWakeUpThs).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///z axis OIS/DualC
    fn z_ois_dc(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegUiOutzLAOisDualC, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Routing of 6D event on INT1. Default value: 0
    fn int1_6d(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd1Cfg).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Routing of activity/inactivity recognition event on INT2. Default: 0
    fn int2_sleep_change(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd2Cfg).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///DEN stamping sensor selection. Default value: 0
    ///0: DEN pin info stamped in the gyroscope axis selected by bits [7:5]
    ///1: DEN pin info stamped in the accelerometer axis selected by bits [7:5]
    fn den_stamp(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl9Xl).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables accelerometer LPF2 and HPF fast-settling mode. The filter sets the second samples after writing this bit. Active only during device exit from power- down mode. Default value: 0
    fn accel_fastsettle(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl8Xl).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Routing of embedded functions event on INT1. Default value: 0
    fn int1_emb_func(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd1Cfg).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables accelerometer user offset correction block; it is valid for the low-pass path - see Figure 17. Accelerometer composite filter. Default value: 0
    ///0: accelerometer user offset correction block bypassed
    ///1: accelerometer user offset correction block enabled
    fn accel_offset_enable(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl7G).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Select batch data rate for temperature. 00: not batched. 01: 1.875Hz. 10: 15Hz. 11: 60Hz.
    fn fifo_temp_bdr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoCtrl4).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (5 + 1))) as u8) >> 4) << 0;
            Ok(_0)
        }
    }
    ///Sign of acceleration detected by tap event. Default: 0
    fn tap_sign(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapSrc).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Y-axis low event (under threshold). Default value: 0
    fn low_y(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegD6dSrc).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Threshold for wakeup: 1 LSB weight depends on WAKE_THS_W in WAKE_UP_DUR (5Ch). Default value: 000000
    fn wk_ths(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegWakeUpThs).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (5 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Routing of wakeup event on INT1. Default value: 0
    fn int1_wu(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd1Cfg).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Extends DEN functionality to accelerometer sensor. Default value: 0
    fn den_accel_enable(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl9Xl).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables boot status on the INT1 pin
    fn int1_boot(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt1Ctrl).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///X-axis low event (under threshold). Default value: 0
    fn low_x(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegD6dSrc).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Routing of tap event on INT1. Default value: 0
    fn int1_double_tap(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd1Cfg).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Duration to go in sleep mode. Default value: 0000 (this corresponds to 16 ODR)
    ///1 LSB = 512 ODR
    fn sleep_dur(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegWakeUpDur).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (3 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Enables COUNTER_BDR_IA interrupt on INT2 pin.
    fn int2_cnt_bdr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt2Ctrl).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Drives the low-pass filtered data with user offset correction (instead of high-pass filtered data) to the wakeup function.
    fn usr_off_on_wu(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegWakeUpThs).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///y axis OIS/EIS
    fn y_ois_eis(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegUiOutyLGOisEis, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Maximum duration of overthreshold event. Default value: 00
    ///Maximum duration is the maximum time of an overthreshold signal detection to be
    ///recognized as a tap event. The default value of these bits is 00b which corresponds
    ///to 4*ODR_XL time. If the SHOCK[1:0] bits are set to a different value, 1LSB
    ///corresponds to 8*ODR_XL time.
    fn shock(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegTapDur).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (1 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Configures the rate of written uncompressed data (default 0). 0: do not force uncompressed data writing, 1: uncompressed data every 8 batch, 2: every 16 batch, 3: every 32 batch
    fn fifo_uncompr_rate(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoCtrl2).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (2 + 1))) as u8) >> 1) << 0;
            Ok(_0)
        }
    }
    ///Single-tap event status. Default value: 0
    fn single_tap(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegAllIntSrc).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Z-axis high event (over threshold). Default value: 0
    fn high_z(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegD6dSrc).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Set 1 to reset software, default 0, auto-cleared.
    fn sw_reset(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl3C).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables COUNTER_BDR_IA interrupt on INT1 pin.
    fn int1_cnt_bdr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt1Ctrl).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///accel data available
    fn accel_da(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegStatusReg).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Wake-up event detection status on Y-axis. Default value: 0
    fn wake_y(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegWakeUpSrc).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables FIFO threshold interrupt on INT1 pin.
    fn int1_fifo_thresh(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt1Ctrl).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Tap event detection status on X-axis. Default value: 0
    fn tap_x(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapSrc).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///temp data available
    fn temp_da(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegStatusReg).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Accelerometer high-resolution selection
    ///0: output from first stage digital filtering selected (default)
    ///1: output from LPF2 second filtering stage selected
    fn accel_lpf2(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl1Xl).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Disables high-performance operating mode for gyroscope. Default value: 0
    fn gyro_hp_dis(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl7G).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Routing of tap event on INT2. Default value: 0
    fn int2_double_tap(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd2Cfg).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Tap event detection status on Z-axis. Default value: 0
    fn tap_z(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapSrc).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Select decimation for timestamp batching. 00: timestamp not batched. 01: write rate = max(fifo_accel_bdr, fifo_gyro_bdr). 10: write rate = max(fifo_accel_bdr, fifo_gyro_bdr)/8.  11: write rate = max(fifo_accel_bdr, fifo_gyro_bdr)/32.
    fn fifo_ts_decim(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoCtrl4).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 6) << 0;
            Ok(_0)
        }
    }
    ///Gyro roll axis angular rate
    fn gyro_roll_rate(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegOutyLG, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Reboots memory content. Default 0, auto-cleared.
    fn reboot_mem(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl3C).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables FIFO full flag interrupt on INT1 pin.
    fn int1_fifo_full(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt1Ctrl).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables FIFO overrun interrupt on INT1 pin.
    fn int1_fifo_over(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt1Ctrl).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables FIFO full interrupt on INT2 pin.
    fn int2_fifo_full(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt2Ctrl).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///X-axis high event (over threshold). Default value: 0
    fn high_x(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegD6dSrc).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Y-axis high event (over threshold). Default value: 0
    fn high_y(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegD6dSrc).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///2-bit counter which identifies sensor time slot
    fn tag_counter(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoDataOutTag).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (2 + 1))) as u8) >> 1) << 0;
            Ok(_0)
        }
    }
    ///Trigger mode selection for DEN
    ///100: Edge-sensitive trigger mode is selected.
    ///010: Level-sensitive trigger mode is selected.
    ///011: Level-sensitive latched mode is selected.
    ///110: Level-sensitive FIFO enable mode is selected.
    fn den_trigger_mode(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl6C).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    ///Routing of embedded functions event on INT2. Default value: 0
    fn int2_emb_func(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd2Cfg).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables routing on INT2 pin of the alert for timestamp overflow within 6.4 ms
    fn int2_timestamp(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd2Cfg).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Double-tap event status. Default value: 0
    fn double_tap(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegAllIntSrc).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Accel output data rate selection.
    ///When gyro_hm_mode=1:
    ///0000: power down (default)
    ///0001: 12.5Hz (low power)
    ///0010: 26Hz (low power)
    ///0011: 52Hz (low power)
    ///0100: 104Hz (normal)
    ///0101: 208Hz (normal)
    ///0110: 416Hz (high performance)
    ///0111: 833Hz (high performance)
    ///1000: 1.66kHz (high performance)
    ///1001: 3.33kHz (high performance)
    ///1010: 6.66kHz (high performance)
    ///others: reserved
    ///When gyro_hm_mode=0:
    ///0000: power down (high performance)
    ///0001: 12.5Hz (high performance)
    ///0010: 26Hz (high performance)
    ///0011: 52Hz (high performance)
    ///0100: 104Hz (high performance)
    ///0101: 208Hz (high performance)
    ///0110: 416Hz (high performance)
    ///0111: 833Hz (high performance)
    ///1000: 1.66kHz (high performance)
    ///1001: 3.33kHz (high performance)
    ///1010: 6.66kHz (high performance)
    ///others: reserved
    fn gyro_odr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl2G).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 4) << 0;
            Ok(_0)
        }
    }
    ///Sets the threshold for the internal
    ///counter of batch events. When this counter reaches the
    ///threshold, the counter is reset and counter_bdr_reached is
    ///set to 1.
    fn batch_counter_thresh(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegCounterBdrReg1, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (((_1 & (1 << (2 + 1))) as u16) >> 0) << 0;
            _0 += (_2 as u16) << 3;
            Ok(_0)
        }
    }
    ///Enables gyroscope digital LPF1 if the auxiliary SPI is disabled; the bandwidth can be selected through FTYPE[2:0] in CTRL6_C (15h).
    fn gyro_lpf1(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl4).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables FIFO threshold interrupt on INT2 pin.
    fn int2_fifo_thresh(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt2Ctrl).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///DEN value stored in LSB of Z-axis. Default value: 1
    fn den_z(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl9Xl).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Disables high-performance operating mode for the accelerometer. Default value: 0
    fn accel_hp_dis(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl6C).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables angular rate sensor self-test. Default value: 00
    ///00: self-test disabled
    ///01: Positive sign self-test
    ///10: Reserved
    ///11: Negative sign self-test
    fn gyro_selftest(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl5).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (3 + 1))) as u8) >> 2) << 0;
            Ok(_0)
        }
    }
    ///Enables accel data-ready interrupt on INT1 pin.
    fn int1_accel_rdy(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt1Ctrl).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Sleep status bit. Default value: 0
    fn sleep_state(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegWakeUpSrc).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Push-pull/open-drain selection on the INT1 and INT2 pins. Default value: 0
    ///0: push-pull mode
    ///1: open-drain mode
    fn spi_mode_select(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl3C).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Disables I┬▓C interface. Default value:
    ///0: SPI, I┬▓C, and MIPI I3CSM interfaces enabled(default)
    ///1: I┬▓C interface disable
    fn disable_i2c(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl4).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Accelerometer LPF2 and HP filter configuration and cutoff setting.
    ///Low Pass, accel_lpf2=0: ODR/2
    ///Low Pass, accel_lpf2=1:
    ///000: ODR/4
    ///001: ODR/10
    ///010: ODR/20
    ///011: ODR/45
    ///100: ODR/100
    ///101: ODR/200
    ///110: ODR/400
    ///111: ODR/800
    ///High Pass:
    ///000: ODR/4
    ///001: ODR/10
    ///010: ODR/20
    ///011: ODR/45
    ///100: ODR/100
    ///101: ODR/200
    ///110: ODR/400
    ///111: ODR/800
    fn accel_hpf_cutoff(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl8Xl).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    ///Enables FIFO overrun interrupt on INT2 pin.
    fn int2_fifo_over(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt2Ctrl).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Accelerometer slope filter / high-pass filter selection
    fn accel_slope_hp(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl8Xl).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///DEN active level configuration. Default value: 0
    fn den_high(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl9Xl).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables accelerometer high-pass filter reference mode (valid for high-pass path - HP_SLOPE_XL_EN bit must be 1). Default value: 0
    fn accel_hp_refmode(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl8Xl).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Wake-up event detection status on X-axis. Default value: 0
    fn wake_x(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegWakeUpSrc).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Activity/inactivity interrupt mode configuration.
    ///If INT1_SLEEP_CHANGE or INT2_SLEEP_CHANGE bits are enabled, drives
    ///the sleep status or sleep change on the INT pins. Default value: 0
    ///(0: sleep change notification on INT pins; 1: sleep status reported on INT pins)
    fn sleep_status_on_int(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapCfg0).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables accelerometer ultralow-power mode. Default value: 0
    fn accel_ulp(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl5).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enable Y direction in tap recognition. Default value: 0
    fn tap_y_en(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapCfg0).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Routing of free-fall event on INT1. Default value: 0
    fn int1_ff(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd1Cfg).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Batch ODR nCHANGE sensor in FIFO
    fn fifo_batch_odrchg(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegFifoCtrl2).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Select batch data rate for gyro data. 0000: gyro not batched, 0001: 1.875Hz, otherwise rate = 1.875*2^{input}, max input 1100
    fn fifo_gyro_bdr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoCtrl3).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (7 + 1))) as u8) >> 4) << 0;
            Ok(_0)
        }
    }
    ///Enables pulsed data-ready mode.
    ///0: data-ready latched mode (returns to 0 only after an interface reading) (default)
    ///1: data-ready pulsed mode (the data ready pulses are 75 ∩┐╜s long)
    fn pulsed_drdy(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCounterBdrReg1).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables all interrupt signals available on the INT1 pin. Default value: 0
    ///0: interrupt signals divided between the INT1 and INT2 pins
    ///1: all interrupt signals in logic or on the INT1 pin
    fn int2_on_int1(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl4).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables/disables the OIS chain from the primary interface when ois_on_primary is 1.
    fn ois_enable(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl7G).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Routing of single-tap recognition event on INT1. Default: 0
    fn int1_single_tap(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd1Cfg).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///These bits are used to select the bus available time when MIPI I3CSM IBI is used.
    ///Default value: 00
    ///(00: bus available time equal to 50 ┬╡sec (default);
    ///01: bus available time equal to 2 ┬╡sec;
    ///10: bus available time equal to 1 msec;
    ///11: bus available time equal to 25 msec
    fn i3c_bus_avb_sel(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegI3cBusAvb).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (4 + 1))) as u8) >> 3) << 0;
            Ok(_0)
        }
    }
    ///Accel z output
    fn accel_z(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegOutzLA, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    fn usr_offset_z(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegZOfsUsr)
    }
    ///Enables gyroscope digital high-pass filter. The filter is enabled only if the gyroscope is in HP mode. Default value: 0
    fn gyro_hpf_enable(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl7G).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables gyro data-ready interrupt on INT1 pin.
    fn int1_gyro_rdy(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt1Ctrl).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enable compression
    fn fifo_compr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegFifoCtrl2).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enable FSM-triggered batching of channel 2 when available, 0 disabled, 1 enabled, default 0
    ///FIFO watermark threshold, 1 LSB = TAG (1 byte) + 1 sensor (6 bytes) in FIFO, flag rises when #bytes in FIFO > threshold
    fn fifo_wtm(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoCtrl2).await?;
            let _2 = self.read_reg(RegFifoCtrl1).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (0 + 1))) as u8) >> 0) << 0;
            _0 += (_2 as u8) << 1;
            Ok(_0)
        }
    }
    ///X-axis tap recognition threshold. Default value: 0
    ///1 LSB = FS_XL / (2^5)
    fn tap_ths_x(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegTapCfg1).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (4 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Weight of 1 LSB of wakeup threshold. Default: 0
    ///(0: 1 LSB = FS_XL / (26);
    ///1: 1 LSB = FS_XL / (28) )
    fn wake_ths_w(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegWakeUpDur).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Single-tap recognition routing on INT2. Default: 0
    fn int2_single_tap(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd2Cfg).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///This bit allows immediately clearing the latched interrupts of an event detection
    ///upon the read of the corresponding status register. It must be set to 1 together
    ///with LIR. Default value: 0
    ///(0: latched interrupt signal cleared at the end of the ODR period;
    ///1: latched interrupt signal immediately cleared)
    fn int_clr_on_read(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapCfg0).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///sensor hub config reg access, default 0
    fn shub_reg(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegFuncCfgAccess).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Select the trigger for the internal counter of batch events for the accel, gyro, and EIS gyro.
    ///00: accel batch event.
    ///01: gyro batch event.
    ///10-11: gyro EIS batch event.
    fn trig_ctr_bdr(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCounterBdrReg1).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (6 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    ///Gyro pitch axis angular rate
    fn gyro_pitch_rate(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegOutxLG, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Routing of activity/inactivity recognition event on INT1. Default: 0
    fn int1_sleep_change(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd1Cfg).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enable basic interrupts (6D/4D, free-fall, wake-up, tap, inactivity). Default value: 0
    fn interrupts_enable(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapCfg2).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///y axis OIS/DualC
    fn y_ois_dc(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegUiOutyLAOisDualC, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Disables MIPI I3CSM communication protocol, default 0
    fn i3c_disable(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl9Xl).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///DEN data-ready signal. It is set high when the data output is related to the data coming from a DEN active condition.
    fn den_drdy(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegD6dSrc).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Z-axis low event (under threshold). Default value: 0
    fn low_z(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegD6dSrc).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enable Z direction in tap recognition. Default value: 0
    fn tap_z_en(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapCfg0).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Temp data output register
    fn temp_data(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u16, Self::Error>> {
        async move {
            let mut read = [0u8; 2usize];
            self.read_contiguous_regs(RegOutTempL, &mut read).await?;
            let [_1, _2] = read;
            let mut _0: u16 = 0;
            _0 += (_1 as u16) << 0;
            _0 += (_2 as u16) << 8;
            Ok(_0)
        }
    }
    ///Selects how to enable and disable the OIS chain, after first configuration and enabling through SPI2.
    ///0: OIS chain is enabled/disabled with SPI2 interface
    ///1: OIS chain is enabled/disabled with primary interface
    fn ois_on_primary(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl7G).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Sends DEN_DRDY (DEN stamped on the sensor data flag) to the INT1 pin
    fn den_drdy_flag(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegInt1Ctrl).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Free-fall event status. Default value: 0
    fn freefall_ia(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegAllIntSrc).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Enables timestamp counter. Default value: 0
    fn timestamp_enable(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl10C).await?;
            let word = (word >> 5) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Expected quiet time after a tap detection. Default value: 00
    ///Quiet time is the time after the first detected tap in which there must not be any
    ///overthreshold event. The default value of these bits is 00b which corresponds to
    ///2*ODR_XL time. If the QUIET[1:0] bits are set to a different value, 1LSB corresponds
    ///to 4*ODR_XL time.
    fn quiet(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegTapDur).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (3 + 1))) as u8) >> 2) << 0;
            Ok(_0)
        }
    }
    fn usr_offset_y(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegYOfsUsr)
    }
    ///Accelerometer full-scale management between UI chain and OIS chain
    ///0: old full-scale mode. When XL UI is on, the full scale is the same between UI/OIS and is chosen by the UI CTRL registers; when XL UI is in PD, the OIS can choose the FS.
    ///1: new full-scale mode. Full scales are independent between the UI/OIS chain but both bound to ┬▒8g
    fn accel_new_fs(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl8Xl).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Threshold for 4D/6D function. Default value: 00
    ///SIXD_THS | Threshold value
    ///00 | 68 degrees
    ///01 | 47 degrees
    ///10 | Reserved
    ///11 | Reserved
    fn sixd_ths(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegTapThs6d).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (6 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    ///1 disables pull-up on OCS_Aux and SDO_Aux pins, 0 enables, default 0
    fn ois_pullup_L(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegPinCtrl).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Mode selection.
    ///000: FIFO disabled.
    ///001: FIFO mode: stop collection when FIFO is full.
    ///010: continuousWTM-to-full mode: cont. mode with wtm until
    ///trigger is deasserted, then FIFO mode,
    ///011: continuous-to-FIFO mode: continuous until trigger is
    ///deasserted, then FIFO mode.
    ///100: bypass-to-continuous mode: FIFO disabled until trigger
    ///is deasserted, then continuous mode.
    ///101: reserved.
    ///110: continuous mode: if FIFO is full, new samples overwrite
    ///old ones.
    ///111: bypass-to-FIFO mode: FIFO disabled until trigger is
    ///deasserted, then FIFO mode.
    fn fifo_mode(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegFifoCtrl4).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (2 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Enable X direction in tap recognition. Default value: 0
    fn tap_x_en(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapCfg0).await?;
            let word = (word >> 3) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Tap event detection status. Default: 0
    fn tap_ia(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapSrc).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Latched Interrupt. Default value: 0
    fn lir(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegTapCfg0).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///DEN value stored in LSB of Y-axis. Default value: 1
    fn den_y(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl9Xl).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///gyro data available
    fn gyro_da(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegStatusReg).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Free fall duration event. Default: 0
    ///For the complete configuration of the free-fall duration, refer to FF_DUR[4:0] in
    ///FREE_FALL (5Dh) configuration.
    ///1 LSB = 1 ODR_time
    fn ff_dur5(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegWakeUpDur).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///LPF2 on 6D function selection.
    ///0: ODR/2 low-pass filtered data sent to 6D interrupt function
    ///1: LPF2 output data sent to 6D interrupt function
    fn lpf2_on_6d(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl8Xl).await?;
            let word = (word >> 0) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Interrupt active for change in position of portrait, landscape, face-up, face-down. Default value: 0
    fn orientation_ia(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegAllIntSrc).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Circular burst mode (wraparound) read from the output registers. Default value: 00
    ///00: no wraparound
    ///01: accelerometer only
    ///10: gyroscope only
    ///11: gyroscope + accelerometer
    fn read_wraparound(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl5).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (6 + 1))) as u8) >> 5) << 0;
            Ok(_0)
        }
    }
    ///Enables linear acceleration sensor self-test. Default value: 00
    ///00: self-test disabled
    ///01: Positive sign self-test
    ///10: Negative sign self-test
    ///11: Reserved
    fn accel_selftest(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl5).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (1 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///embedded functions config register access, default 0
    fn emb_func(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegFuncCfgAccess).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Gyroscope low-pass filter (LPF1) bandwidth selection.
    ///Table didn't fit, see datasheet
    fn gyro_lpf_bw(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl6C).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (2 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Enables pull-up on SDO pin, default 0
    fn sdo_pullup(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegPinCtrl).await?;
            let word = (word >> 6) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Routing of 6D event on INT2. Default value: 0
    fn int2_6d(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd2Cfg).await?;
            let word = (word >> 2) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Accelerometer full-scale selection
    ///When XL_FS_MODE=0 in CTRL8_XL
    ///00: ┬▒4g (default)
    ///01: ┬▒32
    ///10: ┬▒8
    ///11: ┬▒16
    fn accel_fs(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegCtrl1Xl).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (3 + 1))) as u8) >> 2) << 0;
            Ok(_0)
        }
    }
    ///Z-axis recognition threshold. Default value: 0
    fn tap_ths_z(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        async move {
            let _1 = self.read_reg(RegTapThs6d).await?;
            let mut _0: u8 = 0;
            _0 += (((_1 & (1 << (4 + 1))) as u8) >> 0) << 0;
            Ok(_0)
        }
    }
    ///Selects gyro UI chain full-scale ┬▒125 dp
    ///0: FS selected through gyro_fs_select
    ///1: FS set to ┬▒125 dp
    fn gyro_125dps(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl2G).await?;
            let word = (word >> 1) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///Routing of free-fall event on INT2. Default value: 0
    fn int2_ff(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegMd2Cfg).await?;
            let word = (word >> 4) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    ///DEN value stored in LSB of X-axis. Default value: 1
    fn den_x(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<bool, Self::Error>> {
        async move {
            let word = self.read_reg(RegCtrl9Xl).await?;
            let word = (word >> 7) % 1;
            unsafe { Ok(core::mem::transmute(word)) }
        }
    }
    fn usr_offset_x(
        &mut self,
    ) -> impl core::future::Future<Output = core::result::Result<u8, Self::Error>> {
        self.read_reg(RegXOfsUsr)
    }
}
pub trait WriteLsm6dso {
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
    fn set_usr_offset_z(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        self.write_reg(RegZOfsUsr, value)
    }
    fn set_usr_offset_y(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        self.write_reg(RegYOfsUsr, value)
    }
    fn set_usr_offset_x(
        &mut self,
        value: u8,
    ) -> impl core::future::Future<Output = core::result::Result<(), Self::Error>> {
        self.write_reg(RegXOfsUsr, value)
    }
}
pub struct Lsm6dso<S: SpiHandle> {
    spi: S,
}
impl<S: SpiHandle> Lsm6dso<S> {
    pub fn new(spi: S) -> Self {
        Self { spi }
    }
    pub async fn setup(&mut self) -> Result<(), <S::Bus as ErrorType>::Error> {
        self.write_reg(RegCtrl1Xl, 0b1010_11_00 as u8).await?;
        self.write_reg(RegCtrl2G, 0b1010_11_00).await?;
        Ok(())
    }
    pub async fn raw_accel(
        &mut self,
    ) -> Result<(i16, i16, i16), <S::Bus as ErrorType>::Error> {
        Ok(unsafe {
            let accel_x: i16 = mem::transmute(self.accel_x().await?);
            let accel_y: i16 = mem::transmute(self.accel_y().await?);
            let accel_z: i16 = mem::transmute(self.accel_z().await?);
            (accel_x, accel_y, accel_z)
        })
    }
    pub async fn raw_gyro(
        &mut self,
    ) -> Result<(i16, i16, i16), <S::Bus as ErrorType>::Error> {
        Ok(unsafe {
            let gyro_pitch: i16 = mem::transmute(self.gyro_pitch_rate().await?);
            let gyro_roll: i16 = mem::transmute(self.gyro_roll_rate().await?);
            let gyro_yaw: i16 = mem::transmute(self.gyro_yaw_rate().await?);
            (gyro_pitch, gyro_roll, gyro_yaw)
        })
    }
    pub async fn accel_sensitivity(
        &mut self,
    ) -> Result<i32, <S::Bus as ErrorType>::Error> {
        Ok(
            match self.accel_fs().await? {
                0 => 4,
                1 => 32,
                2 => 8,
                3 => 16,
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )
    }
    /// 0 = 4g, 1 = 8g, 2 = 16g, 3 = 32g
    pub async fn set_accel_sensitivity(
        &mut self,
        new_fs: u8,
    ) -> Result<u8, <S::Bus as ErrorType>::Error> {
        let accel_mode = self.read_reg(RegCtrl1Xl).await?;
        let mask = 0b1111_00_11;
        let new_bits = match new_fs {
            0 => 0b0000_00_00,
            1 => 0b0000_10_00,
            2 => 0b0000_11_00,
            3 => 0b0000_01_00,
            _ => 0b0000_00_00,
        };
        self.write_reg(RegCtrl1Xl, accel_mode & mask | new_bits as u8).await?;
        Ok(new_bits >> 2)
    }
    pub async fn gyro_sensitivity(
        &mut self,
    ) -> Result<i32, <S::Bus as ErrorType>::Error> {
        Ok(
            match self.gyro_fs_select().await? {
                0 => 250,
                1 => 500,
                2 => 1000,
                3 => 2000,
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        )
    }
    pub async fn test_fs(&mut self) -> Result<u8, <S::Bus as ErrorType>::Error> {
        Ok(self.accel_fs().await?)
    }
    /// returns a tuple with units of ug (10^-6)
    pub async fn accel(
        &mut self,
    ) -> Result<(i32, i32, i32), <S::Bus as ErrorType>::Error> {
        let (raw_x, raw_y, raw_z) = self.raw_accel().await?;
        let fs = self.accel_sensitivity().await?;
        let scalar: i32 = 122 * fs;
        let accel_x: i32 = scalar * (raw_x as i32);
        let accel_y: i32 = scalar * (raw_y as i32);
        let accel_z: i32 = scalar * (raw_z as i32);
        Ok((accel_x, accel_y, accel_z))
    }
    pub async fn gyro(
        &mut self,
    ) -> Result<(i32, i32, i32), <S::Bus as ErrorType>::Error> {
        let (raw_pitch, raw_roll, raw_yaw) = self.raw_gyro().await?;
        let fs = self.gyro_sensitivity().await?;
        let scalar: i32 = 4375 * fs / 125;
        let gyro_pitch: i32 = scalar * (raw_pitch as i32);
        let gyro_roll: i32 = scalar * (raw_roll as i32);
        let gyro_yaw: i32 = scalar * (raw_yaw as i32);
        Ok((gyro_pitch, gyro_roll, gyro_yaw))
    }
}
impl<S: SpiHandle> ReadLsm6dso for Lsm6dso<S> {
    type Error = <S::Bus as ErrorType>::Error;
    async fn read_contiguous_regs(
        &mut self,
        addr: impl ReadableAddr,
        out: &mut [u8],
    ) -> Result<(), Self::Error> {
        let mut bus = self.spi.select().await;
        let addr: u8 = addr.as_addr() | 0b1000_0000;
        bus.write(&[addr]).await?;
        bus.transfer_in_place(out).await?;
        Ok(())
    }
}
impl<S: SpiHandle> WriteLsm6dso for Lsm6dso<S> {
    type Error = <S::Bus as ErrorType>::Error;
    async fn write_contiguous_regs(
        &mut self,
        addr: impl WritableAddr,
        values: &[u8],
    ) -> Result<(), Self::Error> {
        let mut bus = self.spi.select().await;
        let addr: u8 = addr.as_addr() & 0b0111_1111;
        bus.write(&[addr.as_addr()]).await?;
        bus.write(values).await?;
        Ok(())
    }
}
