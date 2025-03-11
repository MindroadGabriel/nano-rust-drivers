pub const BLACK: u8 = 0;   ///< Draw 'off' pixels
pub const WHITE: u8 = 1;   ///< Draw 'on' pixels
pub const INVERSE: u8 = 2; ///< Invert pixels

pub const MEMORYMODE: u8 = 0x20;          ///< See datasheet
pub const COLUMNADDR: u8 = 0x21;          ///< See datasheet
pub const PAGEADDR: u8 = 0x22;            ///< See datasheet
pub const SETCONTRAST: u8 = 0x81;         ///< See datasheet
pub const CHARGEPUMP: u8 = 0x8D;          ///< See datasheet
pub const SEGREMAP: u8 = 0xA0;            ///< See datasheet
pub const DISPLAYALLON_RESUME: u8 = 0xA4; ///< See datasheet
pub const DISPLAYALLON: u8 = 0xA5;        ///< Not currently used
pub const NORMALDISPLAY: u8 = 0xA6;       ///< See datasheet
pub const INVERTDISPLAY: u8 = 0xA7;       ///< See datasheet
pub const SETMULTIPLEX: u8 = 0xA8;        ///< See datasheet
pub const DISPLAYOFF: u8 = 0xAE;          ///< See datasheet
pub const DISPLAYON: u8 = 0xAF;           ///< See datasheet
pub const COMSCANINC: u8 = 0xC0;          ///< Not currently used
pub const COMSCANDEC: u8 = 0xC8;          ///< See datasheet
pub const SETDISPLAYOFFSET: u8 = 0xD3;    ///< See datasheet
pub const SETDISPLAYCLOCKDIV: u8 = 0xD5;  ///< See datasheet
pub const SETPRECHARGE: u8 = 0xD9;        ///< See datasheet
pub const SETCOMPINS: u8 = 0xDA;          ///< See datasheet
pub const SETVCOMDETECT: u8 = 0xDB;       ///< See datasheet

pub const SETLOWCOLUMN: u8 = 0x00;  ///< Not currently used
pub const SETHIGHCOLUMN: u8 = 0x10; ///< Not currently used
pub const SETSTARTLINE: u8 = 0x40;  ///< See datasheet

pub const EXTERNALVCC: u8 = 0x01;  ///< External display voltage source
pub const SWITCHCAPVCC: u8 = 0x02; ///< Gen. display voltage from 3.3V

pub const RIGHT_HORIZONTAL_SCROLL: u8 = 0x26;              ///< Init rt scroll
pub const LEFT_HORIZONTAL_SCROLL: u8 = 0x27;               ///< Init left scroll
pub const VERTICAL_AND_RIGHT_HORIZONTAL_SCROLL: u8 = 0x29; ///< Init diag scroll
pub const VERTICAL_AND_LEFT_HORIZONTAL_SCROLL: u8 = 0x2A;  ///< Init diag scroll
pub const DEACTIVATE_SCROLL: u8 = 0x2E;                    ///< Stop scroll
pub const ACTIVATE_SCROLL: u8 = 0x2F;                      ///< Start scroll
pub const SET_VERTICAL_SCROLL_AREA: u8 = 0xA3;             ///< Set scroll range

pub const LCDWIDTH: u16 = 128;
pub const LCDHEIGHT: u16 = 64;
