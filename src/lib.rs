#![no_std]

pub extern crate stm32f30x_hal as hal;
use hal::prelude::*;
use hal::delay::Delay;
use hal::gpio::{Output, PushPull, AF5};
use hal::gpio::gpioa::*;
use hal::gpio::gpiob::*;
use hal::gpio::gpioc::*;
use hal::spi::Spi;
use hal::stm32f30x;
use hal::cortex_m;

extern crate mpu9250;
use mpu9250::Mpu9250;

type Led = PC15<Output<PushPull>>;

type BoardSpi = Spi<stm32f30x::SPI1, (PB3<AF5>, PB4<AF5>, PB5<AF5>)>;
type BoardSpiNss = PA15<Output<PushPull>>;
type Mpu = mpu9250::Mpu9250<BoardSpi, BoardSpiNss, mpu9250::Imu>;

type Motor1 = PA6<Output<PushPull>>;
type Motor2 = PA7<Output<PushPull>>;
type Motor3 = PB8<Output<PushPull>>;
type Motor4 = PB9<Output<PushPull>>;

pub struct Board {
    pub led: Led,
    pub mpu: Mpu,
    pub delay: Delay,
    /// motor outputs are supplied voltage only when battery is connected, they are not powered by USB
    /// there are two green LEDs which also come on when battery voltage is applied
    /// the green LEDs do not seem to be controlled by GPIO
    pub motor_1: Motor1,
    pub motor_2: Motor2,
    pub motor_3: Motor3,
    pub motor_4: Motor4,
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

        let motor_1 = gpioa
            .pa6
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        let motor_2 = gpioa
            .pa7
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        let motor_3 = gpiob
            .pb8
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let motor_4 = gpiob
            .pb9
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

        Board { led, mpu, delay, motor_1, motor_2, motor_3, motor_4 }
    }
}
