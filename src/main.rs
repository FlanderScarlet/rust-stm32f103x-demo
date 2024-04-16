#![no_std]
#![no_main]

mod rtt_logger;

use cortex_m as _;
use panic_rtt_target as _; 
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::IOPinSpeed;
use stm32f1xx_hal::gpio::OutputSpeed;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;

use log::{info,warn,error,debug};

#[entry]
fn main() -> ! {
    rtt_logger::init(log::LevelFilter::Debug);
    // 获取stm32外设
    let bp = pac::Peripherals::take().unwrap();
    // 获取 porta 
    let mut gpioa = bp.GPIOA.split();
    let mut led1 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let mut led2 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    led1.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);
    led1.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);
    led1.set_high();
    led2.set_high();

    let mut cnt = 0;
    loop {
       match cnt {
           0 => info!("Hello, world!"),
           1 => warn!("Hello, world!"),
           2 => error!("Hello, world!"),
           1000 => panic!("Hello, world!"),
           _ => debug!("Hello, world!"),
       }
       cnt += 1;
    }
}
