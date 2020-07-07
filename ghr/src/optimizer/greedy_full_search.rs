/*
 * File: greedy_full_search.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::buffer::FieldBuffer;
use crate::calculator::*;
use crate::Vector3;

use std::f32::consts::PI;

use ndarray_linalg::*;

type Complex = c32;

pub struct GreedyFullSearch {
    division: usize,
}

impl GreedyFullSearch {
    pub fn new(division: usize) -> Self {
        Self { division }
    }

    #[allow(non_snake_case)]
    pub fn maximize<C, F>(&self, calculator: &mut C, target_points: &[Vector3], obj_fn: F)
    where
        C: Calculator + ComplexFieldCalculator,
        F: FnMut(&Complex) -> f32,
    {
        let mut scatter = crate::buffer::ComplexFieldBufferScatter::new();
        for target_point in target_points {
            scatter.add_observe_point(*target_point, Complex::new(0., 0.));
        }
        let N = calculator.wave_sources().len();
        let mut obj_fn = obj_fn;
        for i in 0..N {
            calculator.wave_sources()[i].amp = 1.0;
            let mut max_idx = 0;
            let mut max_v = f32::NEG_INFINITY;
            for k in 0..self.division {
                calculator.wave_sources()[i].phase = 2.0 * PI * k as f32 / self.division as f32;
                calculator.calc_complex(&mut scatter);
                let v: f32 = scatter.buffer().iter().map(|c| obj_fn(c)).sum();
                if max_v < v {
                    max_v = v;
                    max_idx = k;
                }
            }
            calculator.wave_sources()[i].phase = 2.0 * PI * max_idx as f32 / self.division as f32;
        }
    }
}
