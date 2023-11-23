#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos_x86_64::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustos_x86_64::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos_x86_64::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos_x86_64::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");

    rustos_x86_64::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rustos_x86_64::hlt_loop();

    // use rustos_x86_64::print;
    // loop {
    //     for _ in 0..10000 {}
    //     print!("-");
    // }
}
