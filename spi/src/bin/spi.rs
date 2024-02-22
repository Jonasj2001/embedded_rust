#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

//Setup serial
use defmt as _;
use defmt_brtt as _;

use embedded_hal::spi::FullDuplex;
//Define panic behaviour
use panic_probe as _;

//Hal library
use stm32f4xx_hal as hal;
use hal::{pac, prelude::*, spi};


use cortex_m_rt::entry;


#[entry] //Intro point of program
fn main() -> ! {
    //Cortex core peripherals
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    
    //Hal peripherals
    let dp = pac::Peripherals::take().unwrap(); //Extract peripherals

    let rcc = dp.RCC.constrain(); //Grab clocks
    let clocks = 
        rcc.cfgr.use_hse(8.MHz())
        .sysclk(84.MHz())
        .freeze(); //Set sysclk compared to external 8Mhz oscillator:
    defmt::info!("Sysclk running at: {}", clocks.sysclk().raw());

    let mut _delay = cp.SYST.delay(&clocks);
    let gpioa = dp.GPIOA.split(); //Extract GPIOA


    //Define pins for SPI
    let sclk = gpioa.pa5.into_alternate::<5>();
    let miso = gpioa.pa6.into_alternate::<5>();
    let mosi = gpioa.pa7.into_alternate::<5>();
    let mut cs = gpioa.pa9.into_push_pull_output();
    cs.set_high(); //Initiate SPI for device off state (This case high).

    //SPI settings
    let spi_mode = spi::Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    };
    //Setup SPI controller 1
    let mut spi1 = dp.SPI1.spi(
        (sclk, miso, mosi),
        spi_mode,
        2.MHz(), //Note clock is based on a prescaler of the peripheral clock! 2, 4, 8, 16, 32, 64, 128, 256 fpclk / prescaler
        &clocks
    );
    //Alternative method:
    
    // let mut spi1 = spi::Spi::new(
    //     dp.SPI1,
    //     (sclk, miso, mosi),
    //     spi_mode,
    //     2.MHz(),
    //     &clocks
    // );


    //SPI Options:
    spi1.bit_format(spi::BitFormat::MsbFirst); // MSB standard
    spi1.enable(true); //Enabled by default after creation
    spi1.listen(spi::Event::RxNotEmpty);

    cs.set_low();
    spi1.write(&[0x03, 0x0, 0x0, 0x0]).unwrap(); //Write discards any incoming transmissions
    
    for _ in 0..20 {
        while spi1.is_busy() {}
        spi1.send(0x0).unwrap();
        while spi1.is_rx_not_empty() {
            match spi1.read_nonblocking() {
                Ok(w) => defmt::info!("Received: {:x}", w),
                Err(_err) => continue,
            };
        }
    }
    cs.set_high();
    loop {} //main() -> ! means it runs forever!
}
