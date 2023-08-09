#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
//use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    /* Get access to device and core peripherals */
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    /* Get access to RCC, and GPIOA */
    let _ = dp.RCC.apb2enr.write(|w| w.iopaen().set_bit());
    let rcc = dp.RCC.constrain();
  
    let mut flash = dp.FLASH.constrain();
    let mut gpioa = dp.GPIOA.split();

    /* Set up LED pin */
    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);

    /* Set up sysclk and freeze it */
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    /* Set up systick delay */
   let mut delay = cp.SYST.delay(&clocks);

    loop {
        /* Light show */
        led.set_low();
        delay.delay_ms(5_000_u16);
        led.set_high();
        delay.delay_ms(1000_u16);
    }
}