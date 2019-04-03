/**
 * File: c:\Users\35327\Githubs\ts-template\src\index.ts
 * Project: c:\Users\35327\Githubs\ts-template
 * Created Date: Friday, June 29th 2018, 12:01:19 am
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Thursday, April 4th 2019, 1:41:30 am
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
console.error(node.local_position);
node.local_position.set(155, 155, 155);
console.error(node.local_position);
// let node2 = new Node();
// node.add_child(node2);

// console.log(node.local_position.data(), node2.get_parent());

// let ref = new Read_ptr(Node);
// console.log(node2.get_parent(), ref.read(node2.get_parent()).local_position.data())







