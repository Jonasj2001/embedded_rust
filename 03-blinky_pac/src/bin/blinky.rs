#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

//Setup serial
use defmt as _;
use defmt_rtt as _;

//Define panic behaviour
use panic_probe as _;
use cortex_m_rt::entry;
use cortex_m::asm::nop;

use stm32f4::stm32f401;


#[entry] //Intro point of program
fn main() -> ! {
    let dp = stm32f401::Peripherals::take().unwrap();

    // Enable GPIOA
    dp.RCC.ahb1enr.write(|w| w.gpioaen().set_bit());
    // Set push pull
    dp.GPIOA.moder.write(|w| w.moder5().bits(0b01));
    
    defmt::debug!("Starting Loop");
    loop {
        for _ in 0..100000 {nop()}
        defmt::info!("On:");
        dp.GPIOA.odr.write(|w| w.odr5().set_bit());
        for _ in 0..100000 {nop()}
        defmt::info!("Off");
        dp.GPIOA.odr.write(|w| w.odr5().clear_bit() );
    }
}
