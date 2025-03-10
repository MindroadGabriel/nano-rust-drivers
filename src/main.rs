#![allow(unused_variables, unused_mut, dead_code)]
#![no_std]
#![no_main]

mod ssd1306;
mod bmi160;
mod bmi160_registers;
mod bmi160_error;
mod byte_stuffing;
mod error;
#[macro_use]
mod print;
mod ssd1306_registers;
mod ssd1306_error;
mod ssd1306_font;

use core::cell::RefCell;
use panic_halt as _;
use arduino_hal::Peripherals;
use arduino_hal::prelude::*;
use crate::ssd1306::BUFFER_SIZE;
use crate::ssd1306_registers::WHITE;

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
        let mut accelerometer = bmi160::Driver::new(embedded_hal_bus::i2c::RefCellDevice::new(&i2c_ref_cell), None, None, |ms| arduino_hal::delay_ms(ms))?;
        let mut buffer = [0x00; ssd1306::BUFFER_SIZE];
        let display_result = ssd1306::DisplayDriver::new(embedded_hal_bus::i2c::RefCellDevice::new(&i2c_ref_cell), None, &mut buffer);
        // print::print_type_name(&display_result);
        //
        // core::result::Result<
        // hackathon_pong_controller::ssd1306::DisplayDriver<embedded_hal_bus::i2c::refcell::RefCellDevice<avr_hal_generic::i2c::I2c<atmega_hal::Atmega,
        // avr_device::devices::atmega328p::TWI,
        // avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input,
        // atmega_hal::port::PC4>,
        // avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input,
        // atmega_hal::port::PC5>,
        // avr_hal_generic::clock::MHz16>>>,
        // hackathon_pong_controller::ssd1306_error::Error<avr_hal_generic::i2c::Error>>
        let mut display = match display_result {
            Ok(display) => {
                display
            }
            Err(error) => {
                return Err(error.into());
            }
        };

        // println!("BMI160 initialized");
        println!("Started");

        // loop {
        //     // let button1_pressed = button1.is_low();
        //     // let button2_pressed = button2.is_low();
        //     // if button1_pressed && !button1_last_pressed {
        //     //     println!("Button1 pressed");
        //     // }
        //     // if button2_pressed && !button2_last_pressed {
        //     //     println!("Button2 pressed");
        //     // }
        //     // button1_last_pressed = button1_pressed;
        //     // button2_last_pressed = button2_pressed;
        //     //
        //     // led.toggle();
        //     // accelerometer.update()?;
        //     // if let Some(output_data) = accelerometer.get_output_data() {
        //     // }
        //     println!("On");
        //     display.fill_screen(ssd1306_registers::WHITE);
        //     display.display()?;
        //     led.set_high();
        //     arduino_hal::delay_ms(2000);
        //
        //     println!("Off");
        //     display.fill_screen(ssd1306_registers::BLACK);
        //     display.display()?;
        //     led.set_low();
        //     arduino_hal::delay_ms(2000);
        // }
        loop {
            display.fill_screen(ssd1306_registers::BLACK);
            for i in 0..32 {
                display.draw_pixel(i, i, WHITE)?;
            }
            for i in 0..32 {
                display.draw_pixel(31 + i, 31, WHITE)?;
            }
            for i in 0..32 {
                display.draw_pixel(63 + i, 31 - i, WHITE)?;
            }
            for i in 0..32 {
                display.draw_pixel(95 + i, 0, WHITE)?;
            }
            display.display()?;
            arduino_hal::delay_ms(1000);
            display.draw_line(4, 4, 57, 25, WHITE)?;
            display.display()?;
            arduino_hal::delay_ms(1000);
            display.draw_line(57, 4, 4, 25, WHITE)?;
            display.display()?;
            arduino_hal::delay_ms(1000);
            display.draw_line(97, 15, 44, 30, WHITE)?;
            display.display()?;
            arduino_hal::delay_ms(1000);
            display.draw_line(44, 15, 97, 30, WHITE)?;
            display.display()?;
            arduino_hal::delay_ms(1000);
            display.draw_fill_rect(4, 4, 57 - 4, 25 - 4, WHITE)?;
            display.display()?;
            // display.dim(true)?;
            arduino_hal::delay_ms(1000);
            // display.dim(false)?;
            arduino_hal::delay_ms(1000);

            display.clear_display();
            display.set_cursor(0, 0);
            // display.draw_string("You're a mean one,\nMister Grinch!");
            display.draw_string(" Abc");
            display.display()?;
            arduino_hal::delay_ms(2000);
        }
    })();
    if let Err(error) = result {
        #[cfg(feature = "string-errors")]
        println!("Error: {}", error);
    }
    loop {}
}
