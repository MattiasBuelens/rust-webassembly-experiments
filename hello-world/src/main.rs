mod webassembly;

use webassembly::console;
use webassembly::math;

pub fn main() {
    console::log("Hello, WebAssembly!");;

    let x = math::sqrt(17.0);
    if x == 4.0 {
        console::log("Yay!");
    } else {
        console::warn("Alert!");
    }
}