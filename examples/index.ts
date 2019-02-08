/**
 * File: c:\Users\35327\Githubs\ts-template\src\index.ts
 * Project: c:\Users\35327\Githubs\ts-template
 * Created Date: Friday, June 29th 2018, 12:01:19 am
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Friday, February 8th 2019, 10:18:31 pm
 * Modified By: liaodh
 * -----
 * Copyright (c) 2018 jiguang
 */

import { Vec2, Mat4, Matrix } from '../pkg/wasm_math.js';

let vec2 = new Vec2(0, 0);
let data = vec2.data();
console.log(data)
data[0] = 1;
console.log(vec2, data, vec2.data())

let m = Mat4.get_identity();
let m2 = Mat4.get_identity();
m.invert()

console.log(
    m.data(), m2.data(), m.equals(m2)
)
console.dir(Mat4)


console.time('1')
for (let i = 0; i < 100000; i++) {
    let vec2 = new Vec2(0, 0);
    vec2.data()
}
console.timeEnd('1')
console.timeStamp()

console.log('---------------------------')

let m1 = new Matrix(0, 0, 0);
console.log(m1.data());

