use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use super::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = create_idt();
}

const APIC_BASE_VECTOR: u8 = 0x20;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = 0xfe,
    Error = 0x31,
    Spurious = 0xff,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init() {
    // Load the IDT.
    IDT.load();

    // We're ready to be interrupted!
    x86_64::instructions::interrupts::enable();
}

fn create_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    // unsafe {
    //     // SAFETY: We configure the stack in super::gdt::init(), and we don't reuse this IST index anywhere in this IDT.
    //     idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    // }
    idt
}

extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: InterruptStackFrame) {
    log::info!("Timer tick");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log::info!("BREAKPOINT: {:#?}", stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("STACK SEGMENT FAULT {:?}: {:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn gp_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("GPFAULT at 0x{:#X}", stack_frame.instruction_pointer);
    loop{}
    //panic!("GENERAL PROTECTION FAULT {:?}: {:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    panic!("PAGE FAULT {:?}: {:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("DOUBLE FAULT: {:#?}", stack_frame);
}
