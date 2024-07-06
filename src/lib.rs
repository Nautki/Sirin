#![no_std]

use core::future::Future;

pub use periphs_macro::{csr, RegValue};

pub trait Register<A, V> {
    const ADDR: A;
    const READABLE: bool;
    const WRITABLE: bool;
    type Value: Into<RegValue<V>> + From<RegValue<V>>;
}

pub trait Readable {}
pub trait Writable {}

pub struct RegValue<V>(pub V);

pub trait WriteReg<A = u8, V = u8> {
    type Error;

    fn write_addr(&mut self, addr: A, value: V) -> impl Future<Output = Result<(), Self::Error>>;

    fn write_reg<R: Register<A, V>>(&mut self, value: R::Value) -> impl Future<Output = Result<(), Self::Error>> {
        let value: RegValue<V> = value.into();
        self.write_addr(R::ADDR, value.0)
    }
}

pub trait ReadReg<A = u8, V = u8> {
    type Error;

    fn read_addr(&mut self, addr: A) -> impl Future<Output = Result<V, Self::Error>>;

    fn read_reg<R: Register<A, V>>(&mut self) -> impl Future<Output = Result<R::Value, Self::Error>> {
        async {
            match self.read_addr(R::ADDR).await {
                Ok(val) => Ok(RegValue(val).into()),
                Err(err) => Err(err)
            }
        }
    }
}

macro_rules! impl_prims {
    ($($prim:ty),*) => {
        $(
            impl From<$prim> for RegValue<$prim> {
                fn from(value: $prim) -> Self {
                    RegValue(value)
                }
            }
            impl From<RegValue<$prim>> for $prim {
                fn from(value: RegValue<$prim>) -> Self {
                    value.0
                }
            }
        )*
    };
}

impl_prims!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);