use core::fmt::{self, Write};

use crate::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for c in s.chars() {
      console_putchar(c as usize);
    }
    Ok(())
  }
}

pub fn print(args: fmt::Arguments) {
  Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {
    $crate::console::print(format_args!($($arg)*));
  };
}
#[macro_export]
macro_rules! println {
  ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
  ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
}


