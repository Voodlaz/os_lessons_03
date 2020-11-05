#![no_std]
#![no_main]

extern crate rlibc;
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    os::vga_driver::writer("jml");
    os::hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
