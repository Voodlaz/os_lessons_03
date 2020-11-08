#![no_std]
#![no_main]

use x86_64::VirtAddr;
extern crate rlibc;
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

extern crate alloc;

use alloc::boxed::Box;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os::heap;
    use os::memory::{self, BootInfoFrameAllocator};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    heap::init_heap(&mut mapper, &mut frame_allocator).expect("NO");

    let heap_value = Box::new(41);

    os::vga_driver::writer("jml");
    os::hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
