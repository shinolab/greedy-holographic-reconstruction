/*
 * File: scalar_filed_buffer_3d.rs
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

pub struct ScalarFieldBuffer3D {
    dim: (Axis, Axis, Axis),
    buffer: Vec<Float>,
    bounds: Bounds,
    origin: Vector3,
    resolution: Float,
}

impl ScalarFieldBuffer3D {
    pub fn new(
        dim: (Axis, Axis, Axis),
        bounds: Bounds,
        origin: Vector3,
        resolution: Float,
    ) -> Self {
        let mut buffer = Vec::with_capacity(bounds.size());
        unsafe {
            buffer.set_len(bounds.size());
        }
        Self {
            dim,
            buffer,
            bounds,
            origin,
            resolution,
        }
    }
}

impl ScalarFieldBuffer for ScalarFieldBuffer3D {}

impl FieldBuffer for ScalarFieldBuffer3D {
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
        Dimension::Three(self.dim.0, self.dim.1, self.dim.2)
    }

    fn observe_points(&self) -> Box<dyn Iterator<Item = Vector3>> {
        macro_rules! to_variable {
            (0, $x: ident, $y: ident, $z: ident) => {
                $x
            };
            (1, $x: ident, $y: ident, $z: ident) => {
                $y
            };
            (2, $x: ident, $y: ident, $z: ident) => {
                $z
            };
        }
        macro_rules! iter_gen {
            ($first:tt, $second:tt, $third:tt, $x: ident, $y: ident, $z: ident, $r: ident, $b: ident, $o: ident) => {
                Box::new(
                    iproduct!(
                        (0..$b[$third]).map(move |n| $o[$third] + (n as Float * $r)),
                        (0..$b[$second]).map(move |n| $o[$second] + (n as Float * $r)),
                        (0..$b[$first]).map(move |n| $o[$first] + (n as Float * $r))
                    )
                    .map(
                        move |(
                            to_variable!($third, $x, $y, $z),
                            to_variable!($second, $x, $y, $z),
                            to_variable!($first, $x, $y, $z),
                        )| { [$x, $y, $z] },
                    ),
                )
            };
        }
        let resolution = self.resolution;
        let bounds = self.bounds;
        let origin = self.origin;
        match self.dim {
            (Axis::X, Axis::Y, Axis::Z) => iter_gen!(0, 1, 2, x, y, z, resolution, bounds, origin),
            (Axis::Z, Axis::X, Axis::Y) => iter_gen!(2, 0, 1, x, y, z, resolution, bounds, origin),
            (Axis::Y, Axis::Z, Axis::X) => iter_gen!(1, 2, 0, x, y, z, resolution, bounds, origin),
            (Axis::X, Axis::Z, Axis::Y) => iter_gen!(0, 2, 1, x, y, z, resolution, bounds, origin),
            (Axis::Y, Axis::X, Axis::Z) => iter_gen!(1, 0, 2, x, y, z, resolution, bounds, origin),
            (Axis::Z, Axis::Y, Axis::X) => iter_gen!(2, 1, 0, x, y, z, resolution, bounds, origin),
            _ => unreachable!(),
        }
    }
}
