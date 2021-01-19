/*
 * File: calculator.rs
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

use ghr::{calculator::*, wave_source::WaveSource, Float};

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

#[no_mangle]
pub unsafe extern "C" fn GHR_SetWaveSourceProps(
    handle: *mut c_void,
    i: u64,
    x: Float,
    y: Float,
    z: Float,
    amp: Float,
    phase: Float,
) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let sources = (*calc).wave_sources();
    let idx = i as usize;
    sources[idx].pos = [x, y, z];
    sources[idx].amp = amp;
    sources[idx].phase = phase;
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_SetWaveSourceAmp(handle: *mut c_void, i: u64, amp: Float) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let sources = (*calc).wave_sources();
    sources[i as usize].amp = amp;
    forget(calc);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_SetWaveSourcePhase(handle: *mut c_void, i: u64, phase: Float) {
    let mut calc: Box<CpuCalculator> = Box::from_raw(handle as *mut _);
    let sources = (*calc).wave_sources();
    sources[i as usize].phase = phase;
    forget(calc);
}
