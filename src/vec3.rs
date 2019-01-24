#[warn(dead_code)]
extern crate wasm_bindgen;
// use std::clone::Clone;
use wasm_bindgen::prelude::*;
// #[wasm_bindgen(typescript_custom_section)]
// const TS_APPEND_CONTENT: &'static str = r#"
// export type Coords = { "latitude": number, "longitude": number };
// "#;

#[wasm_bindgen]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Vec3 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z };
    }
    pub fn data(&self) -> Box<[f64]> {
        return Box::new([self.x, self.y, self.z]);
    }
    pub fn add(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    pub fn sub(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
    pub fn mul(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
    pub fn dot(&self, other: Vec3) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
    #[wasm_bindgen(js_name = lengthSq)]
    pub fn length_sq(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }
    pub fn normalize(&mut self) {
        let sq = self.length_sq();
        let inv = 1.0 / sq;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
    pub fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn clone(&self) -> Self {
        return Vec3::new(self.x, self.y, self.z);
    }
    pub fn equals(&self, other: Vec3) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
    pub fn lerp(&mut self, l: Vec3, r: Vec3, alpha: f64) {
        self.x = l.x + alpha * (r.x - l.x);
        self.y = l.y + alpha * (r.y - l.y);
        self.z = l.z + alpha * (r.z - l.z);
    }
}

// impl Clone for Vec3 {
//     fn clone(&self) -> Self {
//         self.clone()
//     }
// }
