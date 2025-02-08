use embedded_hal::i2c::I2c;

const ADDR: u8 = 0x15;
const TEMP_REGISTER: u8 = 0x15;

pub struct AdafruitSSD1306Driver<I2C> {
    i2c: I2C,
}

impl<I2C: I2c> AdafruitSSD1306Driver<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn read(&mut self) -> Result<u8, I2C::Error> {
        let mut temp = [0];
        self.i2c.write_read(ADDR, &[TEMP_REGISTER], &mut temp)?;
        Ok(temp[0])
    }
}
