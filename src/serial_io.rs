pub fn serial_init() {
    unsafe {
        // disable interrupts
        core::arch::asm!("out dx, al", in("dx") 0x3F9u16, in("al") 0u8);
        // enable DLAB to set baud rate
        core::arch::asm!("out dx, al", in("dx") 0x3FBu16, in("al") 0x80u8);
        // set baud rate to 38400 (divisor 3)
        core::arch::asm!("out dx, al", in("dx") 0x3F8u16, in("al") 3u8);
        core::arch::asm!("out dx, al", in("dx") 0x3F9u16, in("al") 0u8);
        // 8 bits, no parity, one stop bit
        core::arch::asm!("out dx, al", in("dx") 0x3FBu16, in("al") 0x03u8);
        // enable FIFO
        core::arch::asm!("out dx, al", in("dx") 0x3FAu16, in("al") 0xC7u8);
    }
}

pub fn serial_write_byte(byte: u8) {
    unsafe {
        core::arch::asm!(
        "out dx, al",
        in("dx") 0x3F8u16,
        in("al") byte,
        );
    }
}

pub fn serial_print(s: &str) {
    for b in s.bytes() {
        serial_write_byte(b);
    }
}
pub fn serial_print_num(n: usize) {
    if n == 0 {
        serial_write_byte(b'0');
        return;
    }
    let mut buffer = [0u8; 20];
    let mut i = 20;
    let mut n = n;
    while n > 0 {
        i -= 1;
        buffer[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    for &b in &buffer[i..] {
        serial_write_byte(b);
    }
}
