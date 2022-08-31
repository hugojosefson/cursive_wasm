use cursive_core::backend;
use cursive_core::event::Event;
use cursive_core::theme::{Color, ColorPair, Effect};
use cursive_core::Vec2;
use wasm_bindgen::prelude::*;

pub struct Backend {}

impl Backend {
    pub fn init() -> Box<dyn backend::Backend> {
        let c = Backend {};
        Box::new(c)
    }

    fn parse_next(&mut self) -> Option<Event> {
        None
    }
}

impl backend::Backend for Backend {
    fn poll_event(&mut self) -> Option<Event> {
        self.parse_next()
    }

    fn set_title(&mut self, _title: String) {}

    fn refresh(&mut self) {}

    fn has_colors(&self) -> bool {
        true
    }

    fn screen_size(&self) -> Vec2 {
        (80, 24).into()
    }

    fn print_at(&self, _pos: Vec2, _text: &str) {}

    fn clear(&self, _color: Color) {}

    fn set_color(&self, color: ColorPair) -> ColorPair {
        color
    }

    fn set_effect(&self, _effect: Effect) {}

    fn unset_effect(&self, _effect: Effect) {}

    fn name(&self) -> &str {
        "wasm"
    }
}
