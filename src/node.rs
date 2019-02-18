extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use crate::mat4::Mat4;
use crate::quat::Quat;
use crate::vec3::Vec3;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Node {
    local_position: Vec3,
    local_rotation: Quat,
    local_scale: Vec3,
    local_transform: Mat4,
    pub(crate) parent: *mut Node,
    pub(crate) children: Vec<*mut Node>,
}

#[wasm_bindgen]
impl Node {
    pub fn new() -> Node {
        return Node {
            local_position: Vec3::default(),
            local_rotation: Quat::default(),
            local_scale: Vec3::default(),
            local_transform: Mat4::get_identity(),
            parent: std::ptr::null_mut(),
            children: vec![],
        };
    }
    // pub fn new_rc() -> Rc<Node> {
    //     return Rc::new(Node::new());
    // }
    pub fn add_child(&mut self, child: &mut Node) {
        child.parent = child;
        self.children.push(child);
    }
}

#[test]
fn test() {
    let mut node = Node::new();
    let mut node2 = Node::new();
    node.add_child(&mut node2);
    node.local_position = Vec3::new(1.0, 1.0, 1.0);

    unsafe {
        assert_eq!(
            (*node.children[0]).local_position.data(),
            Vec3::default().data()
        );
    }
}
