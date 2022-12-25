use crate::utils::set_panic_hook;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/**
* This is what is printed into the `.d.ts` file.
* Make sure it matches what is declared in the `extern "C"` block below!
*/
#[wasm_bindgen(typescript_custom_section)]
const CURSIVE_BACKEND: &'static str = r#"
interface CursiveBackend {
    setRaw(raw: boolean): void;
    print(s: string): void;
    pollEvent(): string;
    hasColors(): boolean;
    screenSize(): SerializableVec2;
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
    #[wasm_bindgen(method, js_name = "setRaw")]
    pub fn set_raw(this: &CursiveBackend, raw: bool);

    /**
     * Member of the `CursiveBackend` type.
     */
    #[wasm_bindgen(method, js_name = "print")]
    pub fn print(this: &CursiveBackend, s: &str);

    #[wasm_bindgen(method, js_name = "pollEvent")]
    pub fn poll_event(this: &CursiveBackend) -> String;

    #[wasm_bindgen(method, js_name = "hasColors")]
    pub fn has_colors(this: &CursiveBackend) -> bool;

    #[wasm_bindgen(method, js_name = "screenSize")]
    pub fn screen_size(this: &CursiveBackend) -> SerializableVec2;
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

    #[wasm_bindgen(js_name = "checkMyColors")]
    pub fn check_my_colors(&self) -> bool {
        self.backend.has_colors()
    }

    #[wasm_bindgen(js_name = "checkMyScreenSize")]
    pub fn check_my_screen_size(&self) -> SerializableVec2 {
        self.backend.screen_size()
    }
}

impl Drop for Cursive {
    fn drop(&mut self) {
        self.backend.print("Goodbye, world!");
    }
}

impl cursive_core::backend::Backend for Cursive {
    fn has_colors(&self) -> bool {
        self.backend.has_colors()
    }

    fn screen_size(&self) -> cursive_core::Vec2 {
        self.backend.screen_size().into()
    }

    fn poll_event(&mut self) -> Option<cursive_core::event::Event> {
        let event = self.backend.poll_event();
        match event.as_str() {
            "quit" => Some(cursive_core::event::Event::Exit),
            "SOME_EVENT" => Some(cursive_core::event::Event::Refresh),
            _ => None,
        }
    }

    fn set_title(&mut self, _title: String) {
        unimplemented!()
    }

    fn refresh(&mut self) {
        unimplemented!()
    }

    fn print_at(&self, _pos: cursive_core::Vec2, _text: &str) {
        unimplemented!()
    }

    fn clear(&self, _color: cursive_core::theme::Color) {
        unimplemented!()
    }

    fn set_color(&self, _color: cursive_core::theme::ColorPair) -> cursive_core::theme::ColorPair {
        unimplemented!()
    }

    fn set_effect(&self, _effect: cursive_core::theme::Effect) {
        unimplemented!()
    }

    fn unset_effect(&self, _effect: cursive_core::theme::Effect) {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "cursive_core::Vec2")]
#[wasm_bindgen]
pub struct SerializableVec2 {
    pub x: usize,
    pub y: usize,
}

impl From<SerializableVec2> for cursive_core::Vec2 {
    fn from(vec2: SerializableVec2) -> Self {
        cursive_core::Vec2::new(vec2.x, vec2.y)
    }
}

impl From<cursive_core::Vec2> for SerializableVec2 {
    fn from(vec2: cursive_core::Vec2) -> Self {
        SerializableVec2 {
            x: vec2.x,
            y: vec2.y,
        }
    }
}

// SerializableVec2 constructor
#[wasm_bindgen]
impl SerializableVec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
