#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32_utils::oled::oled_i2c::{init_oled, show_char_oled};

#[entry]
fn main() -> ! {
   
    init_oled();

    show_char_oled(1, 1, 'A');
    //show_string_oled(1, 3, "Hello World!");

    loop {
     
    }
}