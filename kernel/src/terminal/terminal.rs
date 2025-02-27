use crate::printer::buffer::Writer;
use alloc::string::String;
use core::convert::Infallible;
use core::fmt::Write;
use embedded_cli::__private::io::ErrorType;

impl ErrorType for Writer {
    type Error = Infallible;
}

impl embedded_cli::__private::io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        // Debugger
        //self.write_str(&alloc::format!("{:x?}\n", buf)).unwrap();

        // See https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
        match buf {
            // Unknown?
            [0x1B, 0x5B, 0x50] => {},
            // Backspace
            [0x1B, 0x5B, 0x44] => self.erase_char(),
            // Erase line sequence
            // ESC[2K
            [0x1B, 0x5B, 0x32, 0x4B] => self.clear_line(),
            _ => self.write_str(&String::from_utf8_lossy(buf)).unwrap()
        }
        Ok(buf.len())
    }

    /// Not needed
    fn flush(&mut self) -> Result<(), Self::Error> {
        //self.clear();
        Ok(())
    }
}