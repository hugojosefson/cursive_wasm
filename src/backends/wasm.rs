use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub struct Cursive {
    backend: CursiveBackend,
}

/**
 * This represents the `Cursive` type that JavaScript code will use.
 */
#[wasm_bindgen]
impl Cursive {
    #[wasm_bindgen(constructor)]
    pub fn new(backend: CursiveBackend) -> Self {
        set_panic_hook();
        Self { backend }
    }

    #[wasm_bindgen(js_name = "printSomethingInRawMode")]
    pub fn print_something_in_raw_mode(&self) {
        self.backend.set_raw(true);
        self.backend.print("Hello, world!");
    }
}
