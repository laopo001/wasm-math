extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use crate::mat4::Mat4;
use crate::quat::Quat;
use crate::vec3::Vec3;
#[allow(unused_imports)]
use std::cell::{RefCell, UnsafeCell};
#[allow(unused_imports)]
use std::ops::Deref;
#[allow(unused_imports)]
use std::rc::{Rc, Weak};

#[wasm_bindgen]
pub struct Node {
    pub(crate) local_position: Box<Vec3>,
    pub(crate) local_rotation: Box<Quat>,
    pub(crate) local_scale: Box<Vec3>,
    pub(crate) local_transform: UnsafeCell<Mat4>,
    pub(crate) world_position: Box<Vec3>,
    pub(crate) world_rotation: Box<Quat>,
    // pub(crate) world_scale: Box<Vec3>,
    pub(crate) world_transform: UnsafeCell<Mat4>,
    pub(crate) parent: *mut Node,
    pub(crate) children: Vec<*mut Node>,
    _dirty_local: bool,
    _dirty_world: bool,
    enabled: bool,
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Node {
        return Node {
            local_position: box Vec3::default(),
            local_rotation: box Quat::default(),
            local_scale: box Vec3::new(1.0, 1.0, 1.0),
            local_transform: UnsafeCell::new(Mat4::default()),
            world_position: box Vec3::default(),
            world_rotation: box Quat::default(),
            // world_scale: box Vec3::new(1.0, 1.0, 1.0),
            world_transform: UnsafeCell::new(Mat4::default()),
            parent: std::ptr::null_mut(),
            children: vec![],
            _dirty_local: false,
            _dirty_world: false,
            enabled: true,
        };
    }
    pub fn add_child(&mut self, child: &mut Node) {
        child.parent = self;
        self.children.push(child);
    }
    pub fn set_local_position(&mut self, x: f64, y: f64, z: f64) {
        self.local_position.set(x, y, z);
        if !self._dirty_local {
            self._dirtify(true);
        }
    }
    #[allow(dead_code)]
    fn get_local_position(&self) -> &Vec3 {
        self.local_position.as_ref()
    }

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        unsafe {
            if self.parent.is_null() {
                self.local_position.set(x, y, z);
            } else {
                let mut inv_parent_transform = (*(*self.parent).get_world_transform()).clone();
                inv_parent_transform.invert();
                inv_parent_transform
                    .transform_point(&Vec3::new(x, y, z), self.local_position.as_mut());
            }
            if !self._dirty_local {
                self._dirtify(true);
            }
        }

    }
    #[allow(dead_code)]
    fn get_position(&mut self) -> &Vec3 {
        unsafe {
            let world_transform_ptr = self.get_world_transform();
            let mut_vec3 = self.world_position.as_mut();
            (*world_transform_ptr).get_translate(mut_vec3);
        }
        return self.world_position.as_ref();
    }
    pub fn get_position_data(&self) -> Box<[f64]> {
        self.local_position.data()
    }
    fn get_rotation(&mut self) -> &Quat {
        unsafe {
            let world_transform_ptr = self.get_world_transform();
            self.world_rotation.set_from_mat4(&*world_transform_ptr);
            return self.world_rotation.as_ref();
        }
    }

    pub fn set_euler_angles(&mut self, x: f64, y: f64, z: f64) {
        self.local_rotation.set_from_euler_angles(x, y, z);
        unsafe {
            if self.parent.is_null() {
                let mut inv_parent_rotation = (*self.parent).get_rotation().clone();
                inv_parent_rotation.invert();
                self.local_rotation.as_mut().mul(&inv_parent_rotation);
            }
            if !self._dirty_local {
                self._dirtify(true);
            }
        }

    }
    pub fn set_local_euler_angles(&mut self, x: f64, y: f64, z: f64) {
        self.local_rotation.set_from_euler_angles(x, y, z);
        if !self._dirty_local {
            self._dirtify(true);
        }
    }
    #[allow(dead_code)]
    fn get_world_transform(&mut self) -> *mut Mat4 {
        if self._dirty_local == false && self._dirty_world == false {
            return self.world_transform.get();
        }
        if !self.parent.is_null() {
            unsafe {
                (*self.parent).get_world_transform();
            }
        }
        self._sync();
        return self.world_transform.get();
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
    #[allow(dead_code, unused_parens)]
    fn sync_hierarchy(&mut self) {
        if (!self.enabled) {
            return;
        }
        if self._dirty_local || self._dirty_world {
            self._sync();
        }
        for i in 0..(self.children.len()) {
            unsafe {
                (*self.children[i]).sync_hierarchy();
            }
        }
    }
    pub fn _sync(&mut self) {
        let world_transform_ptr = self.world_transform.get();
        let local_transform_ptr = self.local_transform.get();
        unsafe {
            if self._dirty_local {
                (*local_transform_ptr).set_from_trs(
                    &self.local_position,
                    &self.local_rotation,
                    &self.local_scale,
                );
                self._dirty_local = false;
            }
            if self._dirty_world {

                if self.parent.is_null() {
                    (*world_transform_ptr).copy(&*local_transform_ptr);
                } else {

                    let parent_world_transform_ptr = (*self.parent).world_transform.get();
                    println!("local_transform_ptr---{:?}", *local_transform_ptr);
                    println!("world_transform_ptr--{:?}", *world_transform_ptr);
                    println!(
                        "parent_world_transform_ptr---{:?}",
                        *parent_world_transform_ptr
                    );

                    (*world_transform_ptr)
                        .mul2(&*parent_world_transform_ptr, &*local_transform_ptr);
                }
                self._dirty_world = false;
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


#[test]
fn test_set_get_position() {
    let mut node = Node::new();
    node.set_position(1.0, 2.0, 3.0);
    assert_eq!(node.get_position().data(), Vec3::new(1.0, 2.0, 3.0).data());
}

#[test]
fn test_set_get_local_position() {
    let mut node = Node::new();
    node.set_local_position(1.0, 2.0, 3.0);
    assert_eq!(
        node.get_local_position().data(),
        Vec3::new(1.0, 2.0, 3.0).data()
    );
}

#[test]
fn test_child_set_get_position() {
    let mut node = Node::new();
    let mut child = Node::new();
    let mut grandson = Node::new();
    node.add_child(&mut child);
    child.add_child(&mut grandson);
    node.set_local_position(1.0, 2.0, 3.0);
    child.set_local_position(1.0, 2.0, 3.0);
    grandson.set_local_position(1.0, 2.0, 3.0);
    assert_eq!(
        grandson.get_position().data(),
        Vec3::new(3.0, 6.0, 9.0).data()
    );
}