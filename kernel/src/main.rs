#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kernel::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {

    println!("cnm");

    kernel::init();
    let ptr = 0xdeadbeaf as *mut u32;
    unsafe { *ptr = 42; }


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
    kernel::test_panic_handler(info)
}
//0x7ea7  load_next_kernel_block_from_disk