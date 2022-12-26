use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[wasm_bindgen]
pub enum Event {
    /// Event fired when the window is resized.
    WindowResize,

    /// Event fired when the view is about to lose focus.
    FocusLost,

    /// Event fired regularly when a auto-refresh is set.
    Refresh,

    Exit,
}

impl From<Event> for cursive_core::event::Event {
    fn from(event: Event) -> Self {
        match event {
            Event::WindowResize => cursive_core::event::Event::WindowResize,
            Event::FocusLost => cursive_core::event::Event::FocusLost,
            Event::Refresh => cursive_core::event::Event::Refresh,
            Event::Exit => cursive_core::event::Event::Exit,
        }
    }
}

impl From<cursive_core::event::Event> for Event {
    fn from(event: cursive_core::event::Event) -> Self {
        match event {
            cursive_core::event::Event::WindowResize => Event::WindowResize,
            cursive_core::event::Event::FocusLost => Event::FocusLost,
            cursive_core::event::Event::Refresh => Event::Refresh,
            cursive_core::event::Event::Exit => Event::Exit,
            _ => panic!("Event not supported"),
        }
    }
}
