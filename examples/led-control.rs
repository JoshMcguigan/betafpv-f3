#![no_std]
#![no_main]

extern crate betafpv_f3;
extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use betafpv_f3::hal::prelude::*;
use betafpv_f3::hal::stm32f30x;
use cortex_m::asm::nop;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let p = stm32f30x::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();


    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

    let mut led = gpioc
        .pc15
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

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
