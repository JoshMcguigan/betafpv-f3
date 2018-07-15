#![no_std]
pub extern crate stm32f30x_hal as hal;

use hal::gpio::{Output, PushPull};
use hal::gpio::gpioc::PC15;

use hal::prelude::*;
use hal::stm32f30x::Peripherals;

pub struct Board {
    pub led: PC15<Output<PushPull>>
}

impl Board {
    pub fn new(p: Peripherals) -> Self {
        let mut rcc = p.RCC.constrain();

        let mut _gpioa = p.GPIOA.split(&mut rcc.ahb);
        let mut _gpiob = p.GPIOB.split(&mut rcc.ahb);
        let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

        let mut led = gpioc
            .pc15
            .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

        Board { led }
    }
}
