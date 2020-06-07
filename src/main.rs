#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rdays::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rdays::println;
use core::panic::PanicInfo;

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    
    println!("cnm");
    #[cfg(test)]
    test_main();

    loop {}

}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Error: {}",info);
    loop{}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rdays::test_panic_handler(info)
}
