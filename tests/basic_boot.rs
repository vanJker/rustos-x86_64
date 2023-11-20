#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos_x86_64::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustos_x86_64::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos_x86_64::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_prinltn output");
}
