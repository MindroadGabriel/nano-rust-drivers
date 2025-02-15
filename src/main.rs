#![allow(unused_variables, unused_mut, dead_code)]
#![no_std]
#![no_main]

mod adafruit_ssd1306;
mod bmi160;
mod bmi160_registers;
mod bmi160_error;

use panic_halt as _;
use core::cell::RefCell;
use arduino_hal::Peripherals;
use arduino_hal::prelude::*;
use ufmt::{Formatter, uDisplay};


fn print_type_name<S: ufmt::uWrite, T>(serial: &mut S, _: T)
    where S: ufmt::uWrite<Error=core::convert::Infallible> {
    let type_name = nostd::any::type_name::<T>();
    ufmt::uwrite!(serial, "{}", type_name).unwrap_infallible();
}

// enum UDisplayError {
//     HalI2cError(arduino_hal::i2c::Error),
// }

enum UDisplayError<I2CError> {
    HalI2cError(arduino_hal::i2c::Error),
    BMI160Error(bmi160_error::Error<I2CError>)
}

impl<I2CError> uDisplay for UDisplayError<I2CError> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error> where W: _ufmt_uWrite + ?Sized {
        match self {
            UDisplayError::HalI2cError(error) => {
                match error {
                    arduino_hal::i2c::Error::ArbitrationLost => f.write_str("ArbitrationLost"),
                    arduino_hal::i2c::Error::AddressNack => f.write_str("AddressNack"),
                    arduino_hal::i2c::Error::DataNack => f.write_str("DataNack"),
                    arduino_hal::i2c::Error::BusError => f.write_str("BusError"),
                    arduino_hal::i2c::Error::Unknown => f.write_str("Unknown"),
                }
            }
            UDisplayError::BMI160Error(error) => {
                f.write_str("BMI160 error: ")?;
                error.fmt(f)
            }
        }
    }
}

impl<I2CError> From<arduino_hal::i2c::Error> for UDisplayError<I2CError> {
    fn from(value: arduino_hal::i2c::Error) -> Self {
        UDisplayError::HalI2cError(value)
    }
}
impl<I2CError> From<bmi160_error::Error<I2CError>> for UDisplayError<I2CError> {
    fn from(value: bmi160_error::Error<I2CError>) -> Self {
        UDisplayError::BMI160Error(value)
    }
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );
    let result = (|| -> Result<(), UDisplayError<arduino_hal::i2c::Error>> {
        let i2c_ref_cell = RefCell::new(i2c);
        // let mut display = adafruit_ssd1306::AdafruitSSD1306Driver::new(
        //     embedded_hal_bus::i2c::RefCellDevice::new(&i2c_ref_cell),
        // );
        // display.read()?;
        let mut accelerometer = bmi160::Driver::new(embedded_hal_bus::i2c::RefCellDevice::new(&i2c_ref_cell), None, None)?;
        ufmt::uwrite!(&mut serial, "BMI160 initialized\n").unwrap_infallible();

        let mut led = pins.d13.into_output();

        // Print some text
        ufmt::uwrite!(&mut serial, "Hello from rust arduino!\n").unwrap_infallible();
        print_type_name(&mut serial, &led);

        loop {
            led.toggle();
            //arduino_hal::delay_ms(1000);
            accelerometer.update()?;
            if let Some(output_data) = accelerometer.get_output_data() {
                let x = (output_data.acceleration.x * 100.0) as i32;
                let y = (output_data.acceleration.y * 100.0) as i32;
                let z = (output_data.acceleration.z * 100.0) as i32;
                let t = (output_data.temperature * 100.0) as i32;
                ufmt::uwrite!(&mut serial, "X: {}, Y: {}, Z: {}, T: {}\n", x, y, z, t).unwrap_infallible();
            }
        }
    })();
    if let Err(error) = result {
        ufmt::uwrite!(&mut serial, "Error: {}\n", error).unwrap_infallible();
    }
    loop {}
}
