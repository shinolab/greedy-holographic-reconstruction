/*
 * File: buffer_type.rs
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

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum BufferType {
    AmplitudeFieldBuffer = 1,
    IntensityFieldBuffer = 2,
}

impl BufferType {
    pub fn from_i32(x: i32) -> Self {
        FromPrimitive::from_i32(x).unwrap()
    }
}
