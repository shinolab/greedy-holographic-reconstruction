/*
 * File: optimizer.rs
 * Project: src
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use std::{ffi::c_void, mem::forget};

use ghr::{calculator::*, optimizer::*, Float, Vector3};

#[no_mangle]
pub unsafe extern "C" fn GHR_GreedyBruteForce(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const Float,
    size: u64,
    phase_div: u64,
    amp_div: u64,
    power_opt: bool,
    randamize: bool,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let mut gfs = GreedyBruteForce::new(phase_div as _, amp_div as _, power_opt, randamize);
    gfs.set_target_foci(foci);
    gfs.set_target_amps(amps);
    gfs.optimize((*calc).wave_sources());
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_Horn(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const Float,
    size: u64,
    repeat: u64,
    alpha: Float,
    lambda: Float,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let mut horn = Horn::new(repeat as _, alpha, lambda);
    horn.set_target_foci(foci);
    horn.set_target_amps(amps);
    horn.optimize((*calc).wave_sources());
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_Long(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const Float,
    size: u64,
    gamma: Float,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let mut long = Long::new(gamma);
    long.set_target_foci(foci);
    long.set_target_amps(amps);
    long.optimize((*calc).wave_sources());
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_LM(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const Float,
    size: u64,
    eps_1: Float,
    eps_2: Float,
    tau: Float,
    repeat: u64,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let mut lm = LM::new(eps_1, eps_2, tau, repeat as _);
    lm.set_target_foci(foci);
    lm.set_target_amps(amps);
    lm.optimize((*calc).wave_sources());
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_GSPAT(
    handle: *mut c_void,
    foci: *const c_void,
    amps: *const Float,
    size: u64,
    repeat: u64,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let len = size as usize;
    let foci = std::slice::from_raw_parts(foci as *mut Vector3, len);
    let amps = std::slice::from_raw_parts(amps, len);
    let mut gspat = GSPAT::new(repeat as _);
    gspat.set_target_foci(foci);
    gspat.set_target_amps(amps);
    gspat.optimize((*calc).wave_sources());
    forget(calc);
}
