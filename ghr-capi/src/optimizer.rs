/*
 * File: optimizer.rs
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

use std::ffi::c_void;
use std::mem::forget;

use ghr::calculator::*;
use ghr::optimizer::*;
use ghr::Vector3;

#[no_mangle]
pub unsafe extern "C" fn GHR_GreedyFullSearch(
    handle: *mut c_void,
    points: *const f32,
    size: u64,
    division: u64,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let points = std::slice::from_raw_parts(points as *mut Vector3, len);

    let gfs = GreedyFullSearch::new(division as usize);
    gfs.maximize(&mut *calc, points, |c| c.norm());

    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_Horn(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f32,
    size: u64,
    wave_len: f64,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let horn = Horn::new(foci.to_vec(), amps.to_vec(), wave_len);
    horn.optimize((*calc).wave_sources());
    forget(calc);
}
