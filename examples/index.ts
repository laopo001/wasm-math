/**
 * File: c:\Users\35327\Githubs\ts-template\src\index.ts
 * Project: c:\Users\35327\Githubs\ts-template
 * Created Date: Friday, June 29th 2018, 12:01:19 am
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Thursday, January 24th 2019, 1:29:19 pm
 * Modified By: liaodh
 * -----
 * Copyright (c) 2018 jiguang
 */

import { Vec2 } from '../pkg/wasm_math.js';

let vec2 = new Vec2(0, 0);

console.time('1')
for (let i = 0; i < 100000; i++) {
    let vec2 = new Vec2(0, 0);
    vec2.data()
}
console.timeEnd('1')
console.timeStamp()
