extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod vec2;

// pub use self::vec2::Vec2;

#[wasm_bindgen]
extern "C" {
    fn alert(msg: &str);
}

#[wasm_bindgen]
pub fn take_number_by_value(x: f32) {
    alert(x.to_string().as_str());
}