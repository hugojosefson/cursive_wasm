use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(remote = "cursive_core::theme::Effect")]
#[wasm_bindgen]
pub enum Effect {
    Simple,
    Reverse,
    Dim,
    Bold,
    Italic,
    Strikethrough,
    Underline,
    Blink,
}

impl From<Effect> for cursive_core::theme::Effect {
    fn from(effect: Effect) -> Self {
        match effect {
            Effect::Simple => cursive_core::theme::Effect::Simple,
            Effect::Reverse => cursive_core::theme::Effect::Reverse,
            Effect::Dim => cursive_core::theme::Effect::Dim,
            Effect::Bold => cursive_core::theme::Effect::Bold,
            Effect::Italic => cursive_core::theme::Effect::Italic,
            Effect::Strikethrough => cursive_core::theme::Effect::Strikethrough,
            Effect::Underline => cursive_core::theme::Effect::Underline,
            Effect::Blink => cursive_core::theme::Effect::Blink,
        }
    }
}

impl From<cursive_core::theme::Effect> for Effect {
    fn from(effect: cursive_core::theme::Effect) -> Self {
        match effect {
            cursive_core::theme::Effect::Simple => Effect::Simple,
            cursive_core::theme::Effect::Reverse => Effect::Reverse,
            cursive_core::theme::Effect::Dim => Effect::Dim,
            cursive_core::theme::Effect::Bold => Effect::Bold,
            cursive_core::theme::Effect::Italic => Effect::Italic,
            cursive_core::theme::Effect::Strikethrough => Effect::Strikethrough,
            cursive_core::theme::Effect::Underline => Effect::Underline,
            cursive_core::theme::Effect::Blink => Effect::Blink,
        }
    }
}
