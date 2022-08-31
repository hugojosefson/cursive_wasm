#![feature(extern_types)]

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod backends;

mod cursive_ext;
mod cursive_runnable;

pub use cursive_ext::CursiveExt;
pub use cursive_runnable::CursiveRunnable;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(s: &str);
}

#[wasm_bindgen]
pub struct WasmCursive {
    runnable: CursiveRunnable,
}

#[wasm_bindgen]
impl WasmCursive {
    pub fn new() -> Self {
        let runnable = CursiveRunnable::wasm();
        Self { runnable }
    }
    #[wasm_bindgen]
    pub fn run(&mut self) {
        self.runnable.run()
    }
}

impl Default for WasmCursive {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_name = "wasmCursive")]
pub fn wasm_cursive() -> WasmCursive {
    WasmCursive::new()
}

#[wasm_bindgen(typescript_custom_section)]
const CURSIVE_BACKEND: &'static str = r#"
interface CursiveBackend {
    print(s: string): void;
    setRaw(raw: boolean): void;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "CursiveBackend")]
    pub type CursiveBackend;

    #[wasm_bindgen(structural, method)]
    pub fn print(this: &CursiveBackend, s: &str);

    #[wasm_bindgen(structural, method, js_name = "setRaw")]
    pub fn set_raw(this: &CursiveBackend, raw: bool);
}

#[wasm_bindgen(js_name = "doSomethingWithMyBackend")]
pub fn do_something_with_my_backend(backend: &CursiveBackend) {
    backend.set_raw(true);
    backend.print("Hello from Rust!");
    backend.set_raw(false);
}
