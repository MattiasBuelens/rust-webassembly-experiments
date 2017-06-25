extern "C" {
    pub fn console_log(message_ptr: *const u8, message_len: usize) -> ();
    pub fn console_warn(message_ptr: *const u8, message_len: usize) -> ();
}

pub fn log(message: &str) -> () {
    unsafe {
        console_log(message.as_ptr(), message.len());
    }
}

pub fn warn(message: &str) -> () {
    unsafe {
        console_warn(message.as_ptr(), message.len());
    }
}