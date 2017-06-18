mod webassembly;

use webassembly::console;
use webassembly::math;

#[no_mangle]
pub extern fn main() {
    console::log("Hello, WebAssembly!");

    let x = math::sqrt(17.0);
    if x == 4.0 {
        console::log("Yay!");
    } else {
        console::warn("Alert!");
    }
}