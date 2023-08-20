#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use stm32_utils::oled::oled_i2c::Oled;
use stm32f1xx_hal::{pac::{interrupt, TIM2}, pac, prelude::*,  timer::{CounterMs, Event}, gpio::OutputSpeed};


// Create a Global Variable for the Timer Peripheral that I'm going to pass around.
static G_TIM: Mutex<RefCell<Option<CounterMs<TIM2>>>> = Mutex::new(RefCell::new(None));
static mut COUNTER: u32 = 0;

// Timer Interrupt
#[interrupt]
fn TIM2() {
    unsafe {
        COUNTER += 1
    }

    cortex_m::interrupt::free(|cs| {
        // Obtain access to Global Timer Peripheral and Clear Interrupt Pending Flag
        let mut timer = G_TIM.borrow(cs).borrow_mut();
        timer.as_mut().unwrap().clear_interrupt(Event::Update);
    });
}

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

    {
        //  the scope ensures that the timer is released once the initialization is done
        let mut flash = dp.FLASH.constrain();
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        // timer 1 second interval
        let mut timer = dp.TIM2.counter(&clocks);
    
        timer.start(1000.millis()).unwrap();
        timer.listen(Event::Update);
        
        unsafe {
            cortex_m::peripheral::NVIC::unmask(interrupt::TIM2);
        }
    
        cortex_m::interrupt::free(|cs| {
            G_TIM.borrow(cs).replace(Some(timer));
        });
       
    } // initialization ends here  must be done in a scope
   
    oled.show_string(1, 1, "Count:");
    
    loop {
        oled.show_number(1, 7, unsafe { COUNTER }, 5);
    }
}