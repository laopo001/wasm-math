/**
 * File: c:\Users\35327\Githubs\ts-template\src\index.ts
 * Project: c:\Users\35327\Githubs\ts-template
 * Created Date: Friday, June 29th 2018, 12:01:19 am
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Thursday, January 24th 2019, 5:48:46 pm
 * Modified By: liaodh
 * -----
 * Copyright (c) 2018 jiguang
 */

import { Vec2, Mat4 } from '../pkg/wasm_math.js';

let vec2 = new Vec2(0, 0);
let data = vec2.data();
console.log(data)
data[0]=1;
console.log(vec2,data,vec2.data())

let m = new Mat4(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
let m2 = new Mat4(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
m.add(m2)
console.log(
    m.data()
)


console.time('1')
for (let i = 0; i < 100000; i++) {
    let vec2 = new Vec2(0, 0);
    vec2.data()
}
console.timeEnd('1')
console.timeStamp()
