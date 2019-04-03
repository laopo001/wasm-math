/**
 * File: c:\Users\35327\Githubs\wasm-math\examples\node.ts
 * Project: c:\Users\35327\Githubs\wasm-math
 * Created Date: Wednesday, April 3rd 2019, 11:39:32 pm
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Thursday, April 4th 2019, 1:10:43 am
 * Modified By: liaodh
 * -----
 * Copyright (c) 2019 liaodh
 */


import { Node as INode } from '../pkg/wasm_math';

export type Constructor<T> = new (...args) => T;
export class Read_ptr<T> {
    instance: T;
    constructor(c: Constructor<T>) {
        this.instance = new c();
    }
    read(ptr: number): T {
        this.instance['ptr'] = ptr;
        return this.instance;
    }
}
