#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use limine::request::{FramebufferRequest, HhdmRequest};

mod font_render;
use font_render::FramebufferWriter;

mod idt;
mod keyboard;
mod pic;
mod serial_io;
mod typer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_io::serial_print("PANIC\n");
    loop {}
}

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_io::serial_init();
    serial_io::serial_print("serial initialised");
    // in main.rs, before idt::init()
    let cs: u16;
    unsafe {
        core::arch::asm!("mov {0:x}, cs", out(reg) cs);
    }
    serial_io::serial_print("cs: ");
    serial_io::serial_print_num(cs as usize);
    serial_io::serial_print("\n");

    idt::init();
    pic::init();
    unsafe {
        core::arch::asm!("sti");
    }
    let fb_response = FRAMEBUFFER_REQUEST.response().unwrap();
    let framebuffer = fb_response.framebuffers().first().unwrap();

    let fb_ptr = framebuffer.address() as *mut u32;
    let stride = (framebuffer.pitch / 4) as usize;
    let height = framebuffer.height as usize;

    let mut writer = FramebufferWriter::new(fb_ptr, stride, 10, 10);
    writer.clear_screen(stride, height);
    writer.set_color(0x0000FFFF);
    writer.println("Welcome to miOS!");
    writer.set_color(0x00FFFFFF);
    writer.println("ts is so peak");
    font_render::print_font_debug();
    unsafe {
        typer::WRITER = Some(writer);
    }
    loop {}
}
