#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VgaColor {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGrey = 7,
	DarkGrey = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	LightMagenta = 13,
	LightBrown = 14,
	White = 15,
}

#[inline]
pub const fn vga_entry_color(foreground: VgaColor, background: VgaColor) -> u8 {
	foreground as u8 | (background as u8) << 4_u8
}

#[inline]
pub fn vga_entry(uc: char, color: u8) -> u16 {
	(uc.as_ascii().unwrap().to_u8() | color.wrapping_shl(8_u32)).into()
}
