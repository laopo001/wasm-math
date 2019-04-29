#[allow(dead_code, unused_mut)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::math::DEG_TO_RAD;
use crate::quat::Quat;
use crate::vec3::Vec3;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Mat4 {
    pub(crate) data: Box<[f64; 16]>,
}

#[wasm_bindgen]
impl Mat4 {
    #[wasm_bindgen(constructor)]
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
        return Mat4 {
            data: Box::new([
                n0, n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14, n15,
            ]),
        };
    }
    pub fn add(&mut self, other: &Mat4) {
        let a = other.data.as_ref();
        let r = self.data.as_mut();
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
    }
    pub fn data(&self) -> Box<[f64]> {
        self.data.clone()
    }
    pub fn set(
        &mut self,
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
    ) {
        let r = self.data.as_mut();
        r[0] = n0;
        r[1] = n1;
        r[2] = n2;
        r[3] = n3;
        r[4] = n4;
        r[5] = n5;
        r[6] = n6;
        r[7] = n7;
        r[8] = n8;
        r[9] = n9;
        r[10] = n10;
        r[11] = n11;
        r[12] = n12;
        r[13] = n13;
        r[14] = n14;
        r[15] = n15;
    }
    pub fn copy(&mut self, v: &Mat4) {
        let data = v.data.as_ref();
        let n0 = data[0];
        let n1 = data[1];
        let n2 = data[2];
        let n3 = data[3];
        let n4 = data[4];
        let n5 = data[5];
        let n6 = data[6];
        let n7 = data[7];
        let n8 = data[8];
        let n9 = data[9];
        let n10 = data[10];
        let n11 = data[11];
        let n12 = data[12];
        let n13 = data[13];
        let n14 = data[14];
        let n15 = data[15];
        self.set(
            n0, n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14, n15,
        );
    }
    // pub fn clone(&self) -> Self {
    //     let data = self.data.as_ref();
    //     let n0 = data[0];
    //     let n1 = data[1];
    //     let n2 = data[2];
    //     let n3 = data[3];
    //     let n4 = data[4];
    //     let n5 = data[5];
    //     let n6 = data[6];
    //     let n7 = data[7];
    //     let n8 = data[8];
    //     let n9 = data[9];
    //     let n10 = data[10];
    //     let n11 = data[11];
    //     let n12 = data[12];
    //     let n13 = data[13];
    //     let n14 = data[14];
    //     let n15 = data[15];
    //     return Mat4::new(
    //         n0, n1, n2, n3, n4, n5, n6, n7, n8, n9, n10, n11, n12, n13, n14, n15,
    //     );
    // }
    pub fn equals(&self, other: &Mat4) -> bool {
        let r = self.data.as_ref();
        let a = other.data.as_ref();
        return r[0] == a[0]
            && r[1] == a[1]
            && r[2] == a[2]
            && r[3] == a[3]
            && r[4] == a[4]
            && r[5] == a[5]
            && r[6] == a[6]
            && r[7] == a[7]
            && r[8] == a[8]
            && r[9] == a[9]
            && r[10] == a[10]
            && r[11] == a[11]
            && r[12] == a[12]
            && r[13] == a[13]
            && r[14] == a[14]
            && r[15] == a[15];
    }
    #[wasm_bindgen(js_name = setTranslate)]
    pub fn set_translate(&mut self, x: f64, y: f64, z: f64) {
        let m = self.data.as_mut();
        m[12] = x;
        m[13] = y;
        m[14] = z;
    }
    #[wasm_bindgen(js_name = getTranslate)]
    pub fn get_translate(&self, v: &mut Vec3) {
        let m = self.data.as_ref();
        v.set(m[12], m[13], m[14]);
    }
    #[wasm_bindgen(js_name = transformPoint)]
    pub fn transform_point(&self, vec: &Vec3, res: &mut Vec3) {
        let m = self.data.as_ref();
        let v = vec.data();
        let x = v[0] * m[0] + v[1] * m[4] + v[2] * m[8] + m[12];
        let y = v[0] * m[1] + v[1] * m[5] + v[2] * m[9] + m[13];
        let z = v[0] * m[2] + v[1] * m[6] + v[2] * m[10] + m[14];
        res.set(x, y, z);
    }
    #[wasm_bindgen(js_name = setFromAxisAngle)]
    pub fn set_from_axis_angle(&mut self, axis: &Vec3, angle: f64) {
        let m = self.data.as_mut();
        let angle = angle * DEG_TO_RAD;
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        let cos = angle.cos();
        let sin = angle.sin();
        let t = 1.0 - cos;
        let tx = t * x;
        let ty = t * y;
        m[0] = tx * x + cos;
        m[1] = tx * y + sin * z;
        m[2] = tx * z - sin * y;
        m[3] = 0.0;
        m[4] = tx * y - sin * z;
        m[5] = ty * y + cos;
        m[6] = ty * z + sin * x;
        m[7] = 0.0;
        m[8] = tx * z + sin * y;
        m[9] = ty * z - x * sin;
        m[10] = t * z * z + cos;
        m[11] = 0.0;
        m[12] = 0.0;
        m[13] = 0.0;
        m[14] = 0.0;
        m[15] = 1.0;
    }
    #[wasm_bindgen(js_name = setFromTRS)]
    pub fn set_from_trs(&mut self, t: &Vec3, r: &Quat, s: &Vec3) {
        let tx = t.x;
        let ty = t.y;
        let tz = t.z;

        let qx = r.x;
        let qy = r.y;
        let qz = r.z;
        let qw = r.w;

        let sx = s.x;
        let sy = s.y;
        let sz = s.z;

        let x2 = qx + qx;
        let y2 = qy + qy;
        let z2 = qz + qz;
        let xx = qx * x2;
        let xy = qx * y2;
        let xz = qx * z2;
        let yy = qy * y2;
        let yz = qy * z2;
        let zz = qz * z2;
        let wx = qw * x2;
        let wy = qw * y2;
        let wz = qw * z2;
        let m = self.data.as_mut();
        m[0] = (1.0 - (yy + zz)) * sx;
        m[1] = (xy + wz) * sx;
        m[2] = (xz - wy) * sx;
        m[3] = 0.0;

        m[4] = (xy - wz) * sy;
        m[5] = (1.0 - (xx + zz)) * sy;
        m[6] = (yz + wx) * sy;
        m[7] = 0.0;

        m[8] = (xz + wy) * sz;
        m[9] = (yz - wx) * sz;
        m[10] = (1.0 - (xx + yy)) * sz;
        m[11] = 0.0;

        m[12] = tx;
        m[13] = ty;
        m[14] = tz;
        m[15] = 1.0;
    }
    #[wasm_bindgen(js_name = setScale)]
    pub fn set_scale(&mut self, x: f64, y: f64, z: f64) {
        let m = self.data.as_mut();
        m[0] = x;
        m[5] = y;
        m[10] = z;
    }
    pub fn get_scale(&self, v: &mut Vec3) {
        let mut temp1 = Vec3::default();
        let mut temp2 = Vec3::default();
        let mut temp3 = Vec3::default();
        self.get_x(&mut temp1);
        self.get_y(&mut temp2);
        self.get_z(&mut temp3);
        v.set(temp1.length(), temp2.length(), temp3.length());
    }
    #[wasm_bindgen(js_name = getX)]
    pub fn get_x(&self, v: &mut Vec3) {
        let m = self.data.as_ref();
        v.set(m[0], m[1], m[2]);
    }
    #[wasm_bindgen(js_name = getY)]
    pub fn get_y(&self, v: &mut Vec3) {
        let m = self.data.as_ref();
        v.set(m[4], m[5], m[6]);
    }
    #[wasm_bindgen(js_name = getZ)]
    pub fn get_z(&self, v: &mut Vec3) {
        let m = self.data.as_ref();
        v.set(m[8], m[9], m[10]);
    }
    #[wasm_bindgen(js_name = setIdentity)]
    pub fn set_identity(&mut self) {
        let m = self.data.as_mut();
        m[0] = 1.0;
        m[1] = 0.0;
        m[2] = 0.0;
        m[3] = 0.0;
        m[4] = 0.0;
        m[5] = 1.0;
        m[6] = 0.0;
        m[7] = 0.0;
        m[8] = 0.0;
        m[9] = 0.0;
        m[10] = 1.0;
        m[11] = 0.0;
        m[12] = 0.0;
        m[13] = 0.0;
        m[14] = 0.0;
        m[15] = 1.0;
    }
    pub fn transpose(&mut self) {
        let mut tmp: f64;
        let m = self.data.as_mut();
        tmp = m[1];
        m[1] = m[4];
        m[4] = tmp;

        tmp = m[2];
        m[2] = m[8];
        m[8] = tmp;

        tmp = m[3];
        m[3] = m[12];
        m[12] = tmp;

        tmp = m[6];
        m[6] = m[9];
        m[9] = tmp;

        tmp = m[7];
        m[7] = m[13];
        m[13] = tmp;

        tmp = m[11];
        m[11] = m[14];
        m[14] = tmp;
    }
    pub fn mul(&mut self, other: &Mat4) {
        let m = self.data.as_mut();
        let a00 = m[0];
        let a01 = m[1];
        let a02 = m[2];
        let a03 = m[3];
        let a10 = m[4];
        let a11 = m[5];
        let a12 = m[6];
        let a13 = m[7];
        let a20 = m[8];
        let a21 = m[9];
        let a22 = m[10];
        let a23 = m[11];
        let a30 = m[12];
        let a31 = m[13];
        let a32 = m[14];
        let a33 = m[15];
        let b = other.data.as_ref();
        let mut b0 = b[0];
        let mut b1 = b[1];
        let mut b2 = b[2];
        let mut b3 = b[3];
        m[0] = a00 * b0 + a10 * b1 + a20 * b2 + a30 * b3;
        m[1] = a01 * b0 + a11 * b1 + a21 * b2 + a31 * b3;
        m[2] = a02 * b0 + a12 * b1 + a22 * b2 + a32 * b3;
        m[3] = a03 * b0 + a13 * b1 + a23 * b2 + a33 * b3;

        b0 = b[4];
        b1 = b[5];
        b2 = b[6];
        b3 = b[7];
        m[4] = a00 * b0 + a10 * b1 + a20 * b2 + a30 * b3;
        m[5] = a01 * b0 + a11 * b1 + a21 * b2 + a31 * b3;
        m[6] = a02 * b0 + a12 * b1 + a22 * b2 + a32 * b3;
        m[7] = a03 * b0 + a13 * b1 + a23 * b2 + a33 * b3;

        b0 = b[8];
        b1 = b[9];
        b2 = b[10];
        b3 = b[11];
        m[8] = a00 * b0 + a10 * b1 + a20 * b2 + a30 * b3;
        m[9] = a01 * b0 + a11 * b1 + a21 * b2 + a31 * b3;
        m[10] = a02 * b0 + a12 * b1 + a22 * b2 + a32 * b3;
        m[11] = a03 * b0 + a13 * b1 + a23 * b2 + a33 * b3;

        b0 = b[12];
        b1 = b[13];
        b2 = b[14];
        b3 = b[15];
        m[12] = a00 * b0 + a10 * b1 + a20 * b2 + a30 * b3;
        m[13] = a01 * b0 + a11 * b1 + a21 * b2 + a31 * b3;
        m[14] = a02 * b0 + a12 * b1 + a22 * b2 + a32 * b3;
        m[15] = a03 * b0 + a13 * b1 + a23 * b2 + a33 * b3;
    }
    pub fn mul2(&mut self, a: &Mat4, b: &Mat4) {
        let m = self.data.as_mut();
        let a = a.data.as_ref();
        let a00 = a[0];
        let a01 = a[1];
        let a02 = a[2];
        let a03 = a[3];
        let a10 = a[4];
        let a11 = a[5];
        let a12 = a[6];
        let a13 = a[7];
        let a20 = a[8];
        let a21 = a[9];
        let a22 = a[10];
        let a23 = a[11];
        let a30 = a[12];
        let a31 = a[13];
        let a32 = a[14];
        let a33 = a[15];
        let b = b.data.as_ref();
        let mut b0 = b[0];
        let mut b1 = b[1];
        let mut b2 = b[2];
        let mut b3 = b[3];
        m[0] = a00 * b0 + a10 * b1 + a20 * b2 + a30 * b3;
        m[1] = a01 * b0 + a11 * b1 + a21 * b2 + a31 * b3;
        m[2] = a02 * b0 + a12 * b1 + a22 * b2 + a32 * b3;
        m[3] = a03 * b0 + a13 * b1 + a23 * b2 + a33 * b3;

        b0 = b[4];
        b1 = b[5];
        b2 = b[6];
        b3 = b[7];
        m[4] = a00 * b0 + a10 * b1 + a20 * b2 + a30 * b3;
        m[5] = a01 * b0 + a11 * b1 + a21 * b2 + a31 * b3;
        m[6] = a02 * b0 + a12 * b1 + a22 * b2 + a32 * b3;
        m[7] = a03 * b0 + a13 * b1 + a23 * b2 + a33 * b3;

        b0 = b[8];
        b1 = b[9];
        b2 = b[10];
        b3 = b[11];
        m[8] = a00 * b0 + a10 * b1 + a20 * b2 + a30 * b3;
        m[9] = a01 * b0 + a11 * b1 + a21 * b2 + a31 * b3;
        m[10] = a02 * b0 + a12 * b1 + a22 * b2 + a32 * b3;
        m[11] = a03 * b0 + a13 * b1 + a23 * b2 + a33 * b3;

        b0 = b[12];
        b1 = b[13];
        b2 = b[14];
        b3 = b[15];
        m[12] = a00 * b0 + a10 * b1 + a20 * b2 + a30 * b3;
        m[13] = a01 * b0 + a11 * b1 + a21 * b2 + a31 * b3;
        m[14] = a02 * b0 + a12 * b1 + a22 * b2 + a32 * b3;
        m[15] = a03 * b0 + a13 * b1 + a23 * b2 + a33 * b3;
    }
    pub fn invert(&mut self) {
        let m = self.data.as_mut();
        let a00 = m[0];
        let a01 = m[1];
        let a02 = m[2];
        let a03 = m[3];
        let a10 = m[4];
        let a11 = m[5];
        let a12 = m[6];
        let a13 = m[7];
        let a20 = m[8];
        let a21 = m[9];
        let a22 = m[10];
        let a23 = m[11];
        let a30 = m[12];
        let a31 = m[13];
        let a32 = m[14];
        let a33 = m[15];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
        if det == 0.0 {
            // #ifdef DEBUG
            warn("Can't invert matrix, determinant is 0");
            // #endif
            self.set_identity();
        } else {
            let inv_det = 1.0 / det;
            m[0] = (a11 * b11 - a12 * b10 + a13 * b09) * inv_det;
            m[1] = (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det;
            m[2] = (a31 * b05 - a32 * b04 + a33 * b03) * inv_det;
            m[3] = (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det;
            m[4] = (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det;
            m[5] = (a00 * b11 - a02 * b08 + a03 * b07) * inv_det;
            m[6] = (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det;
            m[7] = (a20 * b05 - a22 * b02 + a23 * b01) * inv_det;
            m[8] = (a10 * b10 - a11 * b08 + a13 * b06) * inv_det;
            m[9] = (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det;
            m[10] = (a30 * b04 - a31 * b02 + a33 * b00) * inv_det;
            m[11] = (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det;
            m[12] = (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det;
            m[13] = (a00 * b09 - a01 * b07 + a02 * b06) * inv_det;
            m[14] = (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det;
            m[15] = (a20 * b03 - a21 * b01 + a22 * b00) * inv_det;
        }
    }
    pub fn get_identity() -> Self {
        Mat4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
    pub fn default() -> Self {
        Mat4::get_identity()
    }
}

#[test]
fn mat4_add() {
    let mut mat1 = Mat4::new(
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    let mat2 = Mat4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    mat1.add(&mat2);
    assert_eq!(mat2.data[0], 1.0);
}

#[test]
fn mat4_mut() {
    let mut mat1 = Mat4::get_identity();
    let mat2 = Mat4::get_identity();
    mat1.mul(&mat2);
    assert_eq!(mat1.equals(&mat2), true);
}

#[test]
fn mat4_invert() {
    let mut mat1 = Mat4::get_identity();
    let mat2 = Mat4::get_identity();
    mat1.invert();
    assert_eq!(mat1.equals(&mat2), true);
}

#[test]
fn mat4_set_scale_get_scale() {
    let mut mat1 = Mat4::get_identity();
    mat1.set_scale(3.0, 4.0, 5.0);
    let mut v = Vec3::default();
    mat1.get_scale(&mut v);
    assert_eq!(v, Vec3::new(3.0, 4.0, 5.0));
}
