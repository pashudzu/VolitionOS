use core::fmt::{Error, Write};

struct Console;

impl Write for Console {
    fn write_str<W: write>(f: &mut write, s: &str) -> Result<(), Error> {}
    fn write_char<W: write>(f: &mut write, s: &char) -> Result<(), Error> {}
    fn write_fmt<W: write>(f: &mut write, s: &str) -> Result<(), Error> {}
}