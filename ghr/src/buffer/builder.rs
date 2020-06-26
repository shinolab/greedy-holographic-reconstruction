/*
 * File: builder.rs
 * Project: buffer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/06/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use super::dimension::{Axis, Dimension};
use super::generator::*;

pub struct BufferBuilder {
    dimension: Dimension,
    x_range: (f32, f32),
    y_range: (f32, f32),
    z_range: (f32, f32),
    resolution: f32,
}

impl BufferBuilder {
    pub fn new() -> Self {
        Self {
            dimension: Dimension::None,
            x_range: (0., 0.),
            y_range: (0., 0.),
            z_range: (0., 0.),
            resolution: 1.0,
        }
    }

    pub fn x_range(mut self, x_min: f32, x_max: f32) -> Self {
        if self.dimension.contains(Axis::X) {
            panic!("You have already specified the range along x-axis.")
        }
        self.x_range = (x_min, x_max);
        self.dimension_update();
        self
    }

    pub fn y_range(mut self, y_min: f32, y_max: f32) -> Self {
        if self.dimension.contains(Axis::Y) {
            panic!("You have already specified the range along y-axis.")
        }
        self.y_range = (y_min, y_max);
        self.dimension_update();
        self
    }

    pub fn z_range(mut self, z_min: f32, z_max: f32) -> Self {
        if self.dimension.contains(Axis::Z) {
            panic!("You have already specified the range along z-axis.")
        }
        self.z_range = (z_min, z_max);
        self.dimension_update();
        self
    }

    pub fn x_at(mut self, x: f32) -> Self {
        if self.dimension.contains(Axis::X) {
            panic!("You have already specified the range along x-axis.")
        }
        self.x_range = (x, x);
        self.dimension_update();
        self
    }

    pub fn y_at(mut self, y: f32) -> Self {
        if self.dimension.contains(Axis::Y) {
            panic!("You have already specified the range along y-axis.")
        }
        self.y_range = (y, y);
        self.dimension_update();
        self
    }

    pub fn z_at(mut self, z: f32) -> Self {
        if self.dimension.contains(Axis::Z) {
            panic!("You have already specified the range along z-axis.")
        }
        self.z_range = (z, z);
        self.dimension_update();
        self
    }

    pub fn resolution(mut self, resolution: f32) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn generate<T: Generator>(self) -> T::Output {
        T::generate(
            self.dimension,
            self.x_range,
            self.y_range,
            self.z_range,
            self.resolution,
        )
    }

    fn dimension_update(&mut self) {
        let nx = ((self.x_range.1 - self.x_range.0) / self.resolution) as usize + 1;
        let ny = ((self.y_range.1 - self.y_range.0) / self.resolution) as usize + 1;
        let nz = ((self.z_range.1 - self.z_range.0) / self.resolution) as usize + 1;
        match self.dimension {
            Dimension::None => match (nx, ny, nz) {
                (1, 1, 1) => (),
                (_, 1, 1) => self.dimension.append(Axis::X),
                (1, _, 1) => self.dimension.append(Axis::Y),
                (1, 1, _) => self.dimension.append(Axis::Z),
                _ => unreachable!(),
            },
            Dimension::One(first) => match (first, nx, ny, nz) {
                (Axis::X, _, 1, 1) => (),
                (Axis::X, _, _, 1) => self.dimension.append(Axis::Y),
                (Axis::X, _, 1, _) => self.dimension.append(Axis::Z),
                (Axis::Y, 1, _, 1) => (),
                (Axis::Y, _, _, 1) => self.dimension.append(Axis::X),
                (Axis::Y, 1, _, _) => self.dimension.append(Axis::Z),
                (Axis::Z, 1, 1, _) => (),
                (Axis::Z, 1, _, _) => self.dimension.append(Axis::Y),
                (Axis::Z, _, 1, _) => self.dimension.append(Axis::X),
                _ => unreachable!(),
            },
            Dimension::Two(f, s) => match (f, s, nx, ny, nz) {
                (Axis::X, Axis::Y, _, _, 1) => (),
                (Axis::X, Axis::Y, _, _, _) => self.dimension.append(Axis::Z),
                (Axis::Y, Axis::X, _, _, 1) => (),
                (Axis::Y, Axis::X, _, _, _) => self.dimension.append(Axis::Z),
                (Axis::X, Axis::Z, _, 1, _) => (),
                (Axis::X, Axis::Z, _, _, _) => self.dimension.append(Axis::Y),
                (Axis::Z, Axis::X, _, 1, _) => (),
                (Axis::Z, Axis::X, _, _, _) => self.dimension.append(Axis::Y),
                (Axis::Y, Axis::Z, 1, _, _) => (),
                (Axis::Y, Axis::Z, _, _, _) => self.dimension.append(Axis::X),
                (Axis::Z, Axis::Y, 1, _, _) => (),
                (Axis::Z, Axis::Y, _, _, _) => self.dimension.append(Axis::X),
                _ => (),
            },
            Dimension::Three(_, _, _) => unreachable!(),
        }
    }
}

impl std::default::Default for BufferBuilder {
    fn default() -> Self {
        Self::new()
    }
}
