#![no_std]
#![no_main]

extern crate betafpv_f3;
extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use betafpv_f3::hal::prelude::*;
use betafpv_f3::hal::stm32f30x::Peripherals;
use betafpv_f3::Board;
use cortex_m::asm::nop;
use rt::ExceptionFrame;
use betafpv_f3::Mpu;

entry!(main);

fn main() -> ! {
    // TODO confirm SPI mode in lib.rs
    let Board {mut led, mpu: Mpu{mut nss, mut spi}} = Board::new(Peripherals::take().unwrap());

    nss.set_low();

    // 0x75 from https://github.com/betaflight/betaflight/blob/master/src/main/drivers/accgyro/accgyro_mpu.h
    // 0x80 from https://github.com/betaflight/betaflight/blob/0b8709df29b173261b79d799708fbfcb13b7748a/src/main/drivers/bus_spi.c
    let mut buffer = [0x75 | 0x80, 0];
    spi.transfer(&mut buffer).unwrap();

    nss.set_high();

    loop {
        for _ in 0..buffer[1]/10 {
            led.set_high();

            for _i in 0..300_000 {
                nop();
            }

            led.set_low();

            for _i in 0..300_000 {
                nop();
            }
        }

        loop {
            led.set_high();

            for _i in 0..2_000_000 {
                nop();
            }

            led.set_low();

            for _i in 0..2_000_000 {
                nop();
            }
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
