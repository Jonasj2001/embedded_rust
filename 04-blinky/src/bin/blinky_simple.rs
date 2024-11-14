#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

//Setup serial
use defmt as _;
use defmt_rtt as _;

//Define panic behaviour
use panic_probe as _;

//Hal library
use stm32f4xx_hal as hal;
use hal::{pac, prelude::*};


use cortex_m_rt::entry;


#[entry] //Intro point of program
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap(); //Extract peripherals

    let gpioa = dp.GPIOA.split(); //Extract GPIOA
    let mut led = gpioa.pa5.into_push_pull_output(); //Set pin A5 as output

    defmt::debug!("Starting blink sequence!");
    loop {
        defmt::info!("Led on:");
        led.set_high();
        for _ in 0..100000 {}

        defmt::info!("Led off");
        led.set_low();
        for _ in 0..100000 {}
    }
}
