#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;

pub mod vga_driver;
pub mod heap;
pub mod memory;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("alloc error! {:?}", layout)
}
