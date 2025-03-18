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
mod debouncing;

use core::cell::RefCell;
use core::mem::MaybeUninit;
use panic_halt as _;
use arduino_hal::Peripherals;
use arduino_hal::prelude::*;
use crate::debouncing::Debouncer;
use crate::debouncing::DebounceResult::Pressed;
use crate::ssd1306::BUFFER_SIZE;
use crate::ssd1306_registers::WHITE;

enum GameState {
    Menu,
    Displaying,
    Inputting,
    Next,
    Failure,
    Score,
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    print::put_console(serial);
    let mut led = pins.d13.into_output();

    let result = (|| -> Result<(), error::UDisplayError<arduino_hal::i2c::Error>> {
        let mut i2c = arduino_hal::I2c::new(
            dp.TWI,
            pins.a4.into_pull_up_input(),
            pins.a5.into_pull_up_input(),
            50000,
        );
        let mut button1 = pins.d5.into_pull_up_input();
        let mut button2 = pins.d4.into_pull_up_input();
        let i2c_ref_cell = RefCell::new(i2c);
        let mut buffer = [0x00; ssd1306::BUFFER_SIZE];
        let display_result = ssd1306::DisplayDriver::new(embedded_hal_bus::i2c::RefCellDevice::new(&i2c_ref_cell), None, &mut buffer);
        let mut display = match display_result {
            Ok(display) => {
                display
            }
            Err(error) => {
                return Err(error.into());
            }
        };
        let mut debouncer_storage = [0x00; 2];
        let mut debouncer = Debouncer::new(&mut debouncer_storage[..]);
        println!("Started");

        const MAX_SEQUENCE: usize = 128;
        let mut sequence_storage = [MaybeUninit::new(false); MAX_SEQUENCE];
        let mut sequence = fixed_slice_vec::FixedSliceVec::new(&mut sequence_storage[..]);
        sequence.clear();
        let game_state = GameState::Menu;
        let mut next_guess_index = 0;
        let mut highest_cleared = 0;
        let mut first = true;
        sequence.push(true);

        loop{
            // if button1.is_low() {
            //     println!("B1 low");
            // }
            let button1_state = debouncer.update(0, button1.is_low());
            let button2_state = debouncer.update(1, button2.is_low());
            if button1_state == Pressed {
                println!("B1");
            }
            if button2_state == Pressed {
                println!("B2");
            }
            display.set_cursor(0, 0);
            // display.draw_string("Sequence memory! Try buttons.");
            display.fill_screen(WHITE);
            display.display()?;
            arduino_hal::delay_ms(100);
            display.clear_display();
            display.draw_char('a')?;
            // display.draw_string("Sequence memory! Try buttons.");
            display.display()?;

            println!("Loop");
            led.set_high();
            arduino_hal::delay_ms(1000);
            led.set_low();
            arduino_hal::delay_ms(1000);
        }
    })();
    if let Err(error) = result {
        #[cfg(feature = "string-errors")]
        println!("Error: {}", error);
        loop {
            led.set_high();
            arduino_hal::delay_ms(100);
            led.set_low();
            arduino_hal::delay_ms(100);
        }
    }
    loop {}
}
