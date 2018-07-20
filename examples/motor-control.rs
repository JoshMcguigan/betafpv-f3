#![no_std]
#![no_main]

extern crate betafpv_f3;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use betafpv_f3::hal::prelude::*;
use betafpv_f3::Board;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {

    let Board {mut motor_1, mut motor_2, mut motor_3, mut motor_4, mut delay, ..} = Board::new();

    loop {
        motor_1.set_high();
        motor_2.set_high();
        motor_3.set_high();
        motor_4.set_high();

        delay.delay_ms(2000u16);

        motor_1.set_low();
        motor_2.set_low();
        motor_3.set_low();
        motor_4.set_low();

        delay.delay_ms(2000u16);
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
