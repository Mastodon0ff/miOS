#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let fb_response = FRAMEBUFFER_REQUEST.response().unwrap();
    let framebuffer = fb_response.framebuffers().first().unwrap();

    let fb_ptr = framebuffer.address() as *mut u32;
    unsafe {
        fb_ptr.write_volatile(0x00FF0000);
    }
    loop {}
}
