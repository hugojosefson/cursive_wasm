mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod cursive_runnable;

pub use cursive_runnable::CursiveRunnable;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, cursive-wasm!");
}

/// Creates a new Cursive root using a dummy backend.
///
/// Nothing will be output. This is mostly here for tests.
pub fn dummy() -> CursiveRunnable {
    CursiveRunnable::dummy()
}
