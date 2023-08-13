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
    let _ = dp.RCC.apb2enr.write(|w| w.iopben().set_bit());
    let rcc = dp.RCC.constrain();
  
    let mut flash = dp.FLASH.constrain();
    let mut gpiob = dp.GPIOB.split();

    /* Set up LED pin */
    let mut buzzer = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let light_sensor = gpiob.pb13.into_pull_up_input(&mut gpiob.crh);
  

    /* Set up sysclk and freeze it */
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    /* Set up systick delay */
   let _ = cp.SYST.delay(&clocks);

    loop {
        if light_sensor.is_high() {
            buzzer.set_low();
        } else {
            buzzer.set_high();
        }
    }
}