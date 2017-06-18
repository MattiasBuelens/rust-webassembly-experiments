extern crate std;

use std::os::raw::{c_char};

extern "C" {
    pub fn console_log(message: *const c_char) -> ();
    pub fn console_warn(message: *const c_char) -> ();
}

pub fn log(message: &str) -> () {
    unsafe {
        console_log(message.as_ptr());
    }
}

pub fn warn(message: &str) -> () {
    unsafe {
        console_warn(message.as_ptr());
    }
}