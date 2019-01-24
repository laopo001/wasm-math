use wasm_math::mat4::Mat4;
use std::rc::Rc;
use std::cell::RefCell;

fn main (){
    let mut mat1 = Mat4::new(
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
    );
    let mut mat2 = Mat4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
    );
    mat1.add(&mat2);
    let a = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut b = a;
    b[0]=1.0;

    let c = Rc::new(RefCell::new(b));
//    let d= c.as_mut_ptr();
//    d[0]=1.0;
    println!("{:p}", &a);
    println!("{:p}", &b);
    println!("{:?}", a[0]);
    println!("{:?}", b[0]);
//    println!("{:?}", c[0]);
}