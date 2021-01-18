/*
 * File: ghrbf.rs
 * Project: optimizer
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
    buffer::{ComplexFieldBufferScatter, FieldBuffer},
    optimizer::Optimizer,
    utils::transfer,
    wave_source::WaveSource,
    Complex, Float, Vector3, PI,
};

fn transfer_buffer(
    buffer: &ComplexFieldBufferScatter,
    source: &WaveSource,
    wave_num: Float,
) -> Vec<Complex> {
    buffer
        .observe_points()
        .map(|observe_point| {
            transfer(
                source.pos,
                observe_point,
                source.amp,
                source.phase,
                wave_num,
            )
        })
        .collect()
}

pub struct GreedyBruteForce {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    phase_division: usize,
    amp_division: usize,
    wave_length: Float,
    include_amp: bool,
}

impl GreedyBruteForce {
    pub fn new(phase_division: usize, amp_division: usize, wave_length: Float) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            wave_length,
            phase_division,
            amp_division,
            include_amp: amp_division != 1,
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
            let mut min_v = Float::INFINITY;
            for &phase in &phases {
                wave_source.phase = phase;
                let field = transfer_buffer(&scatter, &wave_source, wave_num);
                let v: Float = field
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
            let mut min_v = Float::INFINITY;
            for (&phase, &amp) in iproduct!(&phases, &amps) {
                wave_source.amp = amp;
                wave_source.phase = phase;
                let field = transfer_buffer(&scatter, &wave_source, wave_num);
                let v: Float = field
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
    fn optimize(&self, wave_source: &mut [WaveSource]) {
        if self.include_amp {
            self.optimize_amp_phase(wave_source)
        } else {
            self.optimize_phase(wave_source)
        }
    }

    fn set_target_foci(&mut self, foci: &[Vector3]) {
        self.foci = foci.to_vec();
    }

    fn set_target_amps(&mut self, amps: &[Float]) {
        self.amps = amps.to_vec();
    }
}
