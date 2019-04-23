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
    pub(crate) local_position: Box<Vec3>,
    pub(crate) local_rotation: Box<Quat>,
    pub(crate) local_scale: Box<Vec3>,
    pub(crate) local_transform: Box<Mat4>,
    pub(crate) world_position: Box<Vec3>,
    pub(crate) world_rotation: Box<Quat>,
    pub(crate) world_scale: Box<Vec3>,
    pub(crate) world_transform: Box<Mat4>,
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
            local_position: box Vec3::default(),
            local_rotation:box Quat::default(),
            local_scale:box Vec3::default(),
            local_transform:box Mat4::default(),
            world_position:box Vec3::default(),
            world_rotation:box Quat::default(),
            world_scale:box Vec3::default(),
            world_transform:box Mat4::default(),
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
    pub fn get_local_position_data(&self) -> Box<[f64]> {
        self.local_position.data()
    }
    #[allow(dead_code)]
    fn get_local_position(&self) -> Vec3 {
        *self.local_position
    }
    pub fn set_world_position(&mut self, x: f64, y: f64, z: f64) {}
    pub fn get_world_positon(&mut self) -> Vec3 {
        
        let world_transform = self.get_world_transform();
        let mut_vec3 = self.world_position.as_mut();
        world_transform.get_translate(mut_vec3);
        return *self.world_position;
    }
    #[allow(dead_code)]
    fn get_world_transform(&mut self) -> Box<Mat4> {
        if self._dirty_local == false && self._dirty_world == false {
            return self.world_transform;
        }
        self._sync();
        return self.world_transform;
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
            for item in self.children.iter() {
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
                unsafe {
                    self.world_transform
                        .mul2(&(*self.parent).world_transform, &self.local_transform);
                }
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
