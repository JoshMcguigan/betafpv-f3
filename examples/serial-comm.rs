#![no_std]
#![no_main]

extern crate betafpv_f3;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f30x_hal;
extern crate bit_bang_serial;

use betafpv_f3::hal::prelude::*;
use betafpv_f3::Board;
use rt::ExceptionFrame;
use stm32f30x_hal::gpio::PushPull;
use stm32f30x_hal::gpio::Output;
use stm32f30x_hal::delay::Delay;
use stm32f30x_hal::gpio::gpioa::PA6;

entry!(main);

///
/// NOTE: Connect the positive and negative motor leads with a 10 ohm resistor
///

fn main() -> ! {

    let Board {motor_1: mut output_pin, mut delay, ..} = Board::new();

    let baud = 9600u32;
    let time_adjustment = -6i32;
    let mut tx = bit_bang_serial::Tx::new(output_pin, baud, time_adjustment);

    loop {
        for i in 0..10 {
            let text = "test".as_bytes();
            tx.write(&mut delay, "index: ".as_bytes());
            tx.write(&mut delay, &[(i+48) as u8]);
            tx.write(&mut delay, "\n\r".as_bytes());

            delay.delay_ms(500u32);
        }
    }

}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
