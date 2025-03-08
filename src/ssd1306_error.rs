use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource};
use ufmt::{Formatter, uWrite};

pub enum Error<I2CError> {
    WrongChipId(u8),
    I2cError(I2CError),
    OutsideScreenAccess {
        x: i16,
        y: i16,
    },
}

impl<I2CError> From<I2CError> for Error<I2CError>
    where I2CError: embedded_hal::i2c::Error {
    fn from(value: I2CError) -> Self {
        Self::I2cError(value)
    }
}

impl<I2CError> ufmt::uDisplay for Error<I2CError>
    where I2CError: embedded_hal::i2c::Error {
    fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), <W as uWrite>::Error> where W: uWrite + ?Sized {
        match self {
            Error::WrongChipId(id) => {
                fmt.write_str("wrong chip id")
            }
            Error::I2cError(error) => {
                fmt.write_str("i2c error: ")?;
                match error.kind() {
                    ErrorKind::Bus => {
                        fmt.write_str("bus")
                    }
                    ErrorKind::ArbitrationLoss => {
                        fmt.write_str("arbitration loss")
                    }
                    ErrorKind::NoAcknowledge(a) => {
                        fmt.write_str("no acknowledge, ")?;
                        match a {
                            NoAcknowledgeSource::Address => {
                                fmt.write_str("address")
                            }
                            NoAcknowledgeSource::Data => {
                                fmt.write_str("data")
                            }
                            NoAcknowledgeSource::Unknown => {
                                fmt.write_str("unknown")
                            }
                        }
                    }
                    ErrorKind::Overrun => {
                        fmt.write_str("overrun")
                    }
                    ErrorKind::Other => {
                        fmt.write_str("other")
                    }
                    _ => {
                        fmt.write_str("unknown")
                    }
                }
            }
            Error::OutsideScreenAccess { x, y } => {
                fmt.write_str("oob: ")?;
                x.fmt(fmt)?;
                fmt.write_str(", ")?;
                y.fmt(fmt)
            }
        }
    }
}