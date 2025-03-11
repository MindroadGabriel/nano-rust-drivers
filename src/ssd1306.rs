#![allow(unused_variables, dead_code)]

use core::mem::swap;
use codepage_437::CP437_CONTROL;
use embedded_hal::i2c::{I2c, Operation};
use crate::ssd1306_error::Error;
use crate::ssd1306_font::{FONT, FONT_HEIGHT, FONT_HEIGHT_1, FONT_WIDTH, FONT_WIDTH_1};
use crate::ssd1306_registers::*;

const DEFAULT_ADDRESS: u8 = 0x3C;
const TEMP_REGISTER: u8 = 0x15;
pub const BUFFER_SIZE: usize = LCDWIDTH as usize * ((LCDHEIGHT as usize + 7) / 8);

pub struct DisplayDriver<'buffer, I2C> {
    i2c: I2C,
    address: u8,
    buffer: &'buffer mut [u8; BUFFER_SIZE],
    cursor_x: u16,
    cursor_y: u16,
}

impl<'buffer, I2C> DisplayDriver<'buffer, I2C> {}

impl<'buffer, I2C: I2c> DisplayDriver<'buffer, I2C> {
    pub fn new(mut i2c: I2C, address: Option<u8>, buffer: &'buffer mut [u8; BUFFER_SIZE]) -> Result<Self, Error<I2C::Error>> {
        let address = address.unwrap_or(DEFAULT_ADDRESS);
        // i2c.write(address, &[0xE3])?;
        let vcc_state = SWITCHCAPVCC;
        i2c.write(address, &[0x00, DISPLAYOFF, SETDISPLAYCLOCKDIV, 0x80, SETMULTIPLEX, (LCDHEIGHT - 1).try_into().unwrap()])?;
        i2c.write(address, &[0x00, SETDISPLAYOFFSET, 0x00, SETSTARTLINE | 0x00, CHARGEPUMP])?;
        if vcc_state == EXTERNALVCC {
            i2c.write(address, &[0x00, 0x10])?;
        } else {
            i2c.write(address, &[0x00, 0x14])?;
        }
        i2c.write(address, &[0x00, MEMORYMODE, 0x01, 0xA1, 0xC8])?;

        let com_pins = 0x02;
        let contrast = 0x8F;
        i2c.write(address, &[0x00, SETCOMPINS, com_pins])?;
        i2c.write(address, &[0x00, SETCONTRAST, contrast])?;
        if vcc_state == EXTERNALVCC {
            i2c.write(address, &[0x00, SETPRECHARGE, 0x22])?;
        } else {
            i2c.write(address, &[0x00, SETPRECHARGE, 0xF1])?;
        }
        i2c.write(address, &[0x00, SETVCOMDETECT, 0x40, DISPLAYALLON_RESUME, NORMALDISPLAY, DEACTIVATE_SCROLL, DISPLAYON])?;

        Ok(Self {
            i2c,
            address,
            buffer,
            cursor_x: 0,
            cursor_y: 0,
        })
    }

    pub fn start_of_data(&mut self) -> Result<(), Error<I2C::Error>> {
        // self.i2c.write(self.address, &[PAGEADDR, 0, (LCDHEIGHT/2 - 1).try_into().unwrap(), COLUMNADDR, 0, (LCDWIDTH - 1).try_into().unwrap()])?;
        let last_x_pixel_index: u8 = (LCDWIDTH - 1) as u8;
        let last_y_byte_index: u8 = ((LCDHEIGHT / 8) - 1) as u8;
        self.i2c.write(self.address, &[0x00, COLUMNADDR, 0x00, last_x_pixel_index, PAGEADDR, 0x00, last_y_byte_index])?;
        Ok(())
    }

    pub fn display(&mut self) -> Result<(), Error<I2C::Error>> {
        // self.start_of_data()?;
        // self.i2c.write(self.address, &[0x00, PAGEADDR, 0, 0xFF, COLUMNADDR, 0])?;
        // self.i2c.write(self.address, &[0x00, (LCDWIDTH - 1).try_into().unwrap()])?;

        // self.i2c.write(self.address, &subset)?;
        self.display_num(BUFFER_SIZE)?;
        Ok(())
    }

    pub fn display_num(&mut self, num: usize) -> Result<(), Error<I2C::Error>> {
        self.start_of_data()?;
        // self.i2c.write(self.address, &[0x00, PAGEADDR, 0, (LCDHEIGHT - 1).try_into().unwrap(), COLUMNADDR, 0, (LCDWIDTH - 1).try_into().unwrap()])?;
        // self.i2c.write(self.address, &[0x00, (LCDWIDTH - 1).try_into().unwrap()])?;

        // self.i2c.write(self.address, &subset)?;
        // self.i2c.transaction(self.address, &mut [
        //     Operation::Write(&mut [0x40]),
        //     Operation::Write(&self.buffer[..num]),
        // ])?;
        let chunk_size = 32;
        for i in 0..num.div_ceil(chunk_size) {
            // for i in 0..1 {
            let first = chunk_size * i;
            if first < num {
                let mut last = chunk_size * i + chunk_size - 1;
                if last > num - 1 {
                    last = num - 1
                }
                self.i2c.transaction(self.address, &mut [
                    Operation::Write(&mut [0x40]),
                    Operation::Write(&self.buffer[first..=last]),
                ])?;
            }
        }
        Ok(())
    }

    pub fn clear_display(&mut self) {
        self.fill_screen(BLACK);
    }

    pub fn invert_display(&mut self, inverted: bool) -> Result<(), Error<I2C::Error>> {
        if inverted {
            self.i2c.write(self.address, &[0x00, INVERTDISPLAY])?;
        } else {
            self.i2c.write(self.address, &[0x00, NORMALDISPLAY])?;
        }
        Ok(())
    }

    pub fn dim(&mut self, dim: bool) -> Result<(), Error<I2C::Error>> {
        if dim {
            self.i2c.write(self.address, &[0x00, SETCONTRAST, 0x0])?;
        } else {
            self.i2c.write(self.address, &[0x00, SETCONTRAST, 0x8F])?;
        }
        Ok(())
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16, color: u8) -> Result<(), Error<I2C::Error>> {
        if x > LCDWIDTH || y > LCDHEIGHT {
            return Err(Error::OutsideScreenAccess { x: x as i16, y: y as i16 });
        }
        let byte = 1 << (y % 8);
        let byte_index = (LCDHEIGHT / 8) * x + (y / 8);
        if let Some(view) = self.buffer.get_mut(byte_index as usize) {
            *view = *view | byte;
            Ok(())
        } else {
            Err(Error::OutsideScreenAccess {
                x: x as i16,
                y: y as i16,
            })
        }
    }

    pub fn start_scroll_right(&self, start: u8, stop: u8) {}
    pub fn start_scroll_left(&self, start: u8, stop: u8) {}
    pub fn start_scroll_diag_right(&self, start: u8, stop: u8) {}
    pub fn start_scroll_diag_left(&self, start: u8, stop: u8) {}
    pub fn stop_scroll(&self) {}
    pub fn ssd1306_command(&mut self, command: &[u8]) -> Result<(), Error<I2C::Error>> {
        self.i2c.transaction(self.address, &mut [Operation::Write(&[0x00]), Operation::Write(command)])?;
        Ok(())
    }
    // pub fn get_pixel(&self, x: i16, y: i16) -> bool {
    //
    // }
    // pub fn get_buffer(&mut self) -> &[u8] {
    //
    // }
    pub fn draw_line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: u8) -> Result<(), Error<I2C::Error>> {
        let mut x0 = x0 as i16;
        let mut y0 = y0 as i16;
        let mut x1 = x1 as i16;
        let mut y1 = y1 as i16;
        let steep = (y1 as i16 - y0 as i16).abs() > (x1 as i16 - x0 as i16).abs();
        if steep {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = (y1 as i16 - y0 as i16).abs();
        let mut err = dx / 2;
        let y_step: i16 = if y0 < y1 { 1 } else { -1 };

        let mut x_runner = x0;
        let mut y_runner = y0;
        while x_runner <= x1 {
            if steep {
                self.draw_pixel(y_runner as u16, x_runner as u16, color)?;
            } else {
                self.draw_pixel(x_runner as u16, y_runner as u16, color)?;
            }
            err -= dy;
            if err < 0 {
                y_runner += y_step;
                err += dx;
            }
            x_runner += 1;
        }
        Ok(())
    }
    pub fn draw_fast_h_line(&mut self, x: u16, y: u16, w: u16, color: u8) -> Result<(), Error<I2C::Error>> {
        if x >= LCDWIDTH || y >= LCDHEIGHT {
            return Err(Error::OutsideScreenAccess { x: x as i16, y: y as i16 });
        } else if x + w >= LCDWIDTH {
            return Err(Error::OutsideScreenAccess { x: (x + w) as i16, y: y as i16 });
        }
        // A faster implementation can be added here
        self.draw_line(x, y, x + w, y, color)
    }
    pub fn draw_fast_v_line(&mut self, x: u16, y: u16, h: u16, color: u8) -> Result<(), Error<I2C::Error>> {
        if x >= LCDWIDTH || y >= LCDHEIGHT {
            return Err(Error::OutsideScreenAccess { x: x as i16, y: y as i16 });
        } else if y + h >= LCDHEIGHT {
            return Err(Error::OutsideScreenAccess { x: x as i16, y: (y + h) as i16 });
        }
        // A faster implementation can be added here
        self.draw_line(x, y, x, y + h, color)
    }
    pub fn draw_fill_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color: u8) -> Result<(), Error<I2C::Error>> {
        for i in x..x + w {
            self.draw_fast_v_line(i, y, h, color)?
        }
        Ok(())
    }
    pub fn fill_screen(&mut self, color: u8) {
        if color != 0 {
            for i in 0..BUFFER_SIZE {
                self.buffer[i] = 0xFF;
            }
        } else {
            for i in 0..BUFFER_SIZE {
                self.buffer[i] = 0x00;
            }
        }
    }
    pub fn fill_screen_byte(&mut self, byte: u8) {
        for i in 0..BUFFER_SIZE {
            self.buffer[i] = byte;
        }
    }
    pub fn draw_rect(x: i16, y: i16, w: i16, h: i16, color: u16) {}
    pub fn draw_circle(x0: i16, y0: i16, r: i16, color: u16) {}
    pub fn draw_circle_helper(x0: i16, y0: i16, r: i16, corner_name: u8, color: u16) {}
    pub fn fill_circle(x0: i16, y0: i16, r: i16, color: u16) {}
    pub fn fill_circle_helper(x0: i16, y0: i16, r: i16, corner_name: u8, color: u16) {}


    pub fn read(&mut self) -> Result<u8, I2C::Error> {
        let mut temp = [0];
        self.i2c.write_read(DEFAULT_ADDRESS, &[TEMP_REGISTER], &mut temp)?;
        Ok(temp[0])
    }
    pub fn draw_string(&mut self, text: &str) {
        for character in text.chars() {
            // println!("print {}", character);
            self.draw_char(character);
        }
    }
    pub fn draw_char(&mut self, character: char) {
        if character == '\n' {
            self.cursor_x = 0;
            self.cursor_y += FONT_HEIGHT_1;
        } else if character != '\r' {
            if self.cursor_x + FONT_WIDTH_1 > LCDWIDTH {
                self.cursor_x = 0;
                self.cursor_y += FONT_HEIGHT_1;
            }
            self.draw_char_at(self.cursor_x, self.cursor_y, character, WHITE);
            self.cursor_x += FONT_WIDTH_1;
        }
    }
    pub fn draw_char_at(&mut self, x: u16, y: u16, character: char, color: u8) {
        // println!("draw_char_at");
        if (x + FONT_WIDTH >= LCDWIDTH)
            || (y + FONT_HEIGHT >= LCDHEIGHT)
        // || (x + FONT_WIDTH) < 0
        // || (y + FONT_HEIGHT) < 0
        {
            return;
        }

        // println!("{}, {}, {}", character, x, y);
        if let Some(encoded) = CP437_CONTROL.encode(character) {
            for i in 0..FONT_WIDTH {
                // println!("{}, {}", x, i);
                if let Some(line) = FONT.get(encoded as usize * FONT_WIDTH as usize + i as usize) {
                    let mut line = *line;
                    for j in 0..8 {
                        if line & 1 != 0 {
                            self.draw_pixel(x + i, y + j, color);
                        }
                        line >>= 1;
                    }
                }
            }
        }
    }
    pub fn set_cursor(&mut self, x: u16, y: u16) {
        self.cursor_x = x;
        self.cursor_y = y;
    }
}
