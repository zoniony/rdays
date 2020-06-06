#![no_std]
#![no_main]
#![feature(custom_test_frameworks)] 
#![test_runner(rdays::test_runner)] //https://doc.rust-lang.org/nightly/unstable-book/language-features/custom-test-frameworks.html
#![reexport_test_harness_main = "test_main"] //https://github.com/rust-lang/rust/blob/master/src/librustc_builtin_macros/test_harness.rs

use crate::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Error: {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rdays::test_panic_handler(info);    
}

