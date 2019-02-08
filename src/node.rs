extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use crate::vec3::Vec3;
use std::rc::Rc;

#[wasm_bindgen]
pub struct Node {
    local_position: Rc<Vec3>,
}

impl Node {
    pub fn new() -> Self {
        return Node {
            local_position: Rc::new(Vec3::new(0.0, 0.0, 0.0)),
        };
    }
}
