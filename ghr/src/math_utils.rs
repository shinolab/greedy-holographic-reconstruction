/*
 * File: vec_utils.rs
 * Project: src
 * Created Date: 07/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::Vector3;
use crate::{Complex, Float};

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
    sqrt(norm_sqr(v))
}

#[cfg(feature = "double")]
mod float {
    #[repr(C)]
    union float {
        f: f64,
        i: i64,
    }

    pub fn sqrt(x: f64) -> f64 {
        unsafe {
            let f = float { f: x };
            let tmp: i64 = 0x5fe6eb50c7b537a9i64 - (f.i >> 1);
            let xr = float { i: tmp };
            let x_half = 0.5 * x;
            let xr = xr.f * (1.5 - (x_half * xr.f * xr.f));
            let xr = xr * (1.5 - (x_half * xr * xr));
            xr * x
        }
    }
}

#[cfg(not(feature = "double"))]
mod float {
    #[repr(C)]
    union float {
        f: f32,
        i: i32,
    }

    pub fn sqrt(x: f32) -> f32 {
        unsafe {
            let x_half = 0.5 * x;
            let f = float { f: x };
            let tmp: i32 = 0x5f375a86i32 - (f.i >> 1);
            let xr = float { i: tmp };
            let xr = xr.f * (1.5 - (x_half * xr.f * xr.f));
            xr * x
        }
    }
}

pub use float::*;

pub fn c_norm(x: Complex) -> f64 {
    sqrt(x.norm_sqr())
}
