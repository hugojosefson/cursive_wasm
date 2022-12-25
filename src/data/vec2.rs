use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "cursive_core::Vec2")]
#[wasm_bindgen(inspectable)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl From<Vec2> for cursive_core::Vec2 {
    fn from(vec2: Vec2) -> Self {
        cursive_core::Vec2::new(vec2.x, vec2.y)
    }
}

impl From<cursive_core::Vec2> for Vec2 {
    fn from(vec2: cursive_core::Vec2) -> Self {
        Vec2 {
            x: vec2.x,
            y: vec2.y,
        }
    }
}

#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
