#[warn(dead_code)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Vec2 {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Vec2 {
        return Vec2 { x, y };
    }
    pub fn data(&self) -> (f64, f64) {
        return (self.x, self.y);
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
        self.lengthSq().sqrt()
    }
    pub fn lengthSq(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }
    pub fn normalize(&mut self) {
        let sq = self.lengthSq();
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
    assert_eq!(a.y, 1.0);
}



#[test]
fn vec2_length() {
    let a = Vec2::new(3.0, 4.0);
    assert_eq!(a.length(), 5.0);
}
