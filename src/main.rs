#![no_std]
#![no_main]

use core::panic::PanicInfo;
use vga::WRITER;

mod vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"Hello, World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    use core::fmt::Write;
    WRITER.lock().write_str("Hello again").unwrap();
    write!(WRITER.lock(), "\nSome numbers: {} and {}\n", 42, 1.337).unwrap();

    println!("Hello, World{}", "!");

    // panic!("some panic message");

    loop {}
}
