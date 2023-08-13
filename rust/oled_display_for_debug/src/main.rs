#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32_utils::oled::oled_i2c::Oled;

#[entry]
fn main() -> ! {
   
    let mut oled = Oled::new();
    oled.init();
    oled.show_string(1, 3, "Fuck you!");

    loop {
     
    }
}