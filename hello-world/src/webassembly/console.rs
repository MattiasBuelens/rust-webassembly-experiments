extern "C" {
    pub fn console_log(message: *const u8) -> ();
    pub fn console_warn(message: *const u8) -> ();
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