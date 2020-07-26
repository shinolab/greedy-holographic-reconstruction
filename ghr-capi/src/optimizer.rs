/*
 * File: optimizer.rs
 * Project: src
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/07/2020
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
pub unsafe extern "C" fn GHR_GreedyBruteForce(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f64,
    size: u64,
    wave_len: f64,
    include_amp: bool,
    normalize: bool,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);

    let gfs = GreedyBruteForce::new(foci.to_vec(), amps.to_vec(), wave_len);
    gfs.optimize((*calc).wave_sources(), include_amp, normalize);

    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_Horn(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f64,
    size: u64,
    wave_len: f64,
    include_amp: bool,
    normalize: bool,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let horn = Horn::new(foci.to_vec(), amps.to_vec(), wave_len);
    horn.optimize((*calc).wave_sources(), include_amp, normalize);
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_Long(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f64,
    size: u64,
    wave_len: f64,
    include_amp: bool,
    normalize: bool,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let long = Long::new(foci.to_vec(), amps.to_vec(), wave_len);
    long.optimize((*calc).wave_sources(), include_amp, normalize);
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_LM(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f64,
    size: u64,
    wave_len: f64,
    include_amp: bool,
    normalize: bool,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let lm = LM::new(foci.to_vec(), amps.to_vec(), wave_len);
    lm.optimize((*calc).wave_sources(), include_amp, normalize);
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_GD(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const f64,
    size: u64,
    wave_len: f64,
    include_amp: bool,
    normalize: bool,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let gd = GD::new(foci.to_vec(), amps.to_vec(), wave_len);
    gd.optimize((*calc).wave_sources(), include_amp, normalize);
    forget(calc);
}
