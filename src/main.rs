#![no_std] // Don't use the Rust standard library (and by extension libc)
#![no_main] // Disable Rust-level entry points
#![feature(custom_test_frameworks)] // Enable custom test frameworks feature
#![test_runner(os::test_runner)] // Configure cargo test to use our test runner
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;


// Ensure the Rust compiler outputs a function named "_start" that the linker can use
#[no_mangle]
// extern "C" tells the Rust compiler to using the C calling convention for the function (the Rust one is undefined)
// The return type "!" specifies the function never returns
pub extern "C" fn _start() -> !{
    println!("Hello World{}", "!");

    os::init();
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // trigger a stack overflow
    // stack_overflow();
    // x86_64::instructions::interrupts::int3();
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    #[cfg(test)]
    test_main();

    // panic!("Some panic message");
    println!("It did not crash!");

    os::hlt_loop(); // Call the HLT instruction endlessly (the CPU will still wake for interrupts)
}

// Define custom panic handler since the std one isn't available
// Cargo.toml configures Rust to abort on panic instead of using stack unwinding
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info);
}