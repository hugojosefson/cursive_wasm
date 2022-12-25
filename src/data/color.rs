use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[wasm_bindgen(inspectable)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for cursive_core::theme::Color {
    fn from(color: Color) -> Self {
        cursive_core::theme::Color::Rgb(color.r, color.g, color.b)
    }
}

impl From<cursive_core::theme::Color> for Color {
    fn from(color: cursive_core::theme::Color) -> Self {
        match color {
            cursive_core::theme::Color::Rgb(r, g, b) => Color { r, g, b },
            _ => Color { r: 0, g: 0, b: 0 },
        }
    }
}

#[wasm_bindgen]
impl Color {
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
