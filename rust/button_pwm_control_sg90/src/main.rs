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
use stm32f1xx_hal::gpio::{Input, PullUp};
use stm32f1xx_hal::gpio::gpiob::PB1;

// 舵机的占空比是0.5ms / 20ms ~ 2.5ms / 20ms  => 1 / 40 ~ 5 / 40  CRR: max_duty* (1 / 40) = 500    max_duty* (5 / 40) = 2500 => 舵机CCR 500 - 2500
fn get_duty_sg90_from_angle(angle: f32, max_duty: u16) -> u16 {
    let start_ccr = max_duty / 40;
    let end_ccr = max_duty  / 8;
    let ccr_range = end_ccr - start_ccr;

    let duty = (start_ccr as f32  + ccr_range as f32 * (angle / 180f32)) as u16;
    duty
}

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
    let pin1 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = dp
    .TIM2
    .pwm_hz::<Tim2NoRemap, _, _>(pin1, &mut afio.mapr, 50.Hz(), &clocks);

    pwm.enable(Channel::C2);

    // frequency = 1 / period = 50Hz
    pwm.set_period(50.Hz());

    let mut angle = 0.0;
   
    oled.show_string(1, 1, "Angle:");
   
    let max_duty = pwm.get_max_duty();

    oled.show_string(2, 1, "Max:");
    oled.show_number(2, 5, max_duty as u32, 10);
    loop {
       
        let btn_state = read_button(&mut button);
        if btn_state == 1 {
            angle += 30.0f32;
           
            if angle > 180.0f32 {
                angle = 0.0f32;
            }
            let duty = get_duty_sg90_from_angle(angle, max_duty);
            oled.show_number(3, 1, duty as u32, 5);
            pwm.set_duty(Channel::C2, duty);
            oled.show_number(1, 7, angle as u32, 3);
       }
    }
}