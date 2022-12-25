// This module will be compiled into the main WASM module that backend implementors will import.
#![feature(extern_types)]

mod utils;

use crate::backends::wasm::CursiveBackend;
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod backends;

/**
 * This function is exported from the main WASM module.
 */
#[wasm_bindgen(js_name = "doSomethingWithMyBackend")]
pub fn do_something_with_my_backend(backend: &CursiveBackend) {
    set_panic_hook();
    backend.set_raw(true);
    backend.print("Hello from Rust!");
    backend.set_raw(false);
}
