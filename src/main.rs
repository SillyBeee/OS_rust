#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_rust::println;


//panic handler

//条件编译(非测试)
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//条件编译(测试)
#[cfg(test)]
#[panic_handler]
#[warn(non_snake_case)]
fn panic(info: &PanicInfo) -> ! {
    os_rust::test_panic_handler(info)
}




#[unsafe(no_mangle)] // 不重整函数名
pub extern "C" fn _start() -> ! {
    println!("ciallo");
    os_rust::init();
    x86_64::instructions::interrupts::int3(); 

    #[cfg(test)]
    test_main();
    loop {}
}