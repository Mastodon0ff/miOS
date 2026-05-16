use core::arch::asm;

use crate::keyboard::keyboard_handler;

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry {
    offset_low: 0,
    selector: 0,
    ist: 0,
    flags: 0,
    offset_mid: 0,
    offset_high: 0,
    zero: 0,
}; 256];

#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    base: u64,
}

impl IdtEntry {
    fn new(handler: u64, selector: u16, flags: u8) -> Self {
        Self {
            offset_low: handler as u16,
            selector,
            ist: 0,
            flags,
            offset_mid: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            zero: 0,
        }
    }
}

pub fn init() {
    unsafe {
        IDT[33] = IdtEntry::new(keyboard_handler as *const () as u64, 40, 0x8E);
        let descriptor = IdtDescriptor {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: core::ptr::addr_of!(IDT) as u64,
        };
        asm!("lidt [{}]", in(reg) &descriptor, options(nostack));
    }
}
