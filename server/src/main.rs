extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate itertools;

use std::path::Path;
use std::net::{ToSocketAddrs};

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;

use itertools::Itertools;

const SERVER_PORT: u16 = 8080;

fn wasm_project(path: &Path) -> Mount {
    let mut mount = Mount::new();
    mount.mount("/index.html", Static::new(path.join("static/")));
    mount.mount("/target", Static::new(path.join("target/wasm32-unknown-webassembly/debug/")));
    mount.mount("/runtime", Static::new(path.join("node_modules/webassembly/dist/")));
    mount
}

fn main() {
    let mut mount = Mount::new();
    mount.mount("/hello-world", wasm_project(Path::new("../hello-world/")));

    let address = ("localhost", SERVER_PORT);
    println!("Server listening on {}", address.to_socket_addrs().unwrap().join(", "));

    Iron::new(mount).http(address).unwrap();
}
