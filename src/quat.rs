#[warn(dead_code)]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Quat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[wasm_bindgen]
impl Quat {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quat {
        return Quat { x, y, z, w };
    }
    pub fn data(&self) -> Box<[f64]> {
        return Box::new([self.x, self.y, self.z, self.w]);
    }
    #[wasm_bindgen(js_name = lengthSq)]
    pub fn length_sq(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    }
    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }
    pub fn normalize(&mut self) {
        let len = self.length();
        if len == 0.0 {
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
            self.w = 1.0;
            return;
        }
        let inv = 1.0 / len;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
        self.w *= inv;
    }
    pub fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
    pub fn set(&mut self, x: f64, y: f64, z: f64, w: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
    pub fn copy(&mut self, v: &Quat) {
        self.set(v.x, v.y, v.z, v.w);
    }
    pub fn clone(&self) -> Self {
        return Quat::new(self.x, self.y, self.z, self.w);
    }
    pub fn equals(&self, other: &Quat) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w;
    }
    pub fn lerp(&mut self, l: &Quat, r: &Quat, alpha: f64) {
        self.x = l.x + alpha * (r.x - l.x);
        self.y = l.y + alpha * (r.y - l.y);
        self.z = l.z + alpha * (r.z - l.z);
        self.w = l.w + alpha * (r.w - l.w);
    }
    pub fn default() -> Self {
        Quat::new(0.0, 0.0, 0.0, 1.0)
    }
}
