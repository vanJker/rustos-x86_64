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

    // let ptr = 0xdeadbeaf as *mut u8;
    // unsafe { *ptr = 42; }

    // let ptr = 0x20549a as *mut u8;

    // unsafe {
    //     let _x = *ptr;
    // }
    // println!("read worked");

    // unsafe {
    //     *ptr = 42;
    // }
    // println!("write worked");

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table);

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rustos_x86_64::hlt_loop();
}
