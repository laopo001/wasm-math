/**
 * File: c:\Users\35327\Githubs\ts-template\src\index.ts
 * Project: c:\Users\35327\Githubs\ts-template
 * Created Date: Friday, June 29th 2018, 12:01:19 am
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Friday, April 26th 2019, 8:10:18 pm
 * Modified By: liaodh
 * -----
 * Copyright (c) 2018 jiguang
 */

import {
    Node,
} from '../pkg/wasm_math.js';
import {
    SceneNode
} from 'hypergl';

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


