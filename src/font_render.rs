use core::str::Bytes;

//need to make the font display larger
pub static FONT: &[u8] = include_bytes!("../assets/cozette.psf");
const CHAR_W: usize = 8;
const CHAR_H: usize = 16;
const PSF2_HEADER_SIZE: usize = 32;

pub struct FramebufferWriter {
    ptr: *mut u32,
    stride: usize,
    cursor_x: usize,
    cursor_y: usize,
    color: u32,
}

fn glyph_index(c: char) -> usize {
    let num_glyphs = FONT[16] as usize
        | (FONT[17] as usize) << 8
        | (FONT[18] as usize) << 16
        | (FONT[19] as usize) << 24;
    let bytes_per_glyph = FONT[20] as usize
        | (FONT[21] as usize) << 8
        | (FONT[22] as usize) << 16
        | (FONT[23] as usize) << 24;
    let header_size = 32;

    let table_start = header_size + num_glyphs * bytes_per_glyph;
    let table = &FONT[table_start..];

    let target = c as u32;
    let mut glyph = 0usize;
    let mut i = 0usize;

    while i < table.len() {
        let byte = table[i];
        if byte == 0xFF {
            glyph += 1;
            i += 1;
            continue;
        }
        let (codepoint, len) = decode_utf8(table, i);
        if codepoint == target {
            return glyph;
        }
        i += len;
    }

    0
}

fn decode_utf8(bytes: &[u8], i: usize) -> (u32, usize) {
    let b = bytes[i];
    if b & 0x80 == 0 {
        (b as u32, 1)
    } else if b & 0xE0 == 0xC0 {
        let cp = ((b & 0x1F) as u32) << 6 | (bytes[i + 1] & 0x3F) as u32;
        (cp, 2)
    } else if b & 0xF0 == 0xE0 {
        let cp = ((b & 0x0F) as u32) << 12
            | ((bytes[i + 1] & 0x3F) as u32) << 6
            | (bytes[i + 2] & 0x3F) as u32;
        (cp, 3)
    } else {
        (0, 1)
    }
}

impl FramebufferWriter {
    pub fn new(ptr: *mut u32, stride: usize, start_x: usize, start_y: usize) -> Self {
        Self {
            ptr,
            stride,
            cursor_x: start_x,
            cursor_y: start_y,
            color: 0x00FFFFFF,
        }
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    pub fn clear_screen(&mut self, _width: usize, height: usize) {
        for i in 0..(self.stride * height) {
            unsafe {
                self.ptr.add(i).write_volatile(0x00000000);
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    fn psf2_char_size() -> (usize, usize) {
        let height = FONT[24] as usize
            | (FONT[25] as usize) << 8
            | (FONT[26] as usize) << 16
            | (FONT[27] as usize) << 24;
        let width = FONT[28] as usize
            | (FONT[29] as usize) << 8
            | (FONT[30] as usize) << 16
            | (FONT[31] as usize) << 24;
        (width, height)
    }

    fn putchar(&mut self, x: usize, y: usize, c: char) {
        let code = glyph_index(c);
        let (char_w, char_h) = Self::psf2_char_size();
        let bytes_per_row = (char_w + 7) / 8;
        let bytes_per_glyph = bytes_per_row * char_h;
        let glyph_offset = PSF2_HEADER_SIZE + code * bytes_per_glyph;

        for row in 0..char_h {
            for byte_in_row in 0..bytes_per_row {
                let byte = FONT[glyph_offset + row * bytes_per_row + byte_in_row];
                for bit in 0..8 {
                    let col = byte_in_row * 8 + bit;
                    if col >= char_w {
                        break;
                    }
                    if (byte >> (7 - bit)) & 1 == 1 {
                        let pixel = (y + row) * self.stride + (x + col);
                        unsafe {
                            self.ptr.add(pixel).write_volatile(self.color);
                        }
                    }
                }
            }
        }
    }
    pub fn print(&mut self, s: &str) {
        let (_char_w, char_h) = Self::psf2_char_size();
        for c in s.chars() {
            match c {
                '\n' => {
                    self.cursor_x = 0;
                    self.cursor_y += char_h;
                }
                _ => {
                    self.putchar(self.cursor_x, self.cursor_y, c);
                    self.cursor_x += CHAR_W;

                    if self.cursor_x + CHAR_W > self.stride {
                        self.cursor_x = 0;
                        self.cursor_y += char_h;
                    }
                }
            }
        }
    }

    pub fn println(&mut self, s: &str) {
        self.print(s);
        self.print("\n");
    }

    pub fn print_num(&mut self, n: usize) {
        if n == 0 {
            self.print("0");
            return;
        }
        let mut buffer = [0u8, 20];
        let mut i = 20;
        let mut n = n;
        while n > 0 {
            i -= 1;
            buffer[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }
        for &b in &buffer[i..] {
            self.putchar(self.cursor_x, self.cursor_y, b as char);
            self.cursor_x += 8;
        }
    }
    pub fn print_char(&mut self, c: char) {
        self.print(core::str::from_utf8(&[c as u8]).unwrap_or(""));
    }
    pub fn backspace(&mut self) {
        if self.cursor_x >= CHAR_W {
            self.cursor_x -= CHAR_W;
        } else {
            return;
        }
        for row in 0..CHAR_H {
            for col in 0..CHAR_W {
                let pixel = (self.cursor_y + row) * self.stride + (self.cursor_x + col);
                unsafe {
                    self.ptr.add(pixel).write_volatile(0x00000000);
                }
            }
        }
    }
}

pub fn print_font_debug() {
    crate::serial_io::serial_print("magic: ");
    crate::serial_io::serial_print_num(FONT[0] as usize);
    crate::serial_io::serial_print(" ");
    crate::serial_io::serial_print_num(FONT[1] as usize);
    crate::serial_io::serial_print(" ");
    crate::serial_io::serial_print_num(FONT[2] as usize);
    crate::serial_io::serial_print(" ");
    crate::serial_io::serial_print_num(FONT[3] as usize);
    crate::serial_io::serial_print("\n");

    // manual le bytes instead of try_into to avoid panics
    let bpg = FONT[20] as usize
        | (FONT[21] as usize) << 8
        | (FONT[22] as usize) << 16
        | (FONT[23] as usize) << 24;

    let h = FONT[24] as usize
        | (FONT[25] as usize) << 8
        | (FONT[26] as usize) << 16
        | (FONT[27] as usize) << 24;

    let w = FONT[28] as usize
        | (FONT[29] as usize) << 8
        | (FONT[30] as usize) << 16
        | (FONT[31] as usize) << 24;

    crate::serial_io::serial_print("bpg: ");
    crate::serial_io::serial_print_num(bpg);
    crate::serial_io::serial_print("\n");
    crate::serial_io::serial_print("h: ");
    crate::serial_io::serial_print_num(h);
    crate::serial_io::serial_print("\n");
    crate::serial_io::serial_print("w: ");
    crate::serial_io::serial_print_num(w);
    crate::serial_io::serial_print("\n");
    let flags = FONT[12] as usize | (FONT[13] as usize) << 8;
    crate::serial_io::serial_print("flags: ");
    crate::serial_io::serial_print_num(flags);
    crate::serial_io::serial_print("\n");

    let bpg = 26usize;
    let header_size = 32usize;
    let glyph_offset = header_size + 87 * bpg;
    crate::serial_io::serial_print("W glyph bytes:\n");
    for i in 0..bpg {
        crate::serial_io::serial_print_num(FONT[glyph_offset + i] as usize);
        crate::serial_io::serial_print(" ");
    }
    crate::serial_io::serial_print("\n");
}
