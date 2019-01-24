extern crate cfg_if;
use cfg_if::cfg_if;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod vec2;
pub mod vec3;
// pub use self::vec2::Vec2;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

cfg_if!{
    if #[cfg(feature = "web")] {
        #[wasm_bindgen]
        pub fn test(){
            log("1");
        }
    } else {
        #[wasm_bindgen]
        pub fn test(){
            log("2");
        }
    }
}
