extern "C" {
    pub fn Math_sqrt(num: f64) -> f64;
}

pub fn sqrt(num: f64) -> f64 {
    unsafe {
        Math_sqrt(num)
    }
}