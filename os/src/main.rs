#![no_std]
#![no_main]

mod console;
mod syscall;

use core::panic::PanicInfo;
use syscall::sys_exit;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    println!("Hello, world!");
    sys_exit(9);
    loop {}
}
