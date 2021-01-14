/*
 * File: ghrbf.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    buffer::{ComplexFieldBufferScatter, FieldBuffer},
    optimizer::Optimizer,
    vec_utils::*,
    wave_source::WaveSource,
    Complex, Float, Vector3, PI,
};

const PHASE_DIV: usize = 16;
const AMP_DIV: usize = 16;

fn transfer(
    buffer: &ComplexFieldBufferScatter,
    source: &WaveSource,
    wave_num: Float,
) -> Vec<Complex> {
    buffer
        .observe_points()
        .map(|observe_point| {
            let diff = sub(observe_point, source.pos);
            let dist = norm(diff);
            let r = source.amp / dist;
            let phi = source.phase + wave_num * dist;
            Complex::from_polar(&r, &phi)
        })
        .collect()
}

pub struct GreedyBruteForce {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    phase_division: usize,
    amp_division: usize,
    wave_length: Float,
}

impl GreedyBruteForce {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>, wave_length: Float) -> Self {
        Self {
            foci,
            amps,
            wave_length,
            phase_division: PHASE_DIV,
            amp_division: AMP_DIV,
        }
    }

    fn optimize_phase(&self, wave_sources: &mut [WaveSource]) {
        let mut scatter = crate::buffer::ComplexFieldBufferScatter::new();
        for target_point in self.foci.iter() {
            scatter.add_observe_point(*target_point, Complex::new(0., 0.));
        }
        let wave_num = 2.0 * PI / self.wave_length;
        let mut cache = vec![Complex::new(0., 0.); self.foci.len()];
        let mut good_field = vec![Complex::new(0., 0.); self.foci.len()];
        let phases: Vec<_> = (0..self.phase_division)
            .map(|k| 2.0 * PI * k as Float / self.phase_division as Float)
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

    #[allow(non_snake_case)]
    pub fn optimize_amp_phase(&self, wave_sources: &mut [WaveSource]) {
        let mut scatter = crate::buffer::ComplexFieldBufferScatter::new();
        for target_point in self.foci.iter() {
            scatter.add_observe_point(*target_point, Complex::new(0., 0.));
        }
        let wave_num = 2.0 * PI / self.wave_length;
        let mut cache = vec![Complex::new(0., 0.); self.foci.len()];
        let mut good_field = vec![Complex::new(0., 0.); self.foci.len()];
        let phases: Vec<_> = (0..self.phase_division)
            .map(|k| 2.0 * PI * k as Float / self.phase_division as Float)
            .collect();
        let amps: Vec<_> = (1..=self.amp_division)
            .map(|k| k as Float / self.amp_division as Float)
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

impl Optimizer for GreedyBruteForce {
    fn optimize(&self, wave_source: &mut [WaveSource], include_amp: bool, _normalize: bool) {
        if include_amp {
            self.optimize_amp_phase(wave_source)
        } else {
            self.optimize_phase(wave_source)
        }
    }
}
