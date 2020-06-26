/*
 * File: builder.rs
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

use ghr::buffer::generator::*;
use ghr::buffer::Axis;
use ghr::buffer::BufferBuilder;
use ghr::buffer::{AmplitudeFieldBuffer, IntensityFieldBuffer};

use std::ffi::c_void;
use std::mem::forget;

use super::axis_utils::from_i32;
use super::buffer_type::BufferType;

#[no_mangle]
pub unsafe extern "C" fn GHR_CreateBufferBuilder(out: *mut *mut BufferBuilder) {
    let mut builder = Box::new(BufferBuilder::new());
    let ptr = builder.as_mut() as *mut BufferBuilder;
    forget(builder);
    *out = ptr;
}

#[no_mangle]
pub unsafe extern "C" fn GHR_FreeBufferBuilder(handle: *mut BufferBuilder) {
    Box::from_raw(handle);
}

#[no_mangle]
pub unsafe extern "C" fn GHR_BufferBuilder_At(
    handle: *mut *mut BufferBuilder,
    axis: i32,
    pos: f32,
) {
    let builder = Box::from_raw(*handle);
    let mut builder = match from_i32(axis) {
        Axis::X => Box::new(builder.x_at(pos)),
        Axis::Y => Box::new(builder.y_at(pos)),
        Axis::Z => Box::new(builder.z_at(pos)),
    };
    let ptr = builder.as_mut() as *mut BufferBuilder;
    forget(builder);
    *handle = ptr;
}

#[no_mangle]
pub unsafe extern "C" fn GHR_BufferBuilder_Range(
    handle: *mut *mut BufferBuilder,
    axis: i32,
    min: f32,
    max: f32,
) {
    let builder = Box::from_raw(*handle);
    let mut builder = match from_i32(axis) {
        Axis::X => Box::new(builder.x_range(min, max)),
        Axis::Y => Box::new(builder.y_range(min, max)),
        Axis::Z => Box::new(builder.z_range(min, max)),
    };
    let ptr = builder.as_mut() as *mut BufferBuilder;
    forget(builder);
    *handle = ptr;
}

#[no_mangle]
pub unsafe extern "C" fn GHR_BufferBuilder_Resolution(
    handle: *mut *mut BufferBuilder,
    resolution: f32,
) {
    let builder = Box::from_raw(*handle);
    let mut builder = Box::new(builder.resolution(resolution));
    let ptr = builder.as_mut() as *mut BufferBuilder;
    forget(builder);
    *handle = ptr;
}

#[no_mangle]
pub unsafe extern "C" fn GHR_BufferBuilder_Generate(
    handle: *mut BufferBuilder,
    buffer_type: i32,
    out: *mut *mut c_void,
) {
    let builder = Box::from_raw(handle);
    match BufferType::from_i32(buffer_type) {
        BufferType::AmplitudeFieldBuffer => {
            let mut buffer = builder.generate::<Amplitude>();
            let ptr = buffer.as_mut() as *mut dyn AmplitudeFieldBuffer;
            forget(buffer);
            *out = Box::into_raw(Box::new(ptr)) as *mut c_void;
        }
        BufferType::IntensityFieldBuffer => {
            let mut buffer = builder.generate::<Intensity>();
            let ptr = buffer.as_mut() as *mut dyn IntensityFieldBuffer;
            forget(buffer);
            *out = Box::into_raw(Box::new(ptr)) as *mut c_void;
        }
    }
}
