@echo off

set outdir=./target/wasm32-unknown-webassembly/debug
set wasmlib=./node_modules/webassembly/lib/webassembly.bc
set wasmtools=./node_modules/webassembly/tools/bin/win32-x64

if not exist "%outdir%" mkdir "%outdir%"

rustc --target wasm32-unknown-emscripten --crate-type lib --emit llvm-bc "src/lib.rs" -o "%outdir%/lib.bc"
"%wasmtools%/llvm-link" "%outdir%/lib.bc" "%wasmlib%" -only-needed -o "%outdir%/lib-linked.bc"
"%wasmtools%/llc" "%outdir%/lib-linked.bc" -march=wasm32 -filetype=asm -asm-verbose=false -thread-model=single -data-sections -function-sections -o "%outdir%/lib.s"
"%wasmtools%/s2wasm" "%outdir%/lib.s" --import-memory --allocate-stack 10000 -o "%outdir%/lib.wasm"
"%wasmtools%/wasm-opt" "%outdir%/lib.wasm" -o "%outdir%/lib-opt.wasm"