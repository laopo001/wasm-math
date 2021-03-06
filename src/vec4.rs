#[warn(dead_code)]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[wasm_bindgen]
impl Vec4 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        return Vec4 { x, y, z, w };
    }
    pub fn data(&self) -> Box<[f32]> {
        return Box::new([self.x, self.y, self.z, self.w]);
    }
    pub fn add(&mut self, other: &Vec4) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
    pub fn sub(&mut self, other: &Vec4) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
    pub fn mul(&mut self, other: &Vec4) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
    pub fn dot(&self, other: &Vec4) -> f32 {
        return self.x * other.x + self.y * other.y;
    }
    #[wasm_bindgen(js_name = lengthSq)]
    pub fn length_sq(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    }
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
    pub fn normalize(&mut self) {
        let sq = self.length_sq();
        if sq == 0.0 {
            return;
        }
        let inv = 1.0 / sq;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
        self.w *= inv;
    }
    pub fn scale(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
    pub fn copy(&mut self, v: &Vec4) {
        self.set(v.x, v.y, v.z, v.w);
    }
    // pub fn clone(&self) -> Self {
    //     return Vec4::new(self.x, self.y, self.z, self.w);
    // }
    pub fn equals(&self, other: &Vec4) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w;
    }
    pub fn lerp(&mut self, l: &Vec4, r: &Vec4, alpha: f32) {
        self.x = l.x + alpha * (r.x - l.x);
        self.y = l.y + alpha * (r.y - l.y);
        self.z = l.z + alpha * (r.z - l.z);
        self.w = l.w + alpha * (r.w - l.w);
    }
    pub fn default() -> Self {
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    }
}
