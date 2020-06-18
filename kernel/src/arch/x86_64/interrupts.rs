use super::gdt;
use crate::{println,hlt_loop};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};



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
        idt.page_fault.set_handler_fn(page_fault_handler);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }
        idt
    };
}


//TODO: creat own IDT and x86-interrupt call comventsion
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKOPINT\n{:#?}", stack_frame);
}


extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}


pub fn init() {
    IDT.load();
}

#[cfg(test)]
use crate::serial_println;

#[test_case]
fn test_breakpoint_exception() {
    serial_println!("test breakpoint exception....");
    x86_64::instructions::interrupts::int3();
    serial_println!("[OK]");
}