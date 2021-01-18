/*
 * File: complex_field_buffer_scatter.rs
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

use crate::{
    buffer::{bounds::Bounds, dimension::Dimension, traits::*},
    calculator::*,
    Complex, Vector3,
};

pub struct ComplexFieldBufferScatter {
    buffer: Vec<Complex>,
    observe_points: Vec<Vector3>,
}

impl ComplexFieldBufferScatter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            observe_points: Vec::new(),
        }
    }

    pub fn add_observe_point(&mut self, pos: Vector3, v: Complex) {
        self.observe_points.push(pos);
        self.buffer.push(v);
    }
}

impl Default for ComplexFieldBufferScatter {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldBuffer for ComplexFieldBufferScatter {
    type DataType = Complex;

    fn buffer(&self) -> &[Self::DataType] {
        &self.buffer
    }

    fn buffer_mut(&mut self) -> &mut Vec<Self::DataType> {
        &mut self.buffer
    }

    fn bounds(&self) -> Bounds {
        panic!("Not implemented!")
    }

    fn dimension(&self) -> Dimension {
        panic!("Not implemented!")
    }

    fn observe_points(&self) -> Box<dyn Iterator<Item = Vector3>> {
        Box::new(self.observe_points.clone().into_iter())
    }
}

impl<C> Calculate<C> for ComplexFieldBufferScatter
where
    C: ComplexFieldCalculator + ?Sized,
{
    fn calculate(&mut self, calculator: &C) {
        calculator.calc_complex(self);
    }
}
