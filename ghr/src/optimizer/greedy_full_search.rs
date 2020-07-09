/*
 * File: greedy_full_search.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use rayon::prelude::*;

use crate::buffer::{ComplexFieldBufferScatter, FieldBuffer};
use crate::calculator::*;
use crate::vec_utils::*;
use crate::wave_source::WaveSource;
use crate::Vector3;

use std::f32::consts::PI;

use ndarray_linalg::*;

type Complex = c32;

pub struct GreedyFullSearch {
    foci: Vec<Vector3>,
    amps: Vec<f32>,
    division: usize,
    wave_length: f64,
}

impl GreedyFullSearch {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, wave_length: f64) -> Self {
        Self {
            foci,
            amps: amps.iter().map(|&x| x as f32).collect(),
            wave_length,
            division: 16,
        }
    }
    fn transfer(
        buffer: &ComplexFieldBufferScatter,
        source: &WaveSource,
        wave_num: f32,
    ) -> Vec<Complex> {
        buffer
            .observe_points()
            .collect::<Vec<_>>()
            .par_iter()
            .map(|&observe_point| {
                let diff = sub(observe_point, source.pos);
                let dist = norm(diff);
                let r = source.amp / dist;
                let phi = source.phase - wave_num * dist;
                Complex::from_polar(&r, &phi)
            })
            .collect()
    }

    #[allow(non_snake_case)]
    pub fn optimize(&self, wave_sources: &mut [WaveSource]) {
        let mut scatter = crate::buffer::ComplexFieldBufferScatter::new();
        for target_point in self.foci.iter() {
            scatter.add_observe_point(*target_point, Complex::new(0., 0.));
        }
        let wave_num = 2.0 * PI / self.wave_length as f32;
        let mut cache = vec![Complex::new(0., 0.); self.foci.len()];
        let mut good_field = vec![Complex::new(0., 0.); self.foci.len()];
        for wave_source in wave_sources {
            wave_source.amp = 1.0;
            let mut min_idx = 0;
            let mut min_v = f32::INFINITY;
            for k in 0..self.division {
                wave_source.phase = 2.0 * PI * k as f32 / self.division as f32;
                let field = Self::transfer(&scatter, &wave_source, wave_num);
                let v: f32 = field
                    .iter()
                    .zip(cache.iter())
                    .zip(self.amps.iter())
                    .map(|((f, c), a)| a - (f + c).norm())
                    .sum();
                if v < min_v {
                    min_v = v;
                    min_idx = k;
                    good_field = field;
                }
            }
            for i in 0..cache.len() {
                cache[i] += good_field[i];
            }
            wave_source.phase = 2.0 * PI * min_idx as f32 / self.division as f32;
        }
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
        let wave_sources = calculator.wave_sources();
        let mut obj_fn = obj_fn;
        let wave_num = 2.0 * PI / self.wave_length as f32;
        let mut cache = vec![Complex::new(0., 0.); target_points.len()];
        let mut good_field = vec![Complex::new(0., 0.); target_points.len()];
        for wave_source in wave_sources {
            wave_source.amp = 1.0;
            let mut max_idx = 0;
            let mut max_v = f32::NEG_INFINITY;
            for k in 0..self.division {
                wave_source.phase = 2.0 * PI * k as f32 / self.division as f32;
                let field = Self::transfer(&scatter, &wave_source, wave_num);
                let v: f32 = field
                    .iter()
                    .zip(cache.iter())
                    .map(|(f, c)| obj_fn(&(f + c)))
                    .sum();
                if max_v < v {
                    max_v = v;
                    max_idx = k;
                    good_field = field;
                }
            }
            for i in 0..cache.len() {
                cache[i] += good_field[i];
            }
            wave_source.phase = 2.0 * PI * max_idx as f32 / self.division as f32;
        }
    }

    #[allow(non_snake_case)]
    pub fn minimize<C, F>(&self, calculator: &mut C, target_points: &[Vector3], obj_fn: F)
    where
        C: Calculator + ComplexFieldCalculator,
        F: FnMut(&Complex) -> f32,
    {
        let mut scatter = crate::buffer::ComplexFieldBufferScatter::new();
        for target_point in target_points {
            scatter.add_observe_point(*target_point, Complex::new(0., 0.));
        }
        let wave_sources = calculator.wave_sources();
        let mut obj_fn = obj_fn;
        let wave_num = 2.0 * PI / self.wave_length as f32;
        let mut cache = vec![Complex::new(0., 0.); target_points.len()];
        let mut good_field = vec![Complex::new(0., 0.); target_points.len()];
        for wave_source in wave_sources {
            wave_source.amp = 1.0;
            let mut min_idx = 0;
            let mut min_v = f32::INFINITY;
            for k in 0..self.division {
                wave_source.phase = 2.0 * PI * k as f32 / self.division as f32;
                let field = Self::transfer(&scatter, &wave_source, wave_num);
                let v: f32 = field
                    .iter()
                    .zip(cache.iter())
                    .map(|(f, c)| obj_fn(&(f + c)))
                    .sum();
                if v < min_v {
                    min_v = v;
                    min_idx = k;
                    good_field = field;
                }
            }
            for i in 0..cache.len() {
                cache[i] += good_field[i];
            }
            wave_source.phase = 2.0 * PI * min_idx as f32 / self.division as f32;
        }
    }
}
