#![no_std]
#![no_main]
#![feature(ascii_char)]

mod tty;
mod vga;

extern crate alloc;

use talc::{ClaimOnOom, Span, Talc, Talck};
use tty::terminal_initialize;
use bootloader_api::{entry_point, BootInfo};

static mut ARENA: [u8; 10000] = [0; 10000];

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> =
	Talc::new(unsafe { ClaimOnOom::new(Span::from_array((&raw const ARENA).cast_mut())) }).lock();

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
	unsafe {
		terminal_initialize();
	}

	println!("Hello, world!");

	loop {}
}

#[panic_handler]
const fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}
