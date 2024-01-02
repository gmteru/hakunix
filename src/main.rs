#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hakunix_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hakunix_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hakunix OS (ver. 0.1.0-alpha)");

    hakunix_os::init();

    #[cfg(test)]
    test_main();

    hakunix_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hakunix_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hakunix_os::test_panic_handler(info)
}
