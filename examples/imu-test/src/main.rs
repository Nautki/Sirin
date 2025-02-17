#![no_std]
#![no_main]

use core::mem::{self, MaybeUninit};

use bmp3::{hal::{Bmp3RawData, ReadBmp3, RegErrReg, RegStatus}, Bmp3Readout};
use defmt::*;
use embassy_executor::{task, Executor, Spawner};
use embassy_stm32::{bind_interrupts, dma::NoDma, gpio::{Level, Output, Speed}, peripherals::{self, DMA1_CH0, DMA1_CH1, PD8, PD9, USART3}, usart::{self, Config, Uart}};
use embassy_time::Timer;
use rfm9x::ReadRfm9x;
use lsm6dso::ReadLsm6dso;
use {defmt_rtt as _, panic_probe as _};
use sirin::Sirin;

unsafe fn transmute_into_static<T>(item: &mut T) -> &'static mut T {
    core::mem::transmute(item)
}

#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    let mut executor = Executor::new();
    let executor: &'static mut Executor = transmute_into_static(&mut executor);
    let mut sirin = MaybeUninit::<Sirin>::uninit();
    let sirin = transmute_into_static(&mut sirin);
    executor.run(|spawner| {
        spawner.must_spawn(setup_task(spawner, sirin))
    })
}

#[task()]
async fn setup_task(spawner: Spawner, sirin: &'static mut MaybeUninit<Sirin>) {
    debug!("Begin Sirin init");

    let sirin = Sirin::init(sirin, spawner).await;

    debug!("End Sirin init");

    main_task(sirin).await
}

bind_interrupts!(struct Irqs {
    USART3 => usart::InterruptHandler<peripherals::USART3>;
});

async fn main_task(sirin: &'static mut Sirin) {
    sirin.imu.setup().await.unwrap();
    println!("set sensitivity: {}", sirin.imu.set_accel_sensitivity(2).await.unwrap());
    println!("read ctrl: {}", sirin.imu.read_reg(0x10).await.unwrap());
    
    println!("read real sensitivity: {}", sirin.imu.accel_sensitivity().await.unwrap());
    println!("read bits: {}", sirin.imu.test_fs().await.unwrap());
    let this = (((0b0011_00_00 & (1 << (3 + 1))) as u8) >>2) << 0;
    println!("{}", this);

    for i in 0..3{
        println!("set sensitivity: {}", sirin.imu.set_accel_sensitivity(0).await.unwrap());
        println!("raw accel: {} \nadjusted accel: {}", sirin.imu.raw_accel().await.unwrap(), sirin.imu.accel().await.unwrap());
    }
    loop {
        // println!("raw accel: {} \nadjusted accel: {}", sirin.imu.raw_accel().await.unwrap(), sirin.imu.accel().await.unwrap());
        // println!("gyro: {}", sirin.imu.raw_gyro().await.unwrap());
    }
}