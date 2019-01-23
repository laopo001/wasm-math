#[warn(dead_code)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
extern crate js_sys;
use js_sys::{Array, Float64Array};

#[wasm_bindgen]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Vec2 {
        return Vec2 { x, y };
    }
    pub fn data(&self) -> Float64Array {
        // JsValue::
        return Float64Array::new(&JsValue::from_f64(self.x));
    }
    pub fn add(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
    pub fn sub(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
    pub fn mul(&mut self, rhs: Vec2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
    pub fn dot(&self, rhs: Vec2) -> f64 {
        return self.x * rhs.x + self.y * rhs.y;
    }
    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }
    #[wasm_bindgen(js_name = lengthSq)]
    pub fn length_sq(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }
    pub fn normalize(&mut self) {
        let sq = self.length_sq();
        let inv = 1.0 / sq;
        self.x *= inv;
        self.y *= inv;
    }
    pub fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

#[test]
fn vec2_add() {
    let mut a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(1.0, 1.0);
    a.add(b);
    assert_eq!(a.x, 1.0);
    a.y = 2.0;
    assert_eq!(a.y, 2.0);
}

#[test]
fn vec2_length() {
    let a = Vec2::new(3.0, 4.0);
    assert_eq!(a.length(), 5.0);
}
