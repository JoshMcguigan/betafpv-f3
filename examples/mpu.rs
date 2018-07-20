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
    let Board {mut led, mut mpu, mut delay, ..} = Board::new();

    // https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf
    // expected 0x68 based on register map
    // some startup time is required or this assertion fails
    delay.delay_ms(1000u16);
    assert_eq!(mpu.who_am_i().unwrap(), 0x68);

    // blinking LED means the assertion was correct
    for _i in 0..5 {
        led.set_high();

        delay.delay_ms(500u16);

        led.set_low();

        delay.delay_ms(500u16);
    }

    // LED controlled by orientation of board
    loop {
        let board_up = mpu.accel().unwrap().z > 0;

        if board_up {
            led.set_high();
        } else {
            led.set_low();
        }

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
