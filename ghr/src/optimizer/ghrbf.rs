/*
 * File: ghrbf.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::buffer::{ComplexFieldBufferScatter, FieldBuffer};
use crate::vec_utils::*;
use crate::wave_source::WaveSource;
use crate::Vector3;

use std::f32::consts::PI;

use ndarray_linalg::*;

type Complex = c32;
const PHASE_DIV: usize = 16;
const AMP_DIV: usize = 10;

fn transfer(
    buffer: &ComplexFieldBufferScatter,
    source: &WaveSource,
    wave_num: f32,
) -> Vec<Complex> {
    buffer
        .observe_points()
        .map(|observe_point| {
            let diff = sub(observe_point, source.pos);
            let dist = norm(diff);
            let r = source.amp / dist;
            let phi = source.phase - wave_num * dist;
            Complex::from_polar(&r, &phi)
        })
        .collect()
}

pub struct GreedyBruteForcePhase {
    foci: Vec<Vector3>,
    amps: Vec<f32>,
    division: usize,
    wave_length: f64,
}

impl GreedyBruteForcePhase {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, wave_length: f64) -> Self {
        Self {
            foci,
            amps: amps.iter().map(|&x| x as f32).collect(),
            wave_length,
            division: PHASE_DIV,
        }
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
        let phases: Vec<_> = (0..self.division)
            .map(|k| 2.0 * PI * k as f32 / self.division as f32)
            .collect();
        for wave_source in wave_sources {
            wave_source.amp = 1.0;
            let mut min_phase = 0.0;
            let mut min_v = f32::INFINITY;
            for &phase in &phases {
                wave_source.phase = phase;
                let field = transfer(&scatter, &wave_source, wave_num);
                let v: f32 = field
                    .iter()
                    .zip(cache.iter())
                    .zip(self.amps.iter())
                    .map(|((f, c), a)| a - (f + c).norm())
                    .map(|v| v * v)
                    .sum();
                if v < min_v {
                    min_v = v;
                    min_phase = phase;
                    good_field = field;
                }
            }
            for i in 0..cache.len() {
                cache[i] += good_field[i];
            }
            wave_source.phase = min_phase;
        }
    }
}

pub struct GreedyBruteForcePhaseAmp {
    foci: Vec<Vector3>,
    amps: Vec<f32>,
    phase_division: usize,
    amp_division: usize,
    wave_length: f64,
}

impl GreedyBruteForcePhaseAmp {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, wave_length: f64) -> Self {
        Self {
            foci,
            amps: amps.iter().map(|&x| x as f32).collect(),
            wave_length,
            amp_division: AMP_DIV,
            phase_division: PHASE_DIV,
        }
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
        let phases: Vec<_> = (0..self.phase_division)
            .map(|k| 2.0 * PI * k as f32 / self.phase_division as f32)
            .collect();
        let amps: Vec<_> = (1..=self.amp_division)
            .map(|k| k as f32 / self.amp_division as f32)
            .collect();
        for wave_source in wave_sources {
            let mut min_phase = 0.0;
            let mut min_amp = 0.0;
            let mut min_v = f32::INFINITY;
            for (&phase, &amp) in iproduct!(&phases, &amps) {
                wave_source.amp = amp;
                wave_source.phase = phase;
                let field = transfer(&scatter, &wave_source, wave_num);
                let v: f32 = field
                    .iter()
                    .zip(cache.iter())
                    .zip(self.amps.iter())
                    .map(|((f, c), a)| a - (f + c).norm())
                    .map(|v| v * v)
                    .sum();
                if v < min_v {
                    min_v = v;
                    min_phase = phase;
                    min_amp = amp;
                    good_field = field;
                }
            }
            for i in 0..cache.len() {
                cache[i] += good_field[i];
            }
            wave_source.amp = min_amp;
            wave_source.phase = min_phase;
        }
    }
}
