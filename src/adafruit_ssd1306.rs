#![allow(unused_variables, dead_code)]
// use embedded_hal::i2c::I2c;
//
// const ADDR: u8 = 0x15;
// const TEMP_REGISTER: u8 = 0x15;
//
// pub struct AdafruitSSD1306Driver<I2C> {
//     i2c: I2C,
// }
//
// impl<I2C: I2c> AdafruitSSD1306Driver<I2C> {
//     pub fn new(i2c: I2C) -> Self {
//         Self { i2c }
//     }
//
//     pub fn begin(&mut self, reset: bool, periph_begin: bool) {
//
//     }
//
//     pub fn display(&self) {
//
//     }
//
//     pub fn clear_display(&self) {
//
//     }
//
//     pub fn invert_display(&self, i: bool) {
//
//     }
//
//     pub fn dim(&self, dim: bool) {
//
//     }
//
//     pub fn draw_pixel(&self, x: i16, y: i16, color: u16) {
//
//     }
//
//     pub fn draw_fast_h_line(&self, x: i16, y: i16, w: i16, color: u16) {
//
//     }
//     pub fn draw_fast_v_line(&self, x: i16, y: i16, h: i16, color: u16) {
//
//     }
//     pub fn start_scroll_right(&self, start: u8, stop: u8) {
//
//     }
//     pub fn start_scroll_left(&self, start: u8, stop: u8) {
//
//     }
//     pub fn start_scroll_diag_right(&self, start: u8, stop: u8) {
//
//     }
//     pub fn start_scroll_diag_left(&self, start: u8, stop: u8) {
//
//     }
//     pub fn stop_scroll(&self) {
//
//     }
//     pub fn ssd1306_command(&self, c: u8) {
//
//     }
//     pub fn get_pixel(&self, x: i16, y: i16) -> bool {
//
//     }
//     pub fn get_buffer(&mut self) -> &[u8] {
//
//     }
//     pub fn start_write() {
//
//     }
//     pub fn write_line(x0: u16, y0: u16, x1: i16, y1: i16, color: u16) {
//
//     }
//     pub fn end_write() {
//
//     }
//     pub fn set_rotation(r: u8) {
//
//     }
//     // pub fn invert_display(i: bool) {
//     //
//     // }
//     pub fn fill_rect(x: i16, y: i16, w: i16, h: i16, color: u16) {
//
//     }
//     pub fn fill_screen(color: u16) {
//
//     }
//     pub fn draw_line(x0: i16, y0: i16, x1: i16, y1: i16, color: u16) {
//
//     }
//     pub fn draw_rect(x: i16, y: i16, w: i16, h: i16, color: u16) {
//
//     }
//     pub fn draw_circle(x0: i16, y0: i16, r: i16, color: u16) {
//
//     }
//     pub fn draw_circle_helper(x0: i16, y0: i16, r: i16, corner_name: u8, color: u16) {
//
//     }
//     pub fn fill_circle(x0: i16, y0: i16, r: i16, color: u16) {
//
//     }
//     pub fn fill_circle_helper(x0: i16, y0: i16, r: i16, corner_name: u8, color: u16) {
//
//     }
//
//
//     pub fn read(&mut self) -> Result<u8, I2C::Error> {
//         let mut temp = [0];
//         self.i2c.write_read(ADDR, &[TEMP_REGISTER], &mut temp)?;
//         Ok(temp[0])
//     }
// }
