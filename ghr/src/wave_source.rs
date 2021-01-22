/*
 * File: wave_source.rs
 * Project: src
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{math_utils::zero, Complex, Float, Vector3};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WaveSource {
    pub pos: Vector3,
    pub amp: Float,
    pub phase: Complex,
}

impl WaveSource {
    pub fn new(pos: Vector3, amp: Float, phase: Complex) -> Self {
        Self { pos, amp, phase }
    }
}

impl std::default::Default for WaveSource {
    fn default() -> Self {
        Self::new(zero(), 0., Complex::new(0.,0.))
    }
}
