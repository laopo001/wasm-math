/**
 * File: c:\Users\35327\Githubs\ts-template\src\index.ts
 * Project: c:\Users\35327\Githubs\ts-template
 * Created Date: Friday, June 29th 2018, 12:01:19 am
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Friday, April 19th 2019, 5:17:14 pm
 * Modified By: liaodh
 * -----
 * Copyright (c) 2018 jiguang
 */

import {
    Vec2,
    Mat4,
    Node,
    Vec3
} from '../pkg/wasm_math.js';
import {
    Read_ptr
} from './node';

let v = new Vec3(0, 0, 0);
console.log(v.data());
v.set(155, 1, 1)
console.log(v.data());

let node = new Node();
console.warn(node.get_local_position());

node.set_local_position(9, 9, 9);
node.add_child(new Node())
console.warn(node.get_local_position());
console.warn(node.get_child_ptr(0));


let m = Mat4.default();
console.log(m.data());
m.set(99, 99, 99, 99, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9);
console.log(m.data());



// let node2 = new Node();
// node.add_child(node2);

// console.log(node.local_position.data(), node2.get_parent());

// let ref = new Read_ptr(Node);
// console.log(node2.get_parent(), ref.read(node2.get_parent()).local_position.data())







