/*
 * File: wave_source.rs
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

use crate::vec_utils::zero;
use crate::Vector3;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WaveSource {
    pub pos: Vector3,
    pub amp: f32,
    pub phase: f32,
}

impl WaveSource {
    pub fn new(pos: Vector3, amp: f32, phase: f32) -> Self {
        Self { pos, amp, phase }
    }
}

impl std::default::Default for WaveSource {
    fn default() -> Self {
        Self::new(zero(), 0., 0.)
    }
}
