#![feature(extern_types)]

mod utils;

use crate::backends::wasm::CursiveBackend;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

// main function
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod backends;

#[wasm_bindgen(js_name = "doSomethingWithMyBackend")]
pub fn do_something_with_my_backend(backend: &CursiveBackend) {
    backend.set_raw(true);
    backend.print("Hello from Rust!");
    backend.set_raw(false);
}
