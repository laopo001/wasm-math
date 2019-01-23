/**
 * File: c:\Users\35327\Githubs\ts-template\src\index.ts
 * Project: c:\Users\35327\Githubs\ts-template
 * Created Date: Friday, June 29th 2018, 12:01:19 am
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Wednesday, January 23rd 2019, 7:14:35 pm
 * Modified By: liaodh
 * -----
 * Copyright (c) 2018 jiguang
 */

import * as wasm from '../pkg/wasm_math.js';

let module = wasm as any;
let vec2 = new module.Vec2();
// vec2.x = 1.0;

console.log(vec2)

