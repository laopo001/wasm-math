#[warn(dead_code)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::vec4;

pub struct Mat4 {
    pub data: [f64; 16],
}

impl Mat4 {
    pub fn new(
        n0: f64,
        n1: f64,
        n2: f64,
        n3: f64,
        n4: f64,
        n5: f64,
        n6: f64,
        n7: f64,
        n8: f64,
        n9: f64,
        n10: f64,
        n11: f64,
        n12: f64,
        n13: f64,
        n14: f64,
        n15: f64,
    ) -> Self {
        let a  = [
            n0, n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14, n15,
        ];
        return Mat4 { data: a };
    }
    pub fn add(&mut self, other: &Mat4) {
        let a = other.data;
        let mut r = self.data;
        r[0] += a[0];
        r[1] += a[1];
        r[2] += a[2];
        r[3] += a[3];
        r[4] += a[4];
        r[5] += a[5];
        r[6] += a[6];
        r[7] += a[7];
        r[8] += a[8];
        r[9] += a[9];
        r[10] += a[10];
        r[11] += a[11];
        r[12] += a[12];
        r[13] += a[13];
        r[14] += a[14];
        r[15] += a[15];
        self.data = r;
    }
}

#[test]
fn mat4_add() {
    let mut mat1 = Mat4::new(
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    let mut mat2 = Mat4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    mat1.add(&mat2);
    assert_eq!(mat1.data[0], 1.0);
}
