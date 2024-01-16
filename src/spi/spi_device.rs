use core::future;

use embedded_hal_async::spi::SpiBus;

use super::{spi::{Spi, SpiInstance}, config::SpiConfig};
//use async_trait::async_trait;



pub trait SpiDevice {
    fn select(&self);
    fn deselect(&self);
}

macro_rules! impl_SpiDevice {
    ($ident: ) => {
        
    };
}

/// Only valid INSIDE an SpiDevice implementation
#[macro_export]
macro_rules! spi_transact {
    ($($tx: tt)*) => {{
        let slf = unhygienic!(self);
        let val = {
            let inst = slf.spi.borrow().await;
            slf.select();
            $($tx)*
        }
        slf.deselect();
        val   
    }};
}