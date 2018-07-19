#![no_std]
#![no_main]

extern crate betafpv_f3;
extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use betafpv_f3::hal::prelude::*;
use betafpv_f3::Board;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {

    let Board {mut led, mut delay, ..} = Board::new();

    loop {
        led.set_high();

        delay.delay_ms(500u16);

        led.set_low();

        delay.delay_ms(500u16);
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
