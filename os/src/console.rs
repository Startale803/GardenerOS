use core::fmt::{self,Write};
use crate::syscall::sys_write;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let a = s.as_bytes();
        let mut i = 0;
        while i<a.len() {
            let n = sys_write(1,&a[i..]);
            if n<=0 {return Err(fmt::Error);}
            i += n as usize;
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
    () => {$crate::print!("\n");};
    ($($arg:tt)*) => {
        $crate::print!("{}\n",format_args!($($arg)*));
    };
}
