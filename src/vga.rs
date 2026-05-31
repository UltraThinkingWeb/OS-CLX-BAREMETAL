use core::fmt;
use spin::Mutex;
use volatile::Volatile;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct VgaWriter {
    row: usize,
    col: usize,
    color: u8,
}

impl VgaWriter {
    pub const fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            color: Self::make_color(Color::LightCyan, Color::Black),
        }
    }

    const fn make_color(fg: Color, bg: Color) -> u8 {
        (fg as u8) | ((bg as u8) << 4)
    }

    pub fn clear(&mut self) {
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                self.put_char_at(x, y, b' ');
            }
        }
        self.row = 0;
        self.col = 0;
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.newline();
            } else if byte.is_ascii() {
                self.put_char(byte);
            }
        }
    }

    fn put_char(&mut self, c: u8) {
        self.put_char_at(self.col, self.row, c);
        self.col += 1;
        if self.col >= VGA_WIDTH {
            self.newline();
        }
    }

    fn put_char_at(&mut self, x: usize, y: usize, c: u8) {
        let idx = (y * VGA_WIDTH + x) * 2;
        unsafe {
            VGA_BUFFER.add(idx).write_volatile(c);
            VGA_BUFFER.add(idx + 1).write_volatile(self.color);
        }
    }

    fn newline(&mut self) {
        self.col = 0;
        self.row += 1;
        if self.row >= VGA_HEIGHT {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        for y in 1..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let src = (y * VGA_WIDTH + x) * 2;
                let dst = ((y - 1) * VGA_WIDTH + x) * 2;
                unsafe {
                    let c = VGA_BUFFER.add(src).read_volatile();
                    let color = VGA_BUFFER.add(src + 1).read_volatile();
                    VGA_BUFFER.add(dst).write_volatile(c);
                    VGA_BUFFER.add(dst + 1).write_volatile(color);
                }
            }
        }

        let last_row = VGA_HEIGHT - 1;
        for x in 0..VGA_WIDTH {
            let idx = (last_row * VGA_WIDTH + x) * 2;
            unsafe {
                VGA_BUFFER.add(idx).write_volatile(b' ');
                VGA_BUFFER.add(idx + 1).write_volatile(self.color);
            }
        }
        self.row = last_row;
    }
}

// Global writer - Mutex pa lazy_static
static WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter::new());

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}
