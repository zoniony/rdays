use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};

//Box convert 'static refernce?

/*
//why static mut occur date race?
static mut IDT: InterruptDescriptorTable =  InterruptDescriptorTable::new();
pub fn init_idt() {
    unsafe{
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.load();
    }
}
*/

//lazy initialization idt 
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}


pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKOPINT\n{:#?}", stack_frame);
}

#[cfg(test)]
use crate::serial_println;

#[test_case]
fn test_breakpoint_exception() {
    serial_println!("test breakpoint exception....");
    x86_64::instructions::interrupts::int3();
    serial_println!("[OK]");
}