#[warn(dead_code)]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use crate::mat4::Mat4;
use crate::math::{DEG_TO_RAD, RAD_TO_DEG};
use crate::vec3::Vec3;

#[wasm_bindgen]
#[derive(Debug,Clone, Copy)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[wasm_bindgen]
impl Quat {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quat {
        return Quat { x, y, z, w };
    }
    pub fn data(&self) -> Box<[f32]> {
        return Box::new([self.x, self.y, self.z, self.w]);
    }
    #[wasm_bindgen(js_name = lengthSq)]
    pub fn length_sq(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    }
    pub fn length(&self) -> f32 {
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
    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
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
    pub fn mul(&mut self, q: &Quat) {
        let q1x = self.x;
        let q1y = self.y;
        let q1z = self.z;
        let q1w = self.w;
        let q2x = q.x;
        let q2y = q.y;
        let q2z = q.z;
        let q2w = q.w;
        self.x = q1w * q2x + q1x * q2w + q1y * q2z - q1z * q2y;
        self.y = q1w * q2y + q1y * q2w + q1z * q2x - q1x * q2z;
        self.z = q1w * q2z + q1z * q2w + q1x * q2y - q1y * q2x;
        self.w = q1w * q2w - q1x * q2x - q1y * q2y - q1z * q2z;
    }
    pub fn mul2(&mut self, a: &Quat, b: &Quat) {
        let q1x = a.x;
        let q1y = a.y;
        let q1z = a.z;
        let q1w = a.w;
        let q2x = b.x;
        let q2y = b.y;
        let q2z = b.z;
        let q2w = b.w;
        self.x = q1w * q2x + q1x * q2w + q1y * q2z - q1z * q2y;
        self.y = q1w * q2y + q1y * q2w + q1z * q2x - q1x * q2z;
        self.z = q1w * q2z + q1z * q2w + q1x * q2y - q1y * q2x;
        self.w = q1w * q2w - q1x * q2x - q1y * q2y - q1z * q2z;
    }
    #[wasm_bindgen(js_name = setFromAxisAngle)]
    pub fn set_from_axis_angle(&mut self, axis: &Vec3, angle: f32) {
        let angle_rad = angle * 0.5 * DEG_TO_RAD;
        let sa = angle_rad.sin();
        let ca = angle_rad.cos();
        self.x = sa * axis.x;
        self.y = sa * axis.y;
        self.z = sa * axis.z;
        self.w = ca;
    }
    #[wasm_bindgen(js_name = setFromEulerAngles)]
    pub fn set_from_euler_angles(&mut self, ex: f32, ey: f32, ez: f32) {
        let hald = 0.5 * DEG_TO_RAD;

        let ex = ex * hald;
        let ey = ey * hald;
        let ez = ez * hald;

        let sx = ex.sin();
        let cx = ex.cos();
        let sy = ey.sin();
        let cy = ey.cos();
        let sz = ez.sin();
        let cz = ez.cos();

        self.x = sx * cy * cz - cx * sy * sz;
        self.y = cx * sy * cz + sx * cy * sz;
        self.z = cx * cy * sz - sx * sy * cz;
        self.w = cx * cy * cz + sx * sy * sz;
    }
    #[wasm_bindgen(js_name = setFromMat4)]
    pub fn set_from_mat4(&mut self, mat: &Mat4) {
        let m = *mat.data;

        let mut m00 = m[0];
        let mut m01 = m[1];
        let mut m02 = m[2];
        let mut m10 = m[4];
        let mut m11 = m[5];
        let mut m12 = m[6];
        let mut m20 = m[8];
        let mut m21 = m[9];
        let mut m22 = m[10];
        let lx = 1.0 / (m00 * m00 + m01 * m01 + m02 * m02).sqrt();
        let ly = 1.0 / (m10 * m10 + m11 * m11 + m12 * m12).sqrt();
        let lz = 1.0 / (m20 * m20 + m21 * m21 + m22 * m22).sqrt();
        m00 *= lx;
        m01 *= lx;
        m02 *= lx;
        m10 *= ly;
        m11 *= ly;
        m12 *= ly;
        m20 *= lz;
        m21 *= lz;
        m22 *= lz;
        let tr = m00 + m11 + m22;
        let mut rs: f32;
        if tr >= 0.0 {
            let mut s = (tr + 1.0).sqrt();
            self.w = s * 0.5;
            s = 0.5 / s;
            self.x = (m12 - m21) * s;
            self.y = (m20 - m02) * s;
            self.z = (m01 - m10) * s;
        } else {
            if m00 > m11 {
                if m00 > m22 {
                    // XDiagDomMatrix
                    rs = (m00 - (m11 + m22)) + 1.0;
                    rs = rs.sqrt();

                    self.x = rs * 0.5;
                    rs = 0.5 / rs;
                    self.w = (m12 - m21) * rs;
                    self.y = (m01 + m10) * rs;
                    self.z = (m02 + m20) * rs;
                } else {
                    // ZDiagDomMatrix
                    rs = (m22 - (m00 + m11)) + 1.0;
                    rs = rs.sqrt();

                    self.z = rs * 0.5;
                    rs = 0.5 / rs;
                    self.w = (m01 - m10) * rs;
                    self.x = (m20 + m02) * rs;
                    self.y = (m21 + m12) * rs;
                }
            } else if m11 > m22 {
                // YDiagDomMatrix
                rs = (m11 - (m22 + m00)) + 1.0;
                rs = rs.sqrt();

                self.y = rs * 0.5;
                rs = 0.5 / rs;
                self.w = (m20 - m02) * rs;
                self.z = (m12 + m21) * rs;
                self.x = (m10 + m01) * rs;
            } else {
                // ZDiagDomMatrix
                rs = (m22 - (m00 + m11)) + 1.0;
                rs = rs.sqrt();

                self.z = rs * 0.5;
                rs = 0.5 / rs;
                self.w = (m01 - m10) * rs;
                self.x = (m20 + m02) * rs;
                self.y = (m21 + m12) * rs;
            }
        }
    }
    #[wasm_bindgen(js_name = transformVector)]
    pub fn transform_vector(&self, vec: &Vec3, res: &mut Vec3) {
        let x = vec.x;
        let y = vec.y;
        let z = vec.z;
        let qx = self.x;
        let qy = self.y;
        let qz = self.z;
        let qw = self.w;
        let ix = qw * x + qy * z - qz * y;
        let iy = qw * y + qz * x - qx * z;
        let iz = qw * z + qx * y - qy * x;
        let iw = -qx * x - qy * y - qz * z;
        res.x = ix * qw + iw * -qx + iy * -qz - iz * -qy;
        res.y = iy * qw + iw * -qy + iz * -qx - ix * -qz;
        res.z = iz * qw + iw * -qz + ix * -qy - iy * -qx;
    }
    pub fn lerp(&mut self, lhs: &Quat, rhs: &Quat, alpha: f32) {
        let lx = lhs.x;
        let ly = lhs.y;
        let lz = lhs.z;
        let lw = lhs.w;
        let mut rx = rhs.x;
        let mut ry = rhs.y;
        let mut rz = rhs.z;
        let mut rw = rhs.w;

        // Calculate angle between them.
        let mut cos_half_theta = lw * rw + lx * rx + ly * ry + lz * rz;

        if cos_half_theta < 0.0 {
            rw = -rw;
            rx = -rx;
            ry = -ry;
            rz = -rz;
            cos_half_theta = -cos_half_theta;
        }

        // If lhs == rhs or lhs == -rhs then theta == 0 and we can return lhs
        if cos_half_theta.abs() >= 1.0 {
            self.w = lw;
            self.x = lx;
            self.y = ly;
            self.z = lz;
        }

        // Calculate temporary values.
        let half_theta = cos_half_theta.acos();
        let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();

        // If theta = 180 degrees then result is not fully defined
        // we could rotate around any axis normal to qa or qb
        if sin_half_theta.abs() < 0.001 {
            self.w = lw * 0.5 + rw * 0.5;
            self.x = lx * 0.5 + rx * 0.5;
            self.y = ly * 0.5 + ry * 0.5;
            self.z = lz * 0.5 + rz * 0.5;
        }

        let ratio_a = ((1.0 - alpha) * half_theta).sin() / sin_half_theta;
        let ratio_b = (alpha * half_theta).sin() / sin_half_theta;

        // Calculate Quaternion.
        self.w = lw * ratio_a + rw * ratio_b;
        self.x = lx * ratio_a + rx * ratio_b;
        self.y = ly * ratio_a + ry * ratio_b;
        self.z = lz * ratio_a + rz * ratio_b;
    }
    pub fn default() -> Self {
        Quat::new(0.0, 0.0, 0.0, 1.0)
    }
    pub fn zero() -> Self {
        Quat::new(0.0, 0.0, 0.0, 0.0)
    }
    fn conjugate(&mut self) -> &mut Self {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
        return self;
    }
    pub fn invert(&mut self) {
        return self.conjugate().normalize();
    }
    #[wasm_bindgen(js_name = getEulerAngles)]
    pub fn get_euler_angles(&self, eulers: &mut Vec3) {
        let qx = self.x;

        let qy = self.y;
        let qz = self.z;
        let qw = self.w;
        let a2 = 2.0 * (qw * qy - qx * qz);
        let x: f32;
        let y: f32;
        let z: f32;
        if a2 <= -0.99999 {
            x = 2.0 * qx.atan2(qw);
            y = -std::f32::consts::PI / 2.0;
            z = 0.0;
        } else if a2 >= 0.99999 {
            x = 2.0 * qx.atan2(qw);
            y = std::f32::consts::PI / 2.0;
            z = 0.0;
        } else {
            x = (2.0 * (qw * qx + qy * qz)).atan2(1.0 - 2.0 * (qx * qx + qy * qy));
            y = a2.asin();
            z = (2.0 * (qw * qz + qx * qy)).atan2(1.0 - 2.0 * (qy * qy + qz * qz));
        }
        eulers.set(x, y, z);
        eulers.scale(RAD_TO_DEG);
    }
}

//#[test]
//fn quat_transformVector() {
//    let mut q = Quat::default();
//    q.setFromEulerAngles(0.0,90.0,0.0);
//    let v = Vec3::new(1.0, 0.0, 0.0);
//    let mut res = Vec3::default();
//    q.transformVector(&v, &mut res);
//    assert!(res.equals(&Vec3::new(0.0,0.0,1.0)));
//}
