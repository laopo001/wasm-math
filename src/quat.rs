#[warn(dead_code)]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use crate::vec3::Vec3;
use crate::math::DEG_TO_RAD;
use crate::mat4::Mat4;

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
    pub fn setFromAxisAngle(&mut self, axis: &Vec3, angle: f64) {
        let angleRad = angle * 0.5 * DEG_TO_RAD;
        let sa = angleRad.sin();
        let ca = angleRad.cos();
        self.x = sa * self.x;
        self.y = sa * self.y;
        self.z = sa * self.z;
        self.w = ca;
    }
    pub fn setFromEulerAngles(&mut self, ex: f64, ey: f64, ez: f64) {
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
    pub fn setFromMat4(&mut self, mat: &Mat4) {
        let imm_p_m: *const Mat4 = mat as *const Mat4;
        let m = mat.data.as_slice();

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
        let mut rs: f64;
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
    pub fn transformVector(&self, vec: &Vec3, res: &mut Vec3) {
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
    pub fn lerp(&mut self, lhs: &Quat, rhs: &Quat, alpha: f64) {
        let lx = lhs.x;
        let ly = lhs.y;
        let lz = lhs.z;
        let lw = lhs.w;
        let mut rx = rhs.x;
        let mut ry = rhs.y;
        let mut rz = rhs.z;
        let mut rw = rhs.w;

        // Calculate angle between them.
        let mut cosHalfTheta = lw * rw + lx * rx + ly * ry + lz * rz;

        if (cosHalfTheta < 0.0) {
            rw = -rw;
            rx = -rx;
            ry = -ry;
            rz = -rz;
            cosHalfTheta = -cosHalfTheta;
        }

        // If lhs == rhs or lhs == -rhs then theta == 0 and we can return lhs
        if cosHalfTheta.abs() >= 1.0 {
            self.w = lw;
            self.x = lx;
            self.y = ly;
            self.z = lz;
        }

        // Calculate temporary values.
        let halfTheta = cosHalfTheta.acos();
        let sinHalfTheta = (1.0 - cosHalfTheta * cosHalfTheta).sqrt();

        // If theta = 180 degrees then result is not fully defined
        // we could rotate around any axis normal to qa or qb
        if sinHalfTheta.abs() < 0.001 {
            self.w = (lw * 0.5 + rw * 0.5);
            self.x = (lx * 0.5 + rx * 0.5);
            self.y = (ly * 0.5 + ry * 0.5);
            self.z = (lz * 0.5 + rz * 0.5);
        }

        let ratioA = ((1.0 - alpha) * halfTheta).sin() / sinHalfTheta;
        let ratioB = (alpha * halfTheta).sin() / sinHalfTheta;

        // Calculate Quaternion.
        self.w = (lw * ratioA + rw * ratioB);
        self.x = (lx * ratioA + rx * ratioB);
        self.y = (ly * ratioA + ry * ratioB);
        self.z = (lz * ratioA + rz * ratioB);
    }
    pub fn default() -> Self {
        Quat::new(0.0, 0.0, 0.0, 1.0)
    }
    pub fn zero() -> Self {
        Quat::new(0.0, 0.0, 0.0, 0.0)
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
