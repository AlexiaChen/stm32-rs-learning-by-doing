#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
//use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{
    pac, 
    prelude::*,
    timer::{Channel, Tim2NoRemap},
};

#[entry]
fn main() -> ! {
    /* Get access to device and core peripherals */
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    /* Get access to RCC, and GPIOA */
    let rcc = dp.RCC.constrain();
  
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();

    /* Set up LED pin */
    let led0 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = dp
    .TIM2
    .pwm_hz::<Tim2NoRemap, _, _>(led0, &mut afio.mapr, 1.kHz(), &clocks);

    pwm.enable(Channel::C1);

    // frequency = 1 / period = 1000Hz
    pwm.set_period(1.kHz());

    //asm::bkpt();
    /* Set up systick delay */
    let mut delay = cp.SYST.delay(&clocks);
    let max_duty = pwm.get_max_duty();
    let step = (max_duty / 100) as usize;
    loop {
        /* Light show */
        for i in (0..=max_duty).step_by(step) {
            pwm.set_duty(Channel::C1, i);
            delay.delay_ms(10_u16);
        }
        for i in (0..=max_duty).rev().step_by(step) {
            pwm.set_duty(Channel::C1, i);
            delay.delay_ms(10_u16);
        }
    }
}