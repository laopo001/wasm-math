extern crate wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use crate::mat4::Mat4;
use crate::quat::Quat;
use crate::vec3::Vec3;
use core::mem;
use core::ptr::NonNull;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Coords = { "latitude": number, "longitude": number, }; 
"#;

// type Link = Rc<RefCell<Node>>;
// type WeakLink = Weak<RefCell<Node>>;

pub struct Node {
    pub local_position: Vec3,
    pub local_rotation: Quat,
    pub local_scale: Vec3,
    local_transform: Mat4,
    pub(crate) parent: Option<NonNull<Node>>,
    pub(crate) children: Vec<NonNull<Node>>,
}

impl Node {
    pub fn new() -> Node {
        return Node {
            local_position: Vec3::default(),
            local_rotation: Quat::default(),
            local_scale: Vec3::default(),
            local_transform: Mat4::get_identity(),
            parent: None,
            children: vec![],
        };
    }
    pub fn addChild(&mut self, mut child: Box<Node>) {
        // child.parent = Some(NonNull::from(self));
        self.children.push(Box::into_raw_non_null(child));
    }
    pub fn setParent() {}
}
// impl Node {
//     pub fn new() -> Node {
//         return Node {
//             local_position: Vec3::default(),
//             local_rotation: Quat::default(),
//             local_scale: Vec3::default(),
//             local_transform: Mat4::get_identity(),
//             parent: None,
//             children: vec![],
//         };
//     }
//     pub fn new_rc() -> Rc<Node> {
//         return Rc::new(Node::new());
//     }
//     pub fn add_child(&mut self, child: Link) {
//         // child.parent = Some(Rc::downgrade(child));
//         self.children.push(child);
//     }
//     // pub fn get_parent_ptr(&self) -> *const Node {
//     //     return self.parent;
//     // }
//     // pub fn get_mut_parent_ptr(&mut self) -> *mut Node {
//     //     return self.parent;
//     // }

//     // pub fn get_child_ptr(&self, index: usize) -> *const Node {
//     //     return self.children[index];
//     // }
// }

// #[test]
// fn test() {
//     let mut node = Node::new();
//     let mut node2 = Node::new();
//     node.add_child(&mut node2);
//     node.local_position = Vec3::new(1.0, 1.0, 1.0);

//     // unsafe {
//     //     assert_eq!(
//     //         (*node.children[0]).local_position.data(),
//     //         Vec3::default().data()
//     //     );
//     // }
// }