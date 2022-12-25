use crate::data::color::Color;
use crate::data::color_pair::ColorPair;
use crate::data::effect::Effect;
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
    pollEvent(): string;
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
    pub fn poll_event(this: &CursiveBackend) -> String;

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
    pub fn check_my_screen_size(&self) -> Vec2 {
        self.backend.screen_size()
    }

    #[wasm_bindgen(js_name = "callMe")]
    pub fn call_me(&self) {
        self.backend.set_title("New title!");
        self.backend.refresh();
        self.backend.print_at(Vec2 { x: 0, y: 0 }, "Hello, world!");
        self.backend.clear(Color {
            r: 10,
            g: 20,
            b: 30,
        });
        self.backend.set_color(ColorPair {
            front: Color {
                r: 10,
                g: 20,
                b: 30,
            },
            back: Color {
                r: 40,
                g: 50,
                b: 60,
            },
        });
        self.backend
            .set_effect(cursive_core::theme::Effect::Bold.into());
        self.backend
            .unset_effect(cursive_core::theme::Effect::Simple.into());
    }
}

impl Drop for Cursive {
    fn drop(&mut self) {
        self.backend.print("Goodbye, world!");
    }
}

/**
 * cursive_core will call here, and we should forward the call to the JavaScript implementation of CursiveBackend.
 */
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
