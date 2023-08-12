#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
//use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*};


use stm32f1xx_hal::gpio::{Input, Output, PullUp, PushPull};
use stm32f1xx_hal::gpio::gpiob::PB11;
use stm32f1xx_hal::gpio::gpioa::PA1;

// read the button state
// if button is pressed, turn on the LED 
// if button is not pressed, turn off the LED
fn read_button(button: &mut PB11<Input<PullUp>>) -> i32 {
    let mut key_num: i32 = 0;
    
    if button.is_low() {
        loop {
            if button.is_high() {
                key_num = 1;
                break;
            }
        }
    }
    key_num
}

// turn on/off the LED
fn turn_led(led :&mut PA1<Output<PushPull>>) {
    if led.is_set_high() {
        led.set_low();
    } else {
        led.set_high();
    }
}

// turn ff the LED
fn turn_off_led(led :&mut PA1<Output<PushPull>>) {
    led.set_high();
}

#[entry]
fn main() -> ! {
    /* Get access to device and core peripherals */
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    /* Get access to RCC, and GPIOA */
    let _ = dp.RCC.apb2enr.write(|w| { 
        w.iopaen().set_bit(); 
        w.iopben().set_bit()
    });
    let rcc = dp.RCC.constrain();
  
    let mut flash = dp.FLASH.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    /* Set up LED pin */
    let mut button = gpiob.pb11.into_pull_up_input(&mut gpiob.crh);
    let mut led1 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
  

    /* Set up sysclk and freeze it */
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    /* Set up systick delay */
    let  _ = cp.SYST.delay(&clocks);

    turn_off_led(&mut led1);
    loop {
        let btn_state = read_button(&mut button);
        if btn_state == 1 {
            turn_led(&mut led1);
        }
    }
}