#![no_std]
#![no_main]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate panic_semihosting;
extern crate stm32f30x_hal;

use rt::ExceptionFrame;
use core::ptr;
use cortex_m::asm::nop;

entry!(main);

fn main() -> ! {
    const RCC_AHBENR: u32 = 0x40021014;
    // page 51
    const GPIOC_MODER: u32 = 0x48000800;
    // page 51 for start of GPIOC plus offset 18 on page 240
    const GPIOC_BSRR: u32 = 0x48000818;

    unsafe {
        // reference manual page 148
        *(RCC_AHBENR as *mut u32) = 1 << 19;

        // page 237
        *(GPIOC_MODER as *mut u32) = 1 << 30;

        loop {
            // page 240
            ptr::write_volatile(GPIOC_BSRR as *mut u32, 1 << (15+16));

            for _i in 0..100_000 {
                nop();
            }

            ptr::write_volatile(GPIOC_BSRR as *mut u32, 1 << (15));

            for _i in 0..100_000 {
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
