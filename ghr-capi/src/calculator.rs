/*
 * File: calculator.rs
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

use ghr::{calculator::*, wave_source::WaveSource};

use std::{ffi::c_void, mem::forget};

#[no_mangle]
pub unsafe extern "C" fn GHR_CreateCpuCalculator(out: *mut *mut c_void) {
    let mut calc = Box::new(CpuCalculator::new());
    let ptr = calc.as_mut() as *mut CpuCalculator;
    forget(calc);
    *out = ptr as *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn GHR_FreeCalculator(handle: *mut c_void) {
    let _calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_AddWaveSource(handle: *mut c_void, source: WaveSource) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    (*calc).add_wave_sources(&[source]);
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_InitWaveSources(handle: *mut c_void, size: u64) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    (*calc).init_wave_sources(size as usize);
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_WaveSources(handle: *mut c_void, out: *mut *mut c_void) -> u64 {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let sources = (*calc).wave_sources();
    let ptr = sources.as_ptr() as *mut WaveSource;
    let len = sources.len();
    forget(calc);
    *out = ptr as *mut c_void;
    len as u64
}
