#![allow(unused_variables, dead_code)]

/// BMI160 is a accelerometer from Bosch

use embedded_hal::i2c::I2c;
use crate::bmi160_registers::*;
use crate::bmi160_error::*;

const ADDR: u8 = 0x15;
const TEMP_REGISTER: u8 = 0x15;


pub struct Driver<I2C> {
    i2c: I2C,
    address: u8,
    calibration: Option<CalibrationData>,
    a_res: f32,
    g_res: f32,
    output_data: Option<OutputData>,
}

#[derive(Clone)]
pub struct CalibrationData {
    accel_bias: [f32; 3],
    gyro_bias: [f32; 3],
}

#[derive(Clone)]
pub struct OutputData {
    pub acceleration: Vector,
    pub gyro: Vector,
    pub temperature: f32,
}

#[derive(Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<I2C: I2c> Driver<I2C> {
    pub fn new(mut i2c: I2C, address: Option<u8>, calibration: Option<CalibrationData>) -> Result<Self, Error<I2C::Error>> {
        let address = address.unwrap_or(DEFAULT_ADDRESS);

        // Check that we're talking to a chip with the right chip ID
        let mut chip_id: [u8; 1] = [0x00];
        i2c.write_read(address, &[CHIP_ID], chip_id.as_mut_slice())?;
        if chip_id[0] != CHIP_ID_DEFAULT_VALUE {
            return Err(Error::WrongChipId(chip_id[0]));
        }

        // Soft reset
        i2c.write(address, &[CMD, 0xB6])?;
        arduino_hal::delay_ms(100);

        // Start up accelerometer
        i2c.write(address, &[CMD, 0x11])?;
        arduino_hal::delay_ms(100);

        // Start up gyroscope
        i2c.write(address, &[CMD, 0x15])?;
        arduino_hal::delay_ms(100);

        // Set up full scale accel range +-16G
        i2c.write(address, &[ACC_RANGE, 0x0C])?;
        // Set up full scale gyro range +-2000dps
        i2c.write(address, &[GYR_RANGE, 0x00])?;
        // Set Accel ODR to 500hz, BWP mode to oversample 4, LPF of ~40.5hz
        i2c.write(address, &[ACC_CONF, 0x0A])?;
        // Set Gyro ODR to 500hz, BWP mode to oversample 4, LPF of ~34.15hz
        i2c.write(address, &[GYR_CONF, 0x0A])?;


        Ok(Self {
            i2c,
            address,
            calibration,
            a_res: 16.0 / 32768.0,
            g_res: 2000.0 / 32768.0,
            output_data: None,
        })
    }

    pub fn update(&mut self) -> Result<(), Error<I2C::Error>>{
        let mut raw_data: [u8; 12] = [0; 12];
        let mut signed_data: [i16; 6] = [0; 6];

        // Read 12 raw data registers
        self.i2c.write_read(self.address, &[GYR_X_L], &mut raw_data)?;

        // Convert to signed
        for i in 0..6 {
            signed_data[i] = (((raw_data[i*2 + 1] as u16) << 8) | raw_data[i*2] as u16) as i16;
        }
        let ax = (signed_data[3] as f32) * self.a_res;
        let ay = (signed_data[4] as f32) * self.a_res;
        let az = (signed_data[5] as f32) * self.a_res;

        let gx = (signed_data[0] as f32) * self.g_res;
        let gy = (signed_data[1] as f32) * self.g_res;
        let gz = (signed_data[2] as f32) * self.g_res;

        // Read temperature
        let mut temperature_raw_data: [u8; 2] = [0; 2];
        self.i2c.write_read(self.address, &[TEMPERATURE_0], &mut temperature_raw_data)?;
        let temperature_signed = (((temperature_raw_data[1] as u16) << 8) | temperature_raw_data[0] as u16) as i16;
        let temperature = (temperature_signed as f32 / 512.0) + 23.0;

        if let Some(calibration) = &self.calibration {
            self.output_data = Some(OutputData {
                acceleration: Vector {
                    x: ax - calibration.accel_bias[0],
                    y: ay - calibration.accel_bias[1],
                    z: az - calibration.accel_bias[2],
                },
                gyro: Vector {
                    x: gx - calibration.gyro_bias[0],
                    y: gy - calibration.gyro_bias[1],
                    z: gz - calibration.gyro_bias[2],
                },
                temperature,
            });
        } else {
            self.output_data = Some(OutputData {
                acceleration: Vector {
                    x: ax,
                    y: ay,
                    z: az,
                },
                gyro: Vector {
                    x: gx,
                    y: gy,
                    z: gz,
                },
                temperature,
            });
        }

        Ok(())
    }

    pub fn get_output_data(&self) -> &Option<OutputData> {
        &self.output_data
    }
}