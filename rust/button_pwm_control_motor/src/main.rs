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

use stm32_utils::oled::oled_i2c::Oled;
use stm32f1xx_hal::gpio::{Input, PullUp, Output, PushPull};
use stm32f1xx_hal::gpio::gpiob::PB1;
use stm32f1xx_hal::gpio::gpioa::{PA4, PA5};


fn read_button(button: &mut PB1<Input<PullUp>>) -> i32 {
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

fn set_speed_direction(speed: i8, pin4: &mut PA4<Output<PushPull>>, pin5: &mut PA5<Output<PushPull>>) {
    if speed >= 0 {
        // forward
        // set direction pin
        pin4.set_high();
        pin5.set_low();
    } else {
        // backward
        pin4.set_low();
        pin5.set_high();
    }
}

// speed range is [0, 100]
fn get_duty_for_speed(speed_abs: i8, max_duty: u16) -> u16 {
    let start_ccr = 0;
    let end_ccr = max_duty;
    let ccr_range = end_ccr - start_ccr;

    let duty = (start_ccr as f32  + ccr_range as f32 * (speed_abs as f32 / 100f32)) as u16;
    duty
}

#[entry]
fn main() -> ! {
    /* Get access to device and core peripherals */
    let dp = pac::Peripherals::take().unwrap();
    let _ = cortex_m::Peripherals::take().unwrap();

    /* Get access to RCC, and GPIOA */
    let rcc = dp.RCC.constrain();
  
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    let mut scl = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
    let mut sda = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);

    scl.set_high();
    sda.set_high();

    let mut oled = Oled::new(scl, sda);
    oled.init();

    let mut button = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);
    let pin2 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = dp
    .TIM2
    .pwm_hz::<Tim2NoRemap, _, _>(pin2, &mut afio.mapr, 20.kHz(), &clocks);

    pwm.enable(Channel::C3);

    // frequency = 1 / period = 20kHz
    // cause speed is 0 ~ 100, so period is 100   CK_PSC / (PSC + 1) / (ARR + 1) = 20kHz
    pwm.set_period(20.kHz());

    // init direction pin
    let mut pin4 = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let mut pin5 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    let mut speed: i8 = 0;
   
    oled.show_string(1, 1, "Speed:");
   
    let max_duty = pwm.get_max_duty();

    loop {
       
        let btn_state = read_button(&mut button);
        if btn_state == 1 {
            speed += 20;
           
            if speed > 100 {
                speed = -100;
            }
            set_speed_direction(speed, &mut pin4, &mut pin5);
            pwm.set_duty(Channel::C3,  get_duty_for_speed(speed.abs(), max_duty));
            oled.show_singed_number(1, 7, speed as i32, 5);
       }
    }
}