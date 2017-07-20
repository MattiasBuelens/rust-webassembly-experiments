extern crate nickel;

use std::path::Path;

use nickel::{Nickel, Router, HttpRouter, StaticFilesHandler, Mountable};

const SERVER_PORT: u16 = 8080;

fn wasm_project(path: &Path) -> Router {
    let mut router = Router::new();
    router.get("/index.html", StaticFilesHandler::new(path.join("static/")));
    router.get("/**", StaticFilesHandler::new(path.join("target/wasm32-experimental-emscripten/debug/deps/")));
    router
}

fn main() {
    let mut server = Nickel::new();
    server.mount("/hello-world/", wasm_project(Path::new("../hello-world/")));
    server.listen(("localhost", SERVER_PORT)).unwrap();
}
