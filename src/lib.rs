#![feature(extern_types)]

mod utils;

use crate::backends::wasm::CursiveBackend;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod backends;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(s: &str);
}

#[wasm_bindgen(js_name = "doSomethingWithMyBackend")]
pub fn do_something_with_my_backend(backend: &CursiveBackend) {
    backend.set_raw(true);
    backend.print("Hello from Rust!");
    backend.set_raw(false);
}
