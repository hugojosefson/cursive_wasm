use crate::utils::set_panic_hook;
use wasm_bindgen::convert::{IntoWasmAbi, WasmAbi};
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;

/**
 * Call this function from JavaScript with a JavaScript implementation of `CursiveBackend`, to get
 * back an instance of `Cursive` that uses your backend.
 */
#[wasm_bindgen(js_name = "wasmCursive")]
pub fn wasm_cursive(backend: CursiveBackend) -> WasmCursive {
    set_panic_hook();
    WasmCursive::new(backend)
}

/**
* This is what is printed into the `.d.ts` file.
* Make sure it matches what is declared in the `extern "C"` block below!
*/
#[wasm_bindgen(typescript_custom_section)]
const CURSIVE_BACKEND: &'static str = r#"
interface CursiveBackend {
    print(s: string): void;
    setRaw(raw: boolean): void;
}
"#;

#[wasm_bindgen]
extern "C" {
    /**
     * The Rust type `CursiveBackend`, for use by Rust code. Implementations of its methods are supplied from JavaScript.
     */
    #[wasm_bindgen(typescript_type = "CursiveBackend")]
    pub type CursiveBackend;

    /**
     * Member of the `CursiveBackend` type.
     */
    #[wasm_bindgen(structural, method, js_name = "setRaw")]
    pub fn set_raw(this: &CursiveBackend, raw: bool);

    /**
     * Member of the `CursiveBackend` type.
     */
    #[wasm_bindgen(structural, method, js_name = "print")]
    pub fn print(this: &CursiveBackend, s: &str);
}

pub struct WasmCursive {
    js_backend: CursiveBackend,
}

/**
 * This represents the `Cursive` type that JavaScript code will use.
 */
#[wasm_bindgen]
impl WasmCursive {
    #[wasm_bindgen(constructor)]
    pub fn new(js_backend: CursiveBackend) -> Self {
        Self { js_backend }
    }
}

impl WasmDescribe for WasmCursive {
    fn describe() {
        <CursiveBackend as WasmDescribe>::describe();
    }
}

impl IntoWasmAbi for WasmCursive {
    type Abi = Self;
    fn into_abi(self) -> Self::Abi {
        self
    }
}

unsafe impl WasmAbi for WasmCursive {}
