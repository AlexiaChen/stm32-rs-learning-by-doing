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


static mut INT_PIN_A_O: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA0<Input<PullUp>>> =
    MaybeUninit::uninit();

static mut INT_PIN_A_1: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA1<Input<PullUp>>> =
    MaybeUninit::uninit();

static mut COUNTER: i32 = 0;

// PA0 interrupt handler
#[interrupt]
fn EXTI0() {
    let int_pin_pa0 = unsafe { &mut *INT_PIN_A_O.as_mut_ptr() };
    if int_pin_pa0.check_interrupt() {
      
        let int_pin_pa1 = unsafe { &mut *INT_PIN_A_1.as_mut_ptr() };
        if int_pin_pa1.is_low() {
            unsafe {
                COUNTER -= 1
            }
        } 
      
        // if we don't clear this bit, the ISR would trigger indefinitely
        int_pin_pa0.clear_interrupt_pending_bit();
    }
}


// PA1 interrupt handler
#[interrupt]
fn EXTI1() {
    let int_pin_pa1 = unsafe { &mut *INT_PIN_A_1.as_mut_ptr() };
    if int_pin_pa1.check_interrupt() {
      
        let int_pin_pa0 = unsafe { &mut *INT_PIN_A_O.as_mut_ptr() };
        if int_pin_pa0.is_low() {
            unsafe {
                COUNTER += 1
            }
        }
        // if we don't clear this bit, the ISR would trigger indefinitely
        int_pin_pa1.clear_interrupt_pending_bit();
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
        let mut gpioa = dp.GPIOA.split();
    
        let int_pin_a0 = unsafe { &mut *INT_PIN_A_O.as_mut_ptr() };
        let int_pin_a1 = unsafe { &mut *INT_PIN_A_1.as_mut_ptr() };
        
        let pa0 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
        let pa1 = gpioa.pa1.into_pull_up_input(&mut gpioa.crl);
        *int_pin_a0 = pa0;
        *int_pin_a1 = pa1;
        
        int_pin_a0.make_interrupt_source(&mut afio);
        int_pin_a0.trigger_on_edge(&mut dp.EXTI, Edge::Falling);
        int_pin_a0.enable_interrupt(&mut dp.EXTI);
       
        int_pin_a1.make_interrupt_source(&mut afio);
        int_pin_a1.trigger_on_edge(&mut dp.EXTI, Edge::Falling);
        int_pin_a1.enable_interrupt(&mut dp.EXTI);
    } // initialization ends here
   
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI0);
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI1);
    }

    oled.show_string(1, 1, "Count:");
    loop {
        oled.show_singed_number(1, 7,  unsafe { COUNTER } , 5);
    }
}