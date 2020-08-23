#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mik_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use mik_os::{serial_print, serial_println};
use alloc::boxed::Box;
use alloc::vec::Vec;
use mik_os::allocator::HEAP_SIZE;

entry_point!(main);

#[test_case]
fn simple_allocation() {
    serial_print!("simple allocation...");
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
    serial_println!(" [ok]");
}

#[test_case]
fn large_vec() {
    serial_print!("large vec...");
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
    serial_println!(" [ok]");
}

#[test_case]
fn many_boxes() {
    serial_print!("many boxes...");
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    serial_println!(" [ok]");
}

#[test_case]
fn many_boxes_long_lived() {
    serial_print!("many boxes long lived ...");
    let long_lived = Box::new(1); // new
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1); // new
    serial_println!(" [ok]");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mik_os::test_panic_handler(info)
}

fn main(boot_info: &'static BootInfo) -> ! {
    use mik_os::allocator;
    use mik_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    mik_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    test_main();
    loop {}
}
