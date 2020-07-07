/*
 * File: scalar_filed_buffer_2d.rs
 * Project: scalar_filed_buffer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::traits::ScalarFieldBuffer;
use crate::buffer::bounds::Bounds;
use crate::buffer::dimension::{Axis, Dimension};
use crate::buffer::traits::*;
use crate::Vector3;

pub struct ScalarFieldBuffer2D {
    dim: (Axis, Axis),
    buffer: Vec<f32>,
    bounds: Bounds,
    origin: Vector3,
    resolution: f32,
}

impl ScalarFieldBuffer2D {
    pub fn new(dim: (Axis, Axis), bounds: Bounds, origin: Vector3, resolution: f32) -> Self {
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

impl ScalarFieldBuffer for ScalarFieldBuffer2D {}

impl FieldBuffer for ScalarFieldBuffer2D {
    type DataType = f32;

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
        Dimension::Two(self.dim.0, self.dim.1)
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
            ($first:tt, $second:tt, $x: ident, $y: ident, $z: ident, $another: stmt , $r: ident, $b: ident, $o: ident) => {
                Box::new({
                    $another
                    iproduct!(
                        (0..$b[$second]).map(move |n| $o[$second] + (n as f32 * $r)),
                        (0..$b[$first]).map(move |n| $o[$first] + (n as f32 * $r))
                    )
                    .map(
                        move |(to_variable!($second, $x, $y, $z), to_variable!($first, $x, $y, $z))| {
                            [$x, $y, $z]
                        },
                    )},
                )
            };
        }
        let resolution = self.resolution;
        let bounds = self.bounds;
        let origin = self.origin;
        match self.dim {
            (Axis::X, Axis::Y) => {
                iter_gen!(0, 1, x, y, z, let z = origin[2], resolution, bounds, origin)
            }
            (Axis::Y, Axis::X) => {
                iter_gen!(1, 0, x, y, z, let z = origin[2], resolution, bounds, origin)
            }
            (Axis::Y, Axis::Z) => {
                iter_gen!(1, 2, x, y, z, let x = origin[0], resolution, bounds, origin)
            }
            (Axis::Z, Axis::Y) => {
                iter_gen!(2, 1, x, y, z, let x = origin[0], resolution, bounds, origin)
            }
            (Axis::Z, Axis::X) => {
                iter_gen!(2, 0, x, y, z, let y = origin[1], resolution, bounds, origin)
            }
            (Axis::X, Axis::Z) => {
                iter_gen!(0, 2, x, y, z, let y = origin[1], resolution, bounds, origin)
            }
            _ => unreachable!(),
        }
    }
}
