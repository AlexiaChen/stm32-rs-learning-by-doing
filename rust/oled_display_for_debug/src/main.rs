#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32_utils::oled::oled_i2c::Oled;
use stm32f1xx_hal::{pac, prelude::*, gpio::OutputSpeed};

#[entry]
fn main() -> ! {
   
    let dp = pac::Peripherals::take().unwrap();
    let mut gpiob = dp.GPIOB.split();

    let mut scl = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
    let mut sda = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);

    scl.set_speed(&mut gpiob.crh, stm32f1xx_hal::gpio::IOPinSpeed::Mhz50);
    sda.set_speed(&mut gpiob.crh, stm32f1xx_hal::gpio::IOPinSpeed::Mhz50);
    scl.set_high();
    sda.set_high();

    let mut oled = Oled::new(scl, sda);
    oled.init();
    oled.show_char(1, 1, 'I');
    oled.show_string(1, 3, "Fuck you!");
    oled.show_number(2, 1, 12345, 5);
    oled.show_singed_number(2, 7, -66, 2);
    oled.show_hex_number(3, 1, 0xAA55, 4);
    oled.show_binary_number(4,1, 0b111101011, 9);

    loop {
     
    }
}