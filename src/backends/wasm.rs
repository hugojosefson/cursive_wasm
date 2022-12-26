use crate::data::color::Color;
use crate::data::color_pair::ColorPair;
use crate::data::effect::Effect;
use crate::data::event::Event;
use crate::data::vec2::Vec2;
use crate::utils::set_panic_hook;
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
    pollEvent(): Event;
    hasColors(): boolean;
    screenSize(): Vec2;
    setTitle(title: string): void;
    refresh(): void;
    printAt(pos: Vec2, s: string): void;
    clear(color: Color): void;
    setColor(color: ColorPair): ColorPair;
    setEffect(effect: Effect): void;
    unsetEffect(effect: Effect): void;
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
    pub fn poll_event(this: &CursiveBackend) -> Option<Event>;

    #[wasm_bindgen(method, js_name = "hasColors")]
    pub fn has_colors(this: &CursiveBackend) -> bool;

    #[wasm_bindgen(method, js_name = "screenSize")]
    pub fn screen_size(this: &CursiveBackend) -> Vec2;

    #[wasm_bindgen(method, js_name = "setTitle")]
    pub fn set_title(this: &CursiveBackend, title: &str);

    #[wasm_bindgen(method, js_name = "refresh")]
    pub fn refresh(this: &CursiveBackend);

    #[wasm_bindgen(method, js_name = "printAt")]
    pub fn print_at(this: &CursiveBackend, pos: Vec2, text: &str);

    #[wasm_bindgen(method, js_name = "clear")]
    pub fn clear(this: &CursiveBackend, color: Color);

    #[wasm_bindgen(method, js_name = "setColor")]
    #[allow(non_snake_case)]
    pub fn set_color(this: &CursiveBackend, colorPair: ColorPair) -> ColorPair;

    #[wasm_bindgen(method, js_name = "setEffect")]
    pub fn set_effect(this: &CursiveBackend, effect: Effect);

    #[wasm_bindgen(method, js_name = "unsetEffect")]
    pub fn unset_effect(this: &CursiveBackend, effect: Effect);

}

/**
not all trait items implemented, missing: `poll_event`, `set_title`, `refresh`, `has_colors`, `screen_size`, `print_at`, `clear`, `set_color`, `set_effect`, `unset_effect` [E0046]
missing `poll_event`, `set_title`, `refresh`, `has_colors`, `screen_size`, `print_at`, `clear`, `set_color`, `set_effect`, `unset_effect` in implementation
Help: implement the missing item: `fn poll_event(&mut self) -> std::option::Option<cursive_core::event::Event> { todo!() }`
Help: implement the missing item: `fn set_title(&mut self, _: std::string::String) { todo!() }`
Help: implement the missing item: `fn refresh(&mut self) { todo!() }`
Help: implement the missing item: `fn has_colors(&self) -> bool { todo!() }`
Help: implement the missing item: `fn screen_size(&self) -> cursive_core::XY<usize> { todo!() }`
Help: implement the missing item: `fn print_at(&self, _: cursive_core::XY<usize>, _: &str) { todo!() }`
Help: implement the missing item: `fn clear(&self, _: cursive_core::theme::Color) { todo!() }`
Help: implement the missing item: `fn set_color(&self, _: cursive_core::theme::ColorPair) -> cursive_core::theme::ColorPair { todo!() }`
Help: implement the missing item: `fn set_effect(&self, _: cursive_core::theme::Effect) { todo!() }`
Help: implement the missing item: `fn unset_effect(&self, _: cursive_core::theme::Effect) { todo!() }`
*/

#[wasm_bindgen]
pub struct Cursive {
    cursive_runtime: cursive_core::Cursive,
}

#[wasm_bindgen]
pub struct CursiveBackendWrapper {
    backend: CursiveBackend,
}

/**
 * This represents the `Cursive` type that JavaScript code will use.
 */
#[wasm_bindgen]
impl Cursive {
    #[wasm_bindgen(js_name = "letsGo")]
    pub fn lets_go(backend: CursiveBackend) -> Self {
        set_panic_hook();
        let mut cursive_runtime = cursive_core::Cursive::new();
        let cursive_backend_wrapper = CursiveBackendWrapper { backend };
        let boxed_backend_wrapper: Box<dyn cursive_core::backend::Backend> =
            Box::new(cursive_backend_wrapper);
        let lambda = || -> Box<dyn cursive_core::backend::Backend> { boxed_backend_wrapper };
        cursive_runtime.run_with(lambda);
        Cursive { cursive_runtime }
    }
}

/**
 * cursive_core will call here, and we should forward the call to the JavaScript implementation of CursiveBackend.
 */
impl cursive_core::backend::Backend for CursiveBackendWrapper {
    fn has_colors(&self) -> bool {
        self.backend.has_colors()
    }

    fn screen_size(&self) -> cursive_core::Vec2 {
        self.backend.screen_size().into()
    }

    fn poll_event(&mut self) -> Option<cursive_core::event::Event> {
        self.backend.poll_event().map(|e| e.into())
    }

    fn set_title(&mut self, title: String) {
        self.backend.set_title(&title);
    }

    fn refresh(&mut self) {
        self.backend.refresh();
    }

    fn print_at(&self, pos: cursive_core::Vec2, text: &str) {
        self.backend.print_at(pos.into(), text);
    }

    fn clear(&self, color: cursive_core::theme::Color) {
        self.backend.clear(color.into());
    }

    fn set_color(&self, color: cursive_core::theme::ColorPair) -> cursive_core::theme::ColorPair {
        self.backend.set_color(color.into()).into()
    }

    fn set_effect(&self, effect: cursive_core::theme::Effect) {
        self.backend.set_effect(effect.into());
    }

    fn unset_effect(&self, effect: cursive_core::theme::Effect) {
        self.backend.unset_effect(effect.into());
    }
}
