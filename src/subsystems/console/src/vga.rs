/*
 * vga.rs - interface for writing to the VGA text buffer and displaying text on the screen
 * Copyright (C) 2025 pashudzu
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

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

struct ColorColor(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((foreground as u8) << 4 | (background as u8))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 120;
const BUFFER_WIDTH: usize = 30;

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

struct Console {
    column_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Write for Console {
    fn write_str<W: write>(f: &mut write, s: &str) -> Result<(), Error> {}
    fn write_char<W: write>(f: &mut write, s: &char) -> Result<(), Error> {}
    fn write_fmt<W: write>(f: &mut write, s: &str) -> Result<(), Error> {}
}