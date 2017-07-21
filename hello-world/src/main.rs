#![feature(lang_items, intrinsics)]
#![feature(global_allocator, allocator_api)]
#![feature(alloc, alloc_system)]
#![feature(start)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate alloc_system;
extern crate numtoa;

use alloc_system::System;
use alloc::boxed::Box;
use alloc::arc::Arc;
use numtoa::NumToA;

#[global_allocator]
static ALLOCATOR: System = System;

mod webassembly;

use webassembly::console;
use webassembly::math;

// Entry point for this program.
#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    console::log("Hello, WebAssembly!");;

    let x = math::sqrt(17.0);
    if x == 4.0 {
        console::log("Yay!");
    } else {
        console::warn("Alert!");
    }

    log_arc_boxed_str(Arc::new(Box::new("Heaps of fun!")));

    let mut buffer = [0u8; 10];
    print_decimal(1234, &mut buffer);
    print_binary(0b1010_0101, &mut buffer);

    0
}

pub fn log_arc_boxed_str(msg_arc : Arc<Box<&str>>) {
    let message = Arc::try_unwrap(msg_arc).unwrap();
    console::log(*message);
}

pub fn print_num(num: u32, base : u32, buffer: &mut [u8; 10]) {
    let index = num.numtoa(base, buffer);
    let str = core::str::from_utf8(&buffer[index..]).unwrap();
    console::log(&str);
}

pub fn print_binary(num: u32, buffer: &mut [u8; 10]) {
    print_num(num, 2, buffer)
}

pub fn print_decimal(num: u32, buffer: &mut [u8; 10]) {
    print_num(num, 10, buffer)
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! { loop {} }