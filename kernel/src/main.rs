#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kernel::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};



entry_point!(kernel_main);  

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!("cnm");

    kernel::init();

    use kernel::arch::memory::translate_addr;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::PageTable;
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

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