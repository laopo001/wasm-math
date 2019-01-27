extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Mat3 {
    pub(crate) data: Vec<f64>,
}

#[wasm_bindgen]
impl Mat3 {
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
    ) -> Self {
        return Mat3 {
            data: vec![n0, n1, n2, n3, n4, n5, n6, n7, n8],
        };
    }
    pub fn data(&self) -> Box<[f64]> {
        self.data.clone().into_boxed_slice()
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
    ) {
        let r = self.data.as_mut_slice();
        r[0] = n0;
        r[1] = n1;
        r[2] = n2;
        r[3] = n3;
        r[4] = n4;
        r[5] = n5;
        r[6] = n6;
        r[7] = n7;
        r[8] = n8;
    }
    pub fn copy(&mut self, v: &Mat3) {
        let data = v.data.as_slice();
        let n0 = data[0];
        let n1 = data[1];
        let n2 = data[2];
        let n3 = data[3];
        let n4 = data[4];
        let n5 = data[5];
        let n6 = data[6];
        let n7 = data[7];
        let n8 = data[8];
        self.set(n0, n1, n2, n3, n4, n5, n6, n7, n8);
    }
    pub fn clone(&self) -> Self {
        let data = self.data.as_slice();
        let n0 = data[0];
        let n1 = data[1];
        let n2 = data[2];
        let n3 = data[3];
        let n4 = data[4];
        let n5 = data[5];
        let n6 = data[6];
        let n7 = data[7];
        let n8 = data[8];
        return Mat3::new(n0, n1, n2, n3, n4, n5, n6, n7, n8);
    }
    pub fn equals(&self, other: &Mat3) -> bool {
        let r = self.data();
        let a = other.data();
        return r[0] == a[0]
            && r[1] == a[1]
            && r[2] == a[2]
            && r[3] == a[3]
            && r[4] == a[4]
            && r[5] == a[5]
            && r[6] == a[6]
            && r[7] == a[7]
            && r[8] == a[8];
    }
    #[wasm_bindgen(js_name = setIdentity)]
    pub fn set_identity(&mut self) {
        let m = self.data.as_mut_slice();
        m[0] = 1.0;
        m[1] = 0.0;
        m[2] = 0.0;

        m[3] = 0.0;
        m[4] = 1.0;
        m[5] = 0.0;

        m[6] = 0.0;
        m[7] = 0.0;
        m[8] = 1.0;
    }
    pub fn transpose(&mut self) {
        let m = self.data.as_mut_slice();
        let mut tmp: f64;
        tmp = m[1];
        m[1] = m[3];
        m[3] = tmp;
        tmp = m[2];
        m[2] = m[6];
        m[6] = tmp;
        tmp = m[5];
        m[5] = m[7];
        m[7] = tmp;
    }
    #[wasm_bindgen(js_name = getIdentity)]
    pub fn get_identity() -> Self {
        Mat3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
    }
}
