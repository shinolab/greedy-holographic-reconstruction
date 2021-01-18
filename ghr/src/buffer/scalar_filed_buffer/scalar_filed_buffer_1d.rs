/*
 * File: scalar_filed_buffer_1d.rs
 * Project: scalar_filed_buffer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::ScalarFieldBuffer;
use crate::{
    buffer::{
        bounds::Bounds,
        dimension::{Axis, Dimension},
        traits::*,
    },
    Float, Vector3,
};

pub struct ScalarFieldBuffer1D {
    axis: Axis,
    buffer: Vec<Float>,
    bounds: Bounds,
    origin: Vector3,
    resolution: Float,
}

impl ScalarFieldBuffer1D {
    pub fn new(axis: Axis, bounds: Bounds, origin: Vector3, resolution: Float) -> Self {
        let mut buffer = Vec::with_capacity(bounds.size());
        unsafe {
            buffer.set_len(bounds.size());
        }
        Self {
            axis,
            buffer,
            bounds,
            origin,
            resolution,
        }
    }
}

impl ScalarFieldBuffer for ScalarFieldBuffer1D {}

impl FieldBuffer for ScalarFieldBuffer1D {
    type DataType = Float;

    fn buffer(&self) -> &[Self::DataType] {
        &self.buffer
    }

    fn buffer_mut(&mut self) -> &mut Vec<Self::DataType> {
        &mut self.buffer
    }

    fn bounds(&self) -> Bounds {
        self.bounds
    }

    fn dimension(&self) -> Dimension {
        Dimension::One(self.axis)
    }

    fn observe_points(&self) -> Box<dyn Iterator<Item = Vector3>> {
        let resolution = self.resolution;
        let len = self.bounds.size();
        let origin = self.origin;
        match self.axis {
            Axis::X => Box::new(
                (0..len)
                    .map(move |n| [origin[0] + (n as Float * resolution), origin[1], origin[2]]),
            ),
            Axis::Y => Box::new(
                (0..len)
                    .map(move |n| [origin[0], origin[1] + (n as Float * resolution), origin[2]]),
            ),
            Axis::Z => Box::new(
                (0..len)
                    .map(move |n| [origin[0], origin[1], origin[2] + (n as Float * resolution)]),
            ),
        }
    }
}
