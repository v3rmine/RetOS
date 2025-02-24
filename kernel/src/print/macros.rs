use core::fmt;

use crate::print::buffer::WRITER;

#[macro_export]
macro_rules! println {
    () => ($crate::print::macros::_print(format_args!("\n")));
    ($($arg:tt)*) => ($crate::print::macros::_print(format_args!("{}\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::macros::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.write().write_fmt(args).unwrap();
}