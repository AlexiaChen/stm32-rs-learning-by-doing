#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use panic_halt as _;
use core::mem::MaybeUninit;
use pac::interrupt;
use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::{pac, prelude::*, gpio::PullUp, gpio::Input};

use stm32_utils::oled::oled_i2c::Oled;


static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpiob::PB14<Input<PullUp>>> =
    MaybeUninit::uninit();

static mut COUNTER: u32 = 0;

#[interrupt]
fn EXTI15_10() {
    let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };
    if int_pin.check_interrupt() {
      
        unsafe {
            COUNTER += 1
        }
        // if we don't clear this bit, the ISR would trigger indefinitely
        int_pin.clear_interrupt_pending_bit();
    }
}

#[entry]
fn main() -> ! {

    let mut dp = pac::Peripherals::take().unwrap();
    let mut gpiob = dp.GPIOB.split();

    let mut scl = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
    let mut sda = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);

    scl.set_speed(&mut gpiob.crh, stm32f1xx_hal::gpio::IOPinSpeed::Mhz50);
    sda.set_speed(&mut gpiob.crh, stm32f1xx_hal::gpio::IOPinSpeed::Mhz50);
    scl.set_high();
    sda.set_high();
    
    let mut oled = Oled::new(scl, sda);
    oled.init();

  
    {
        let mut afio = dp.AFIO.constrain();
    
        let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };
        
        let sensor = gpiob.pb14.into_pull_up_input(&mut gpiob.crh);
        *int_pin = sensor;
        int_pin.make_interrupt_source(&mut afio);
        int_pin.trigger_on_edge(&mut dp.EXTI, Edge::Falling);
        int_pin.enable_interrupt(&mut dp.EXTI);
    } // initialization ends here
   
    unsafe {
        // set priority to 1
    
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI15_10);

    }

    oled.show_string(1, 1, "Count:");
    loop {
        oled.show_number(1, 7,  unsafe { COUNTER } , 5);
    }
}