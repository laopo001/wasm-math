extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use crate::mat4::Mat4;
use crate::quat::Quat;
use crate::vec3::Vec3;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[wasm_bindgen]
pub struct Node {
    local_position: Vec3,
    local_rotation: Quat,
    local_scale: Vec3,
    local_transform: Mat4,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

trait QRC<T: ?Sized> {
    fn get_mut(&mut self);
}

#[wasm_bindgen]
impl Node {
    pub fn new() -> Node {
        return Node {
            local_position: Vec3::default(),
            local_rotation: Quat::default(),
            local_scale: Vec3::default(),
            local_transform: Mat4::get_identity(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        };
    }
    pub fn new_rc() -> Rc<Node> {
        return Rc::new(Node::new());
    }
    pub fn add_child2(&self, child: Rc<Node>) {
        // *child.parent.borrow_mut() = Rc::downgrade(&Rc::new(*self));
        self.children.borrow_mut().push(child);
    }
    pub fn add_child(node: Rc<Node>, child: Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(&node);
        node.children.borrow_mut().push(child);
    }
    fn get_mut() {
        // *Rc::get_mut(&mut node).unwrap()
    }
    // pub fn into_rc_refcell(){
    //     return
    // }
}

#[test]
fn test() {
    let mut node = Node::new_rc();
    let node2 = Node::new_rc();
    Rc::get_mut(&mut node).unwrap().local_position = Vec3::new(1.0, 1.0, 1.0);
    // (*node).local_position = Vec3::new(1.0, 1.0, 1.0);
    // node.add_child2(node2.clone());
    Node::add_child(node.clone(), node2.clone());
    // // let children = node.children.borrow();
    // assert_eq!(
    //     node.children.borrow()[0].local_position.data(),
    //     Vec3::default().data()
    // );
    assert_eq!(node.local_position.data(), Vec3::default().data());
}
