use core::fmt::{Error, Write};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1, 
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11, 
    LightRed = 12, 
    Pink = 13,
    Yellow = 14,
    White = 15,
}

struct Console;

impl Write for Console {
    fn write_str<W: write>(f: &mut write, s: &str) -> Result<(), Error> {}
    fn write_char<W: write>(f: &mut write, s: &char) -> Result<(), Error> {}
    fn write_fmt<W: write>(f: &mut write, s: &str) -> Result<(), Error> {}
}