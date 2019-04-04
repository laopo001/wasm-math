extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use crate::mat4::Mat4;
use crate::quat::Quat;
use crate::vec3::Vec3;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Node {
    pub local_position: Vec3,
    pub local_rotation: Quat,
    pub local_scale: Vec3,
    pub(crate) local_transform: Mat4,
    pub(crate) parent: *mut Node,
    pub(crate) children: Vec<*mut Node>,
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Node {
        return Node {
            local_position: Vec3::default(),
            local_rotation: Quat::default(),
            local_scale: Vec3::default(),
            local_transform: Mat4::default(),
            parent: std::ptr::null_mut(),
            children: vec![],
        };
    }
    pub fn add_child(&mut self, child: &mut Node) {
        child.parent = self;
        self.children.push(child);
    }
    pub fn get_parent(&self) -> *mut Node {
        return self.parent;
    }
    pub fn get_child(&self, index: usize) -> *mut Node {
        return self.children[index];
    }
}

#[test]
fn test() {
    let mut node = Node::new();
    node.local_position.set(1.0, 2.0, 3.0);
    unsafe {
        assert_eq!(node.local_position.data(), Vec3::new(1.0, 2.0, 3.0).data());
    }
}
