#[warn(dead_code)]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

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
    pub fn data(&self) -> Box<[f64]> {
        return Box::new([self.x, self.y]);
    }
    pub fn add(&mut self, other: &Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
    pub fn sub(&mut self, other: &Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
    pub fn mul(&mut self, other: &Vec2) {
        self.x *= other.x;
        self.y *= other.y;
    }
    pub fn dot(&self, other: &Vec2) -> f64 {
        return self.x * other.x + self.y * other.y;
    }
    #[wasm_bindgen(js_name = lengthSq)]
    pub fn length_sq(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }
    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
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
    pub fn copy(&mut self, v: &Vec2) {
        self.set(v.x, v.y);
    }
    pub fn clone(&self) -> Self {
        return Vec2::new(self.x, self.y);
    }
    pub fn equals(&self, other: &Vec2) -> bool {
        return self.x == other.x && self.y == other.y;
    }
    pub fn lerp(&mut self, l: &Vec2, r: &Vec2, alpha: f64) {
        self.x = l.x + alpha * (r.x - l.x);
        self.y = l.y + alpha * (r.y - l.y);
    }
    pub fn default() -> Self {
        Vec2::new(0.0, 0.0)
    }
}

#[test]
fn vec2_add() {
    let mut a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(1.0, 1.0);
    a.add(&b);
    assert_eq!(a.x, 1.0);
    assert_eq!(b.y, 1.0);
}

#[test]
fn vec2_length() {
    let a = Vec2::new(3.0, 4.0);
    assert_eq!(a.length(), 5.0);
}

#[test]
fn vec2_clone() {
    let a = Vec2::new(3.0, 4.0);
    assert_eq!(a.clone().length(), 5.0);
}
