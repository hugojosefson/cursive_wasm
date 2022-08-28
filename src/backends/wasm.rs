use crate::backend;
use crate::event::Event;
use crate::theme::{Color, ColorPair, Effect};
use crate::Vec2;

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
    fn name(&self) -> &str {
        "wasm"
    }

    fn set_title(&mut self, _title: String) {}

    fn set_color(&self, color: ColorPair) -> ColorPair {
        color
    }

    fn has_colors(&self) -> bool {
        true
    }

    fn screen_size(&self) -> Vec2 {
        (80, 24).into()
    }

    fn clear(&self, _color: Color) {}

    fn refresh(&mut self) {}

    fn print_at(&self, _pos: Vec2, _text: &str) {}

    fn poll_event(&mut self) -> Option<Event> {
        self.parse_next()
    }

    fn set_effect(&self, _effect: Effect) {}

    fn unset_effect(&self, _effect: Effect) {}
}
