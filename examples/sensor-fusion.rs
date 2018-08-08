#![no_std]
#![no_main]

extern crate betafpv_f3;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate bit_bang_serial;
extern crate madgwick;
extern crate mpu9250;

use betafpv_f3::hal::prelude::*;
use betafpv_f3::Board;
use rt::ExceptionFrame;
use betafpv_f3::write::write_to;
use madgwick::F32x3;
use mpu9250::I16x3;
use core::f32::consts::PI;

entry!(main);

///
/// WORK IN PROGRESS
///

fn main() -> ! {
    let Board {mut led, motor_1: output_pin, mut mpu, mut delay, ..} = Board::new();

    // https://www.invensense.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf
    // expected 0x68 based on register map
    // some startup time is required or this assertion fails
    delay.delay_ms(1000u32);
    assert_eq!(mpu.who_am_i().unwrap(), 0x68);

    let baud = 9600u32;
    let time_adjustment = -6i32;
    let mut tx = bit_bang_serial::Tx::new(output_pin, baud, time_adjustment);


    // blinking LED means the assertion was correct
    for _i in 0..5 {
        led.set_high();

        delay.delay_ms(500u32);

        led.set_low();

        delay.delay_ms(500u32);
    }

    let mut marg = madgwick::Marg::new(1f32, 1f32);

    loop {
//        let orientation = marg.update(
//            stub_magnetometer_value(),
//            to_f32x3(mpu.accel().unwrap()),
//            to_f32x3(mpu.gyro().unwrap())
//        );

        let raw_g = mpu.accel().unwrap();
        let g_scale = 2.0 / 32_767.0; // scaled to units of g

        let raw_ar = mpu.gyro().unwrap();
        let ar_scale = 250.0 / 32_767.0 * (PI / 180.0); // scaled to rad/s

        let scaled_g = scale_to_f32x3(raw_g, g_scale);
        let scaled_ar = scale_to_f32x3(raw_ar, ar_scale);

        for string in [
                format_args!("gx: {}\n\r", scaled_g.x),
                format_args!("gy: {}\n\r", scaled_g.y),
                format_args!("gz: {}\n\r", scaled_g.z),
                format_args!("arx: {}\n\r", scaled_ar.x),
                format_args!("ary: {}\n\r", scaled_ar.y),
                format_args!("arz: {}\n\r", scaled_ar.z),
            ].iter() {

            let mut buf = [0u8; 64];
            let s: &str = write_to::show(
                &mut buf,
                *string,
            ).unwrap();
            tx.write(&mut delay, s.as_bytes());

        }

        delay.delay_ms(2000u32);
    }
}

fn scale_to_f32x3(item: I16x3, scale: f32) -> F32x3 {
    F32x3 {
        x: item.x as f32 * scale,
        y: item.y as f32 * scale,
        z: item.z as f32 * scale,
    }
}

fn stub_magnetometer_value() -> F32x3 {
    F32x3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
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