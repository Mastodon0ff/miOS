use crate::font_render::FramebufferWriter;

pub struct Spinlock {
    locked: core::sync::atomic::AtomicBool,
}

impl Spinlock {
    pub const fn new() -> Self{
        Self { locked: core::sync::atomic::AtomicBool::new(false) }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, core::sync::atomic::Ordering::Acquire) {}
    }

    pub fn unlock(&self) {
        self.locked.store(false, core::sync::atomic::Ordering::Release);
    }
}

pub static WRITER_LOCK: Spinlock = Spinlock::new();
pub static mut WRITER: Option<FramebufferWriter> = None;

