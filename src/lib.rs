extern crate cfg_if;
use cfg_if::cfg_if;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod math;
pub mod mat4;
pub mod vec2;
pub mod vec3;
pub mod vec4;
// pub use self::vec2::Vec2;

#[wasm_bindgen(start)]
pub fn main() {
    cfg_if! {
        if #[cfg(feature = "web")] {

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = console)]
                fn log(s: &str);
            }

            #[wasm_bindgen]
            pub fn test(){
                log("start");
            }
        }
    }
}
