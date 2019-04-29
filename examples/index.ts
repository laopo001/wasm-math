/**
 * File: c:\Users\35327\Githubs\ts-template\src\index.ts
 * Project: c:\Users\35327\Githubs\ts-template
 * Created Date: Friday, June 29th 2018, 12:01:19 am
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Monday, April 29th 2019, 4:39:23 pm
 * Modified By: liaodh
 * -----
 * Copyright (c) 2018 jiguang
 */

import {
    Node, Mat4
} from '../pkg/wasm_math.js';
import {
    SceneNode
} from 'hypergl';

let m1 = new Mat4(9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
let m2 = new Mat4(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
m2.copy(m1);
console.log(m2.data());



let count = 10000;
function test1() {
    console.time('test1');
    let node = new Node();
    let child = new Node();
    node.addChild(child);
    for (let i = 0; i < count; i++) {
        let temp = new Node();
        temp.setLocalPosition(1, 1, 1);
        child.addChild(temp);
        child = temp;
    }
    console.log(child.getPositionData());

    console.timeEnd('test1');
}

test1();

function test2() {
    console.time('test2');
    let node = new SceneNode();
    let child = new SceneNode();
    node.addChild(child);
    for (let i = 0; i < count; i++) {
        let temp = new SceneNode();
        temp.setLocalPosition(1, 1, 1);
        child.addChild(temp);
        child = temp;
    }
    console.log(child.getPosition().data);
    console.timeEnd('test2');
}

test2();


