/*
 * File: axis_utils.rs
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

use ghr::buffer::Axis;

pub fn to_i32(axis: Axis) -> i32 {
    match axis {
        Axis::X => 0,
        Axis::Y => 1,
        Axis::Z => 2,
    }
}

pub fn from_i32(axis: i32) -> Axis {
    match axis {
        0 => Axis::X,
        1 => Axis::Y,
        2 => Axis::Z,
        _ => todo!(),
    }
}
