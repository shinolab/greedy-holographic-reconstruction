/*
 * File: generator.rs
 * Project: buffer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::{
    amplitude_field_buffer::AmplitudeFieldBuffer,
    bounds::Bounds,
    dimension::{Axis, Dimension},
    intensity_field_buffer::IntensityFieldBuffer,
    scalar_filed_buffer::*,
};
use crate::Float;

pub trait Generator {
    type Output;
    fn generate(
        dimension: Dimension,
        x_range: (Float, Float),
        y_range: (Float, Float),
        z_range: (Float, Float),
        r: Float,
    ) -> Self::Output;
}

macro_rules! scalar_gen {
    ($dimension: ident, $x_range: ident,$y_range: ident,$z_range: ident, $r: ident, $trait: ident) => {{
        let nx = (($x_range.1 - $x_range.0) / $r) as usize + 1;
        let ny = (($y_range.1 - $y_range.0) / $r) as usize + 1;
        let nz = (($z_range.1 - $z_range.0) / $r) as usize + 1;

        let origin = [$x_range.0, $y_range.0, $z_range.0];
        let ptr: Box<dyn $trait> = match $dimension {
            Dimension::None => Box::new(ScalarFieldBuffer1D::new(
                Axis::X,
                Bounds::new(nx, ny, nz),
                origin,
                $r,
            )),
            Dimension::One(axis) => Box::new(ScalarFieldBuffer1D::new(
                axis,
                Bounds::new(nx, ny, nz),
                origin,
                $r,
            )),
            Dimension::Two(f, s) => Box::new(ScalarFieldBuffer2D::new(
                (f, s),
                Bounds::new(nx, ny, nz),
                origin,
                $r,
            )),
            Dimension::Three(f, s, t) => Box::new(ScalarFieldBuffer3D::new(
                (f, s, t),
                Bounds::new(nx, ny, nz),
                origin,
                $r,
            )),
        };
        ptr
    }};
}

pub struct Amplitude {}

impl Generator for Amplitude {
    type Output = Box<dyn AmplitudeFieldBuffer>;
    fn generate(
        dimension: Dimension,
        x_range: (Float, Float),
        y_range: (Float, Float),
        z_range: (Float, Float),
        r: Float,
    ) -> Self::Output {
        scalar_gen!(
            dimension,
            x_range,
            y_range,
            z_range,
            r,
            AmplitudeFieldBuffer
        )
    }
}

pub struct Intensity {}

impl Generator for Intensity {
    type Output = Box<dyn IntensityFieldBuffer>;
    fn generate(
        dimension: Dimension,
        x_range: (Float, Float),
        y_range: (Float, Float),
        z_range: (Float, Float),
        r: Float,
    ) -> Self::Output {
        scalar_gen!(
            dimension,
            x_range,
            y_range,
            z_range,
            r,
            IntensityFieldBuffer
        )
    }
}
