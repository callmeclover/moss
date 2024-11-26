use alloc::vec::Vec;
use core::fmt;

use once_cell::unsync::Lazy;

use crate::vga::{vga_entry, vga_entry_color, VgaColor};

pub const VGA_WIDTH: usize = 80_usize;
pub const VGA_HEIGHT: usize = 25_usize;
const VGA_MEMORY: Lazy<&mut [u16; VGA_WIDTH * VGA_HEIGHT]> =
	Lazy::new(|| unsafe { &mut *(0xB8000 as *mut [u16; VGA_WIDTH * VGA_HEIGHT]) });

static mut TERMINAL_ROW: usize = 0_usize;
static mut TERMINAL_COLUMN: usize = 0_usize;
static mut TERMINAL_COLOR: u8 = 0_u8;
static mut TERMINAL_BUFFER: &mut [u16; VGA_WIDTH * VGA_HEIGHT] =
	&mut [0_u16; VGA_WIDTH * VGA_HEIGHT];

pub unsafe fn terminal_initialize() {
	TERMINAL_ROW = 0_usize;
	TERMINAL_COLUMN = 0_usize;
	TERMINAL_COLOR = vga_entry_color(VgaColor::LightGrey, VgaColor::Black);
	*TERMINAL_BUFFER = *(*VGA_MEMORY);

	for y in 0..VGA_HEIGHT {
		for x in 0..VGA_WIDTH {
			TERMINAL_BUFFER[y * VGA_WIDTH + x] = vga_entry(' ', TERMINAL_COLOR);
		}
	}
}

unsafe fn terminal_setcolor(color: u8) {
	TERMINAL_COLOR = color;
}

unsafe fn terminal_putentryat(character: char, color: u8, x: usize, y: usize) {
	TERMINAL_BUFFER[y * VGA_WIDTH + x] = vga_entry(character, color);
}

unsafe fn terminal_putchar(character: char) {
	terminal_putentryat(character, TERMINAL_COLOR, TERMINAL_COLUMN, TERMINAL_ROW);
	TERMINAL_COLUMN += 1;
	if TERMINAL_COLUMN == VGA_WIDTH {
		TERMINAL_COLUMN = 0;
		TERMINAL_ROW += 1;
		if TERMINAL_ROW == VGA_HEIGHT {
			TERMINAL_ROW = 0;
		}
	}
}

unsafe fn terminal_write(data: &[char], size: usize) {
	for character in data.iter().take(size) {
		terminal_putchar(*character);
	}
}

unsafe fn terminal_writestring(data: &str) {
	terminal_write(data.chars().collect::<Vec<char>>().as_slice(), data.len());
}

struct MacroWriter {}

impl fmt::Write for MacroWriter {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		unsafe {
			terminal_writestring(s);
		}
		Ok(())
	}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	MacroWriter {}.write_fmt(args).unwrap();
}
