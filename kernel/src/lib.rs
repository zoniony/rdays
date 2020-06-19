#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)] 
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)] //https://doc.rust-lang.org/nightly/unstable-book/language-features/custom-test-frameworks.html
#![reexport_test_harness_main = "test_main"] //https://github.com/rust-lang/rust/blob/master/src/librustc_builtin_macros/test_harness.rs

pub mod util;

#[path = "arch/x86_64/mod.rs"]
pub mod arch;

pub use crate::arch::*;
use core::panic::PanicInfo;

// auto exit qmeu 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed  = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

//entry
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {    
    serial_println!("Begin Test:");
    test_main();
    loop {} //remove the line will return ()
}


//custom test frameworks handler runners
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}


//handler Failed case
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

pub fn init() {
    gdt::init();
    interrupts::init();
}

