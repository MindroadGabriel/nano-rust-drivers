#![no_std]
#![no_main]

// mod adafruit_ssd1306;

use arduino_hal::prelude::*;
use panic_halt as _;

fn get_type_name<T>(_: T) -> &'static str {
    nostd::any::type_name::<T>()
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    // examples and inspiration, head to https://github.com/Rahix/avr-hal/tree/main/examples

    // let mut i2c = arduino_hal::I2c::new(
    //     dp.TWI,
    //     pins.a4.into_pull_up_input(),
    //     pins.a5.into_pull_up_input(),
    //     50000,
    // );
    //
    // let display = adafruit_ssd1306::AdafruitSSD1306Driver::new(i2c);
    let mut led = pins.d13.into_output();

    let result: core::result::Result<(), core::convert::Infallible> = ufmt::uwrite!(&mut serial, "Hello from rust arduino!");
    let type_name = get_type_name(result);

    result.unwrap_infallible();
    ufmt::uwrite!(&mut serial, "{}", type_name).unwrap_infallible();
    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
