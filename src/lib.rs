#![no_std]
pub extern crate stm32f30x_hal as hal;

use hal::gpio::{Output, PushPull};
use hal::gpio::gpioc::PC15;

use hal::prelude::*;
use hal::stm32f30x::Peripherals;
use hal::spi::Spi;
use hal::hal::spi::{Mode, Phase, Polarity};
use hal::gpio::gpioa::PA15;
use hal::gpio::AF5;
use hal::gpio::gpiob::{PB3, PB4, PB5};
use hal::stm32f30x::SPI1;

pub struct Board {
    pub led: PC15<Output<PushPull>>,
    pub mpu: Mpu,
}

/// Motion Processing Unit
/// MPU6000
pub struct Mpu {
    pub nss: PA15<Output<PushPull>>,
    pub spi: Spi<SPI1, (PB3<AF5>, PB4<AF5>, PB5<AF5>)>
}

impl Board {
    pub fn new(p: Peripherals) -> Self {
        let mut rcc = p.RCC.constrain();
        let mut flash = p.FLASH.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
        let mut gpiob = p.GPIOB.split(&mut rcc.ahb);
        let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

        let led = gpioc
            .pc15
            .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

        let nss = gpioa
            .pa15
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

        let sck = gpiob.pb3.into_af5(&mut gpiob.moder, &mut gpiob.afrl);
        let miso = gpiob.pb4.into_af5(&mut gpiob.moder, &mut gpiob.afrl);
        let mosi = gpiob.pb5.into_af5(&mut gpiob.moder, &mut gpiob.afrl);

        let mode = Mode {
            phase: Phase::CaptureOnFirstTransition,
            polarity: Polarity::IdleLow,
        }; // this configuration is a guess for now

        let spi = Spi::spi1(
            p.SPI1,
            (sck, miso, mosi),
            mode,
            1.mhz(),
            clocks,
            &mut rcc.apb2,
        );

        Board { led, mpu: Mpu {nss, spi} }
    }
}
