/*
 * File: lib.rs
 * Project: src
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/06/2020
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
pub mod wave_source;

pub type Vector3 = na::Vector3<f32>;
