#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_rust::println;
use bootloader::{BootInfo, entry_point};


//panic handler

//条件编译(非测试)
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os_rust::hlt_loop();
}

//条件编译(测试)
#[cfg(test)]
#[panic_handler]
#[warn(non_snake_case)]
fn panic(info: &PanicInfo) -> ! {
    os_rust::test_panic_handler(info)
}


entry_point!(kernel_main);

#[unsafe(no_mangle)] // 不重整函数名
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("ciallo");
    os_rust::init();
    // x86_64::instructions::interrupts::int3(); 

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();
    os_rust::hlt_loop();
}