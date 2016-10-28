#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate vga;
extern crate rlibc;

use core::fmt::Write;

#[cfg(not(test))]
pub mod panic;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    unsafe {
      let slice = core::slice::from_raw_parts_mut(0xb8400 as *mut u8, 4000);
      let mut vga = vga::Vga::new(slice);

      vga.write_str("hello rust belt rust!");
      vga.flush();
    };

    loop { }
}
