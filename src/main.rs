

#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;
mod vga_buffer;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

#[unsafe(no_mangle)] // 不重整函数名
pub extern "C" fn _start() -> ! {
    println!("ciallo");
    panic!("Some panic message");
    loop {}
}