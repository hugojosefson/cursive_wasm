use crate::data::color::Color;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[wasm_bindgen(inspectable)]
pub struct ColorPair {
    pub front: Color,
    pub back: Color,
}

impl From<ColorPair> for cursive_core::theme::ColorPair {
    fn from(color_pair: ColorPair) -> Self {
        cursive_core::theme::ColorPair {
            front: color_pair.front.into(),
            back: color_pair.back.into(),
        }
    }
}

impl From<cursive_core::theme::ColorPair> for ColorPair {
    fn from(color_pair: cursive_core::theme::ColorPair) -> Self {
        ColorPair {
            front: color_pair.front.into(),
            back: color_pair.back.into(),
        }
    }
}

#[wasm_bindgen]
impl ColorPair {
    #[wasm_bindgen(constructor)]
    pub fn new(front: Color, back: Color) -> Self {
        Self { front, back }
    }
}
