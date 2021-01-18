/*
 * File: buffer.rs
 * Project: src
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use ghr::{
    buffer::{AmplitudeFieldBuffer, Dimension, IntensityFieldBuffer},
    calculator::*,
    Float,
};

use std::ffi::c_void;
use std::mem::forget;

use super::axis_utils::to_i32;
use super::buffer_type::BufferType;

#[no_mangle]
pub unsafe extern "C" fn GHR_FreeBuffer(handle: *mut c_void) {
    Box::from_raw(handle);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_GetScalarBufferArray(
    handle: *mut c_void,
    out: *mut *const c_void,
    buffer_type: i32,
) -> u64 {
    macro_rules! get_scalar {
        ($trait: ident) => {{
            let buffer: Box<Box<dyn $trait>> = Box::from_raw(handle as *mut _);
            let array = buffer.buffer();
            let len = array.len();
            let ptr = array.as_ptr() as *const Float;
            forget(buffer);
            *out = ptr as *const c_void;
            len as u64
        }};
    };
    match BufferType::from_i32(buffer_type) {
        BufferType::AmplitudeFieldBuffer => get_scalar!(AmplitudeFieldBuffer),
        BufferType::IntensityFieldBuffer => get_scalar!(IntensityFieldBuffer),
    }
}

#[no_mangle]
pub unsafe extern "C" fn GHR_GetScalarMax(handle: *mut c_void, buffer_type: i32) -> Float {
    macro_rules! get_max {
        ($trait: ident) => {{
            let buffer: Box<Box<dyn $trait>> = Box::from_raw(handle as *mut _);
            let max = buffer.max();
            forget(buffer);
            max
        }};
    };
    match BufferType::from_i32(buffer_type) {
        BufferType::AmplitudeFieldBuffer => get_max!(AmplitudeFieldBuffer),
        BufferType::IntensityFieldBuffer => get_max!(IntensityFieldBuffer),
    }
}

#[no_mangle]
pub unsafe extern "C" fn GHR_GetBounds(
    handle: *mut c_void,
    buffer_type: i32,
    x: *mut u64,
    y: *mut u64,
    z: *mut u64,
) {
    macro_rules! get_bounds {
        ($trait: ident) => {{
            let buffer: Box<Box<dyn $trait>> = Box::from_raw(handle as *mut _);
            let bounds = buffer.bounds();
            forget(buffer);
            bounds
        }};
    };
    let bounds = match BufferType::from_i32(buffer_type) {
        BufferType::AmplitudeFieldBuffer => get_bounds!(AmplitudeFieldBuffer),
        BufferType::IntensityFieldBuffer => get_bounds!(IntensityFieldBuffer),
    };
    *x = bounds.x() as u64;
    *y = bounds.y() as u64;
    *z = bounds.z() as u64;
}

#[no_mangle]
pub unsafe extern "C" fn GHR_GetDimension(
    handle: *mut c_void,
    buffer_type: i32,
    first: *mut i32,
    second: *mut i32,
    third: *mut i32,
) {
    macro_rules! get_dimension {
        ($trait: ident) => {{
            let buffer: Box<Box<dyn $trait>> = Box::from_raw(handle as *mut _);
            let dimension = buffer.dimension();
            forget(buffer);
            dimension
        }};
    };
    let dimension = match BufferType::from_i32(buffer_type) {
        BufferType::AmplitudeFieldBuffer => get_dimension!(AmplitudeFieldBuffer),
        BufferType::IntensityFieldBuffer => get_dimension!(IntensityFieldBuffer),
    };

    *first = -1;
    *second = -1;
    *third = -1;

    match dimension {
        Dimension::None => (),
        Dimension::One(f) => *first = to_i32(f),
        Dimension::Two(f, s) => {
            *first = to_i32(f);
            *second = to_i32(s);
        }
        Dimension::Three(f, s, t) => {
            *first = to_i32(f);
            *second = to_i32(s);
            *third = to_i32(t);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn GHR_Calculate(
    calculator: *mut c_void,
    buffer: *mut c_void,
    buffer_type: i32,
) {
    let calc: Box<CpuCalculator> = Box::from_raw(calculator as *mut _);
    match BufferType::from_i32(buffer_type) {
        BufferType::AmplitudeFieldBuffer => {
            let mut buffer: Box<Box<dyn AmplitudeFieldBuffer>> = Box::from_raw(buffer as *mut _);
            (*buffer).calculate(&*calc);
            forget(buffer);
        }
        BufferType::IntensityFieldBuffer => {
            let mut buffer: Box<Box<dyn IntensityFieldBuffer>> = Box::from_raw(buffer as *mut _);
            (*buffer).calculate(&*calc);
            forget(buffer);
        }
    }

    forget(calc);
}
