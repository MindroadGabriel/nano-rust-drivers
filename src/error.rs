use arduino_hal::prelude::_ufmt_uWrite;
use ufmt::{Formatter, uDisplay};
use crate::bmi160_error;

pub enum UDisplayError<I2CError> {
    HalI2cError(arduino_hal::i2c::Error),
    BMI160Error(bmi160_error::Error<I2CError>),
    PostcardError(postcard::Error),
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
            UDisplayError::PostcardError(error) => {
                f.write_str("Postcard serialization error.")
            }
        }
    }
}

impl<I2CError> From<arduino_hal::i2c::Error> for UDisplayError<I2CError> {
    fn from(value: arduino_hal::i2c::Error) -> Self {
        Self::HalI2cError(value)
    }
}
impl<I2CError> From<bmi160_error::Error<I2CError>> for UDisplayError<I2CError> {
    fn from(value: bmi160_error::Error<I2CError>) -> Self {
        Self::BMI160Error(value)
    }
}
impl<I2CError> From<postcard::Error> for UDisplayError<I2CError> {
    fn from(value: postcard::Error) -> Self {
        Self::PostcardError(value)
    }
}

