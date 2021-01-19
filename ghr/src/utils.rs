/*
 * File: utils.rs
 * Project: src
 * Created Date: 01/01/1970
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

#[cfg(feature = "cache")]
mod cache {
    use crate::{consts::WAVE_NUMBER, math_utils::*, Complex, Float, Vector3, PI};
    use once_cell::sync::Lazy;

    const PHASE_CACHE_SIZE: usize = 1000;
    const DIST_CACHE_SIZE: usize = 1000000;
    const DIST_CACHE_STEP: Float = 1.0;
    const DIST_CACHE_STEP_INV: Float = 1.0 / DIST_CACHE_STEP;
    const INV_2_PI: Float = PHASE_CACHE_SIZE as Float / (2.0 * PI);

    static PHASE_CACHE: Lazy<Vec<Complex>> = Lazy::new(|| {
        (0..PHASE_CACHE_SIZE)
            .map(|p| Complex::new(0.0, 2.0 * PI * (p as Float / PHASE_CACHE_SIZE as Float)).exp())
            .collect()
    });

    static DIST_CACHE: Lazy<Vec<Complex>> = Lazy::new(|| {
        (0..DIST_CACHE_SIZE)
            .map(|d| {
                let dist = (d as Float).sqrt() * DIST_CACHE_STEP;
                1.0 / dist * (Complex::new(0., WAVE_NUMBER * dist)).exp()
            })
            .collect()
    });

    pub fn transfer(trans_pos: Vector3, target_pos: Vector3, amp: Float, phase: Float) -> Complex {
        let diff = sub(target_pos, trans_pos);
        let dist = norm_sqr(diff);
        amp * DIST_CACHE[(dist * DIST_CACHE_STEP_INV) as usize]
            * PHASE_CACHE[(phase * INV_2_PI) as usize]
    }
}

#[cfg(not(feature = "cache"))]
mod cache {
    use crate::{consts::WAVE_NUMBER, math_utils::*, Complex, Float, Vector3};

    pub fn transfer(trans_pos: Vector3, target_pos: Vector3, amp: Float, phase: Float) -> Complex {
        let diff = sub(target_pos, trans_pos);
        let dist = norm(diff);
        amp / dist * (Complex::new(0., phase + WAVE_NUMBER * dist)).exp()
    }
}

pub use cache::transfer;
