use core::arch::asm;

static SHIFT_HELD: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

fn inb(port: u16) -> u8 {
    let val: u8;
    unsafe {
        asm!("in al, dx", out("al") val, in("dx") port);
    }
    val
}

const SCANCODE_MAP: [u8; 58] = [
    0, 0, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=',
    0, // backspace
    b'\t', b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',
    0, // left ctrl
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0, // left shift
    b'#', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0, // right shift
    b'*', 0, b' ',
];

const SCANCODE_MAP_SHIFT: [u8; 58] = [
    0, 0, b'!', b'"', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 0, b'\t', b'Q',
    b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', b'\n', 0, b'A', b'S', b'D',
    b'F', b'G', b'H', b'J', b'K', b'L', b':', b'@', b'~', 0, b'|', b'Z', b'X', b'C', b'V', b'B',
    b'N', b'M', b'<', b'>', b'?', 0, b'*', 0, b' ',
];

pub extern "x86-interrupt" fn keyboard_handler(_frame: &mut InterruptFrame) {
    let scancode = inb(0x60);

    if scancode & 0x80 != 0 {
        // key release
        let released = scancode & 0x7F; // strip bit 7 to get the base scancode
        if released == 42 || released == 54 {
            SHIFT_HELD.store(false, core::sync::atomic::Ordering::Relaxed);
        }
        crate::pic::eoi();
        return;
    }

    // key press
    if scancode == 42 || scancode == 54 {
        SHIFT_HELD.store(true, core::sync::atomic::Ordering::Relaxed);
        crate::pic::eoi();
        return;
    }

    if scancode == 14 {
        unsafe {
            if let Some(w) = &mut *(&raw mut crate::typer::WRITER) {
                w.backspace();
            }
        }
        crate::pic::eoi();
        return;
    }

    if (scancode as usize) < SCANCODE_MAP.len() {
        let shift = SHIFT_HELD.load(core::sync::atomic::Ordering::Relaxed);
        let c = if shift {
            SCANCODE_MAP_SHIFT[scancode as usize]
        } else {
            SCANCODE_MAP[scancode as usize]
        };
        if c != 0 {
            unsafe {
                if let Some(w) = &mut *(&raw mut crate::typer::WRITER) {
                    w.print_char(c as char);
                }
            }
        }
    }
    crate::pic::eoi();
}

#[repr(C)]
pub struct InterruptFrame {
    ip: u64,
    cs: u64,
    flags: u64,
    sp: u64,
    ss: u64,
}
