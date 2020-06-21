#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use kernel::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};


entry_point!(kernel_main);  

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!("cnm");

    kernel::init();

    use kernel::arch::memory::{self, BootInfoFrameAllocator}; 
    use kernel::allocator;
    use x86_64::VirtAddr;
    use alloc::boxed::Box;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::heap_init(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let _x = Box::new(233);


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
