#![allow(unused_variables, unused_mut, dead_code)]
#![no_std]
#![no_main]

mod adafruit_ssd1306;
mod bmi160;
mod bmi160_registers;
mod bmi160_error;
mod byte_stuffing;
mod error;
#[macro_use]
mod print;

use core::cell::RefCell;
use panic_halt as _;
use arduino_hal::Peripherals;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    print::put_console(serial);

    let result = (|| -> Result<(), error::UDisplayError<arduino_hal::i2c::Error>> {
        let mut i2c = arduino_hal::I2c::new(
            dp.TWI,
            pins.a4.into_pull_up_input(),
            pins.a5.into_pull_up_input(),
            50000,
        );
        let mut led = pins.d13.into_output();
        let mut button1 = pins.d5.into_pull_up_input();
        let mut button2 = pins.d4.into_pull_up_input();
        let mut button1_last_pressed = button1.is_low();
        let mut button2_last_pressed = button2.is_low();
        let i2c_ref_cell = RefCell::new(i2c);
        let mut accelerometer = bmi160::Driver::new(embedded_hal_bus::i2c::RefCellDevice::new(&i2c_ref_cell), None, None)?;
        println!("BMI160 initialized");
        println!("Hello from rust arduino!");

        loop {
            let button1_pressed = button1.is_low();
            let button2_pressed = button2.is_low();
            if button1_pressed && !button1_last_pressed {
                println!("Button1 pressed");
            }
            if button2_pressed && !button2_last_pressed {
                println!("Button2 pressed");
            }
            button1_last_pressed = button1_pressed;
            button2_last_pressed = button2_pressed;

            led.toggle();
            accelerometer.update()?;
            if let Some(output_data) = accelerometer.get_output_data() {
            }
            arduino_hal::delay_ms(10);
        }
    })();
    if let Err(error) = result {
        println!("Error: {}", error);
    }
    loop {}
}
