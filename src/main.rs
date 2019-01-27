use std::cell::RefCell;
use std::rc::Rc;
use wasm_math::mat4::Mat4;
use wasm_math::quat::Quat;
use wasm_math::vec3::Vec3;

fn main() {
    let mut mat1 = Mat4::new(
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    let mat2 = Mat4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    mat1.add(&mat2);
    println!("{:?}", mat1.data()[0]);

    let a = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    let mut b = a;
    b[0] = 1.0;
    println!("{:p}", &a);
    println!("{:p}", &b);
    println!("{:?}", a[0]);
    println!("{:?}", b[0]);
    println!("{:?}", "----------------");
    let c = vec![
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    test(&c);
    println!("{:?}", c[0]);
    let mut q = Quat::default();
    q.setFromEulerAngles(0.0, 90.0, 90.0);
    println!("{:?}", q.data());
    let v = Vec3::new(1.0, 0.0, 0.0);
    let mut res = Vec3::default();
    q.transformVector(&v, &mut res);
    println!("{:?}", res.data());
}

fn test(v: &Vec<f64>) {}
