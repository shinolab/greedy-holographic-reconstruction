/*
 * File: lib.rs
 * Project: src
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

#[macro_use]
extern crate itertools;

pub mod buffer;
pub mod calculator;
pub mod optimizer;
pub mod utils;
pub mod vec_utils;
pub mod wave_source;

#[cfg(feature = "double")]
mod float {
    use ndarray_linalg::*;

    /// Floating-point number
    pub type Float = f64;
    pub type Complex = c64;

    pub const PI: Float = std::f64::consts::PI;
}

#[cfg(not(feature = "double"))]
mod float {
    use ndarray_linalg::*;

    /// Floating-point number
    pub type Float = f32;
    pub type Complex = c32;
    pub const PI: Float = std::f32::consts::PI;
}

pub use float::*;
pub type Vector3 = [Float; 3];
