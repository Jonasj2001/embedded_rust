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

// If HAL is in Cargo.toml it must be initialized in code:
use stm32f4xx_hal as _;

#[entry] //Intro point of program
fn main() -> ! {
    defmt::debug!("Starting Loop");
    loop {
        for _ in 0..100000 {nop()}
        defmt::info!("Hello World!");
    }
}
