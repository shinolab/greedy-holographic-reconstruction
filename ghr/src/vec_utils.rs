/*
 * File: vec_utils.rs
 * Project: src
 * Created Date: 07/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::Float;
use crate::Vector3;

pub fn zero() -> Vector3 {
    [0., 0., 0.]
}

pub fn add(x: Vector3, y: Vector3) -> Vector3 {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2]]
}

pub fn sub(x: Vector3, y: Vector3) -> Vector3 {
    [x[0] - y[0], x[1] - y[1], x[2] - y[2]]
}

pub fn norm_sqr(v: Vector3) -> Float {
    let x = v[0];
    let y = v[1];
    let z = v[2];
    x * x + y * y + z * z
}

pub fn norm(v: Vector3) -> Float {
    norm_sqr(v).sqrt()
}
