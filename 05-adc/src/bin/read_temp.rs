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
use stm32f4xx_hal::{self as hal};
use hal::{pac, prelude::*, adc::{Adc, Temperature, config::{AdcConfig,SampleTime}}};


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

    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());
    adc.enable_temperature_and_vref();
    // let gpioa = dp.GPIOA.split(); //Extract GPIOA
    // let mut led = gpioa.pa5.into_push_pull_output(); //Set pin A5 as output
    
    defmt::debug!("Starting blink sequence!");
    loop {
        let x = adc.convert(&Temperature, SampleTime::Cycles_480);
        let x_celcius = ((x as f32 - 940 as f32)/2.5)+25 as f32;
        defmt::info!("Temp Raw: {}, mv: {}, Celcius: {}",x,adc.sample_to_millivolts(x), x_celcius);
        // defmt::info!("Led on:");
        // led.set_high();
        // _delay.delay_ms(250);
// 
        // defmt::info!("Led off");
        // led.set_low();
        _delay.delay_ms(250);
    }
}
