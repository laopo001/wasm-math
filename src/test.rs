extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use std::rc::Rc;

#[wasm_bindgen]
pub struct Matrix {
    data: Rc<Vec<f64>>,
}
#[wasm_bindgen]
impl Matrix {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Matrix {
        return Matrix {
            data: Rc::new(vec![x, y, z]),
        };
    }
    pub fn data(&self) -> Box<[f64]> {
        return (*Rc::try_unwrap(self.data.clone())
            .unwrap_err()).clone()
            .into_boxed_slice();
    }
}
