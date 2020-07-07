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
pub mod vec_utils;
pub mod wave_source;

pub type Float = f32;
pub type Vector3 = [Float; 3];
