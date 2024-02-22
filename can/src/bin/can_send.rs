#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

//Setup serial
use defmt as _;
use defmt_brtt as _;

//Define panic behaviour
use panic_probe as _;

use stm32f4xx_hal as hal;
//Hal library
use hal::{pac, prelude::*};

// Add the bxcan crate as a dependency
use bxcan::filter::Mask32;
use bxcan::{Fifo, Frame, StandardId};
use cortex_m_rt::entry;
use nb::block;

#[entry] //Intro point of program
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    //Opsætter clocks
    let rcc = dp.RCC.constrain();

    // External clock på Nucleo F446Re er 8Mhz
    // Gennem PLL bliver denne til 180Mhz
    // CAN hører til periphral clock 1 -> 45MHz for high speed can :D
    let _clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(180.MHz()).pclk1(45.MHz());
    defmt::info!("Clocks set up");

    //Spkitter GPIOA op i individuelle pins
    let gpioa = dp.GPIOA.split();
    //Påbegynder opsætning af CAN
    let mut can1 = {
        //Definer rx og tx pins - pa11 og pa12 passer til CAN1 på de grønne boards
        let rx = gpioa.pa11.into_alternate::<9>();
        let tx = gpioa.pa12.into_alternate::<9>();

        //Linker pins til can controller
        let can = dp.CAN1.can((tx, rx));
        defmt::info!("Can linked to pins");

        bxcan::Can::builder(can)
            // APB1 (PCLK1): 45MHz, Bit rate: 1MBit/s, Sample Point 87.5%
            // Value was calculated with http://www.bittiming.can-wiki.info/
            .set_bit_timing(0x00390002)
            //.set_automatic_retransmit(true)
            .enable()
        //NOTE! Can::builder looks for 11 recesive bits in a row, as to ensure it is connected.
    };
    defmt::info!("Can set up");

    //Enabler interrupts, så den interrupter når der er en ny besked i fifo0, fifo0 er fuld eller der er overrun
    //I dette eksempel bliver interrupt ikke bruge :)
    can1.enable_interrupts({
        use bxcan::Interrupts as If;
        If::FIFO0_MESSAGE_PENDING | If::FIFO0_FULL | If::FIFO0_OVERRUN
    });
    defmt::info!("Interrupts enabled");

    // Opsætter filter til at acceptere alle beskeder - kan også opsættes til at acceptere specifikke id'er
    let mut filters = can1.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    defmt::info!("Filters set up");

    // Drop filters to leave filter configuration mode.
    drop(filters);

    //En tilfældig id ＼（〇_ｏ）／
    let id: u16 = 0x500;

    //Frame::Data kan håndtere [u8; N], 1<N<=8
    let mut test: [u8; 1] = [0; 1];
    defmt::info!("Starting loop");
    loop {
        test[0] += 1;
        //Creates a frame from the id and test
        let test_frame = Frame::new_data(StandardId::new(id).unwrap(), test);
        defmt::info!("Sending frame {:?}", test_frame);
        //Transmits the frame - ignores error
        block!(can1.transmit(&test_frame)).unwrap();
        //Removes overflow
        if test[0] >= 255 {
            test[0] = 1;
        }

        //Delay :P
        for _ in 0..1000000 {
            cortex_m::asm::nop();
        }
    }
}
