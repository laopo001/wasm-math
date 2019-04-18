extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use crate::mat4::Mat4;
use crate::quat::Quat;
use crate::vec3::Vec3;
#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::ops::Deref;
#[allow(unused_imports)]
use std::rc::{Rc, Weak};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Node {
    pub(crate) local_position: Vec3,
    pub(crate) local_rotation: Quat,
    pub(crate) local_scale: Vec3,
    pub(crate) local_transform: Mat4,
    pub(crate) world_transform: Mat4,
    pub(crate) parent: *mut Node,
    pub(crate) children: Vec<*mut Node>,
    _dirty_local: bool,
    _dirty_world: bool,
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
            world_transform: Mat4::default(),
            parent: std::ptr::null_mut(),
            children: vec![],
            _dirty_local: false,
            _dirty_world: false,
        };
    }
    pub fn add_child(&mut self, child: &mut Node) {
        child.parent = self;
        self.children.push(child);
    }
    pub fn get_parent_ptr(&self) -> *mut Node {
        return self.parent;
    }
    pub fn get_child_ptr(&self, index: usize) -> *mut Node {
        return self.children[index];
    }
    pub fn set_local_position(&mut self, x: f64, y: f64, z: f64) {
        self.local_position.set(x, y, z);
        if !self._dirty_local {
            self._dirtify(true);
        }
    }
    pub fn get_local_position(&self) -> Box<[f64]> {
        self.local_position.data()
    }
    pub fn _dirtify(&mut self, local: bool) {
        if self._dirty_local && self._dirty_world {
            return;
        }
        if local {
            self._dirty_local = true;
        }
        if !self._dirty_world {
            self._dirty_world = true;
            for item in self.children.iter_mut() {
                unsafe {
                    (**item)._dirtify(true);
                }
            }
        }
    }
    pub fn _sync(&mut self) {
        if self._dirty_local {
            self.local_transform.set_from_trs(
                &self.local_position,
                &self.local_rotation,
                &self.local_scale,
            );
            self._dirty_local = false;
        }
        if self._dirty_world {
            if self.parent.is_null() {
                self.world_transform.copy(&self.local_transform);
            } else {
                //
            }
        }
    }
}

#[test]
fn test() {
    let mut node = Node::new();
    node.local_position.set(1.0, 2.0, 3.0);
    assert_eq!(node.local_position.data(), Vec3::new(1.0, 2.0, 3.0).data());
}
