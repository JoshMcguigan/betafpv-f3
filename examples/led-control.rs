#![no_std]
#![no_main]

extern crate betafpv_f3;
extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use betafpv_f3::hal::prelude::*;
use betafpv_f3::Board;
use cortex_m::asm::nop;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {

    let Board {mut led, ..} = Board::new();

    loop {
        led.set_high();

        for _i in 0..100_000 {
            nop();
        }

        led.set_low();

        for _i in 0..100_000 {
            nop();
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
