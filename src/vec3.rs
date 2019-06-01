#[warn(dead_code)]
extern crate wasm_bindgen;

use std::ops::{Add, Sub, AddAssign, SubAssign, MulAssign, Mul, Rem};
// use std::clone::Clone;
use wasm_bindgen::prelude::*;
// #[wasm_bindgen(typescript_custom_section)]
// const TS_APPEND_CONTENT: &'static str = r#"
// export type Coords = { "latitude": number, "longitude": number };
// "#;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[wasm_bindgen]
impl Vec3 {
	#[wasm_bindgen(constructor)]
	pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
		return Vec3 { x, y, z };
	}
	pub fn data(&self) -> Box<[f32]> {
		return Box::new([self.x, self.y, self.z]);
	}
	//	pub fn add(&mut self, other: &Vec3) {
//		self.x += other.x;
//		self.y += other.y;
//		self.z += other.z;
//	}
//	pub fn add2(&mut self, a: &Vec3, b: &Vec3) {
//		self.x = a.x + b.x;
//		self.y = a.y + b.y;
//		self.z = a.z + b.z;
//	}
//	pub fn sub(&mut self, other: &Vec3) {
//		self.x -= other.x;
//		self.y -= other.y;
//		self.z -= other.z;
//	}
//	pub fn sub2(&mut self, a: &Vec3, b: &Vec3) {
//		self.x = a.x - b.x;
//		self.y = a.y - b.y;
//		self.z = a.z - b.z;
//	}
//	pub fn mul(&mut self, other: &Vec3) {
//		self.x *= other.x;
//		self.y *= other.y;
//		self.z *= other.z;
//	}
//	pub fn mul2(&mut self, a: &Vec3, b: &Vec3) {
//		self.x = a.x * b.x;
//		self.y = a.y * b.y;
//		self.z = a.z * b.z;
//	}
	pub fn dot(&self, other: &Vec3) -> f32 {
		return self.x * other.x + self.y * other.y + self.z * other.z;
	}
	#[wasm_bindgen(js_name = lengthSq)]
	pub fn length_sq(&self) -> f32 {
		return self.x * self.x + self.y * self.y + self.z * self.z;
	}
	pub fn length(&self) -> f32 {
		self.length_sq().sqrt()
	}
	pub fn normalize(mut self) -> Self {
		let sq = self.length();
		if sq == 0.0 {
			return self;
		}
		let inv = 1.0 / sq;
		self.x *= inv;
		self.y *= inv;
		self.z *= inv;
		self
	}
	pub fn scale(mut self, scalar: f32) -> Self {
		self.x *= scalar;
		self.y *= scalar;
		self.z *= scalar;
		self
	}
	pub fn set(&mut self, x: f32, y: f32, z: f32) {
		self.x = x;
		self.y = y;
		self.z = z;
	}
	pub fn copy(&mut self, v: &Vec3) {
		self.set(v.x, v.y, v.z);
	}
	// pub fn clone(&self) -> Self {
	//     return Vec3::new(self.x, self.y, self.z);
	// }
	pub fn equals(&self, other: &Vec3) -> bool {
		return self.x == other.x && self.y == other.y && self.z == other.z;
	}
	pub fn lerp(&mut self, l: &Vec3, r: &Vec3, alpha: f32) {
		self.x = l.x + alpha * (r.x - l.x);
		self.y = l.y + alpha * (r.y - l.y);
		self.z = l.z + alpha * (r.z - l.z);
	}
	pub fn default() -> Self {
		Vec3::new(0.0, 0.0, 0.0)
	}
}

// impl Clone for Vec3 {
//     fn clone(&self) -> Self {
//         self.clone()
//     }
// }

impl PartialEq for Vec3 {
	fn eq(&self, other: &Vec3) -> bool {
		return self.x == other.x && self.y == other.y && self.z == other.z;
	}
}

//impl Sized for Vec3 {}

impl AsRef<Vec3> for Vec3 {
	fn as_ref(&self) -> &Vec3 {
		self
	}
}

impl Add for Vec3 {
	type Output = Vec3;
	fn add(self, other: Vec3) -> Vec3 {
		let mut res = Vec3::default();
		res.x = self.x + other.x;
		res.y = self.y + other.y;
		res.z = self.z + other.z;
		res
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}
}

impl Sub for Vec3 {
	type Output = Vec3;
	fn sub(self, other: Vec3) -> Vec3 {
		let mut res = Vec3::default();
		res.x = self.x - other.x;
		res.y = self.y - other.y;
		res.z = self.z - other.z;
		res
	}
}

impl SubAssign for Vec3 {
	fn sub_assign(&mut self, other: Self) {
		self.x -= other.x;
		self.y -= other.y;
		self.z -= other.z;
	}
}

impl Mul for Vec3 {
	type Output = Vec3;
	fn mul(self, other: Vec3) -> Vec3 {
		let mut res = Vec3::default();
		res.x = self.x * other.x;
		res.y = self.y * other.y;
		res.z = self.z * other.z;
		res
	}
}

impl MulAssign for Vec3 {
	fn mul_assign(&mut self, other: Self) {
		self.x *= other.x;
		self.y *= other.y;
		self.z *= other.z;
	}
}

impl Rem for Vec3 {
	type Output = Self;
	fn rem(self, b: Self) -> Self {
		let mut res = Vec3::default();
//		self.y*b.z-self.z*b.y
//		self.z*b.x-self.x*b.z
//		self.x*b.y-self.y*b.x
		res.x = self.y * b.z - self.z * b.y;
		res.y = self.z * b.x - self.x * b.z;
		res.z = self.x * b.y - self.y * b.x;
		res
	}
}


#[test]
fn test_add() {
	let v1 = Vec3::new(0., 1., 2.);
	let v2 = Vec3::new(0., 1., 2.);
	assert_eq!(v1 + v2, Vec3::new(0., 2., 4.));
}

fn t<T: AsRef<Vec3>>(s: T) -> bool {
	let v1 = Vec3::new(0., 1., 2.);
	v1.equals(s.as_ref())
}

#[test]
fn test_AsRef() {
	let v1 = Vec3::new(0., 1., 2.);
	let v2 = Vec3::new(0., 1., 2.);
	assert!(t(v1));
}
