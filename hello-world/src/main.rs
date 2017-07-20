#![feature(lang_items, intrinsics)]
#![feature(start)]
#![no_std]
#![no_main]

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

    0
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