#![no_std]
pub extern crate stm32f30x_hal as hal;
extern crate cortex_m;

use hal::gpio::{Output, PushPull};
use hal::gpio::gpioc::PC15;

use hal::prelude::*;
use hal::stm32f30x;
use hal::spi::Spi;

extern crate mpu9250;
use mpu9250::Mpu9250;
use hal::delay::Delay;

pub struct Board {
    pub led: PC15<Output<PushPull>>,
    pub mpu: mpu9250::Mpu9250<hal::spi::Spi<hal::stm32f30x::SPI1, (hal::gpio::gpiob::PB3<hal::gpio::AF5>, hal::gpio::gpiob::PB4<hal::gpio::AF5>, hal::gpio::gpiob::PB5<hal::gpio::AF5>)>, hal::gpio::gpioa::PA15<hal::gpio::Output<hal::gpio::PushPull>>, mpu9250::Imu>,
}

impl Board {
    pub fn new() -> Self {
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = stm32f30x::Peripherals::take().unwrap();
        let mut rcc = dp.RCC.constrain();
        let mut flash = dp.FLASH.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
        let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

        let led = gpioc
            .pc15
            .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

        let nss = gpioa
            .pa15
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

        let sck = gpiob.pb3.into_af5(&mut gpiob.moder, &mut gpiob.afrl);
        let miso = gpiob.pb4.into_af5(&mut gpiob.moder, &mut gpiob.afrl);
        let mosi = gpiob.pb5.into_af5(&mut gpiob.moder, &mut gpiob.afrl);

        let spi = Spi::spi1(
            dp.SPI1,
            (sck, miso, mosi),
            mpu9250::MODE,
            1.mhz(),
            clocks,
            &mut rcc.apb2,
        );
        let mut delay = Delay::new(cp.SYST, clocks);

        let mpu = Mpu9250::imu(spi, nss, &mut delay).unwrap();

        Board { led, mpu }
    }
}
