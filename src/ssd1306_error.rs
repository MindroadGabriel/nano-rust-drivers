use ufmt::{Formatter, uWrite};

pub enum Error<I2CError> {
    WrongChipId(u8),
    I2cError(I2CError),
}

impl<I2CError> From<I2CError> for Error<I2CError>
    where I2CError: embedded_hal::i2c::Error {
    fn from(value: I2CError) -> Self {
        Self::I2cError(value)
    }
}
impl<I2CError> ufmt::uDisplay for Error<I2CError> {
    fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), <W as uWrite>::Error> where W: uWrite + ?Sized {
        match self {
            Error::WrongChipId(id) => {
                fmt.write_str("wrong chip id")
            }
            Error::I2cError(error) => {
                fmt.write_str("i2c error: ")
            }
        }
    }
}