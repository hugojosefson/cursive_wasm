use wasm_bindgen::convert::{IntoWasmAbi, WasmAbi};
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "wasmCursive")]
pub fn wasm_cursive(js_backend: CursiveBackend) -> Backend {
    Backend::new(js_backend)
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

    #[wasm_bindgen(structural, method, js_name = "setRaw")]
    pub fn set_raw(this: &CursiveBackend, raw: bool);

    #[wasm_bindgen(structural, method, js_name = "print")]
    pub fn print(this: &CursiveBackend, s: &str);
}

pub struct Backend {
    js_backend: CursiveBackend,
}

#[wasm_bindgen]
impl Backend {
    pub fn new(js_backend: CursiveBackend) -> Self {
        Self { js_backend }
    }
}

impl WasmDescribe for Backend {
    fn describe() {
        <CursiveBackend as WasmDescribe>::describe();
    }
}

impl IntoWasmAbi for Backend {
    type Abi = Self;
    fn into_abi(self) -> Self::Abi {
        self
    }
}

unsafe impl WasmAbi for Backend {}
