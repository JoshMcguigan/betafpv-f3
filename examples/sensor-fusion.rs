#![no_std]
#![no_main]

extern crate betafpv_f3;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate bit_bang_serial;
extern crate mpu9250;
extern crate imu;
extern crate byteorder;
extern crate cobs;

use betafpv_f3::hal::prelude::*;
use betafpv_f3::Board;
use rt::ExceptionFrame;
use mpu9250::I16x3;
use core::f32::consts::PI;
use imu::{filter_update, Q, V, Euler};
use byteorder::{LE, ByteOrder};
use betafpv_f3::write::write_to;

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
    for _i in 0..3 {
        led.set_high();

        delay.delay_ms(250u32);

        led.set_low();

        delay.delay_ms(250u32);
    }

    // TODO calibrate sensors before starting
    // TODO look at including checksum to see if display smooths out

    let mut orientation = Q {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0
    };

    loop {

        let raw_g = mpu.accel().unwrap();
        let g_scale = 2.0 / 32_767.0; // scaled to units of g

        let raw_ar = mpu.gyro().unwrap();
        let ar_scale = 250.0 / 32_767.0 * (PI / 180.0); // scaled to rad/s

        let scaled_g = scale_to_f32x3(raw_g, g_scale);
        // Fix the X Y Z components of the accelerometer so they match the gyro axes
        let scaled_g = V {
            x: scaled_g.y,
            y: -scaled_g.x,
            z: scaled_g.z,
        };
        let scaled_ar = scale_to_f32x3(raw_ar, ar_scale);

        orientation = filter_update(
            scaled_ar.clone(),
            scaled_g.clone(),
            orientation,
            0.02 // sampling period in seconds
        );

        let euler = Euler::from(orientation.clone());

//        // human readable output
//        for string in [
//                format_args!("gx: {}\n\r", scaled_g.x),
//                format_args!("gy: {}\n\r", scaled_g.y),
//                format_args!("gz: {}\n\r", scaled_g.z),
//                format_args!("arx: {}\n\r", scaled_ar.x),
//                format_args!("ary: {}\n\r", scaled_ar.y),
//                format_args!("arz: {}\n\r", scaled_ar.z),
//                format_args!("q1: {}\n\r", orientation.q1),
//                format_args!("q2: {}\n\r", orientation.q2),
//                format_args!("q3: {}\n\r", orientation.q3),
//                format_args!("q4: {}\n\r", orientation.q4),
//            ].iter() {
//
//            let mut buf = [0u8; 64];
//            let s: &str = write_to::show(
//                &mut buf,
//                *string,
//            ).unwrap();
//            tx.write(&mut delay, s.as_bytes());
//        }
//        delay.delay_ms(2000u32);


        // machine readable output
        // use with https://github.com/japaric/f3/blob/v0.5.3/viz
        // Serialize the quaternion
        let mut start = 0;
        let mut buf: [u8; 16] = [0; 16];
        LE::write_f32(&mut buf[start..start + 4], orientation.w);
        start += 4;
        LE::write_f32(&mut buf[start..start + 4], orientation.x);
        start += 4;
        LE::write_f32(&mut buf[start..start + 4], orientation.y);
        start += 4;
        LE::write_f32(&mut buf[start..start + 4], orientation.z);
        // start += 4;

        // Log data
        let mut output_buf: [u8; 18] = [0; 18]; // this array needs space for cob overhead, plus delimiter byte
        cobs::encode(&buf, &mut output_buf);

        tx.write(&mut delay, &output_buf);
    }
}

fn scale_to_f32x3(item: I16x3, scale: f32) -> V {
    V {
        x: item.x as f32 * scale,
        y: item.y as f32 * scale,
        z: item.z as f32 * scale,
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
