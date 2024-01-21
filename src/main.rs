#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use crate::vga::vga_buffer::WRITER;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}
