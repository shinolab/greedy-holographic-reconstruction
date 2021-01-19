/*
 * File: ghrbf.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    math_utils::*, optimizer::Optimizer, utils::transfer, wave_source::WaveSource, Complex, Float,
    Vector3, PI,
};

fn transfer_buffer(
    observe_points: &[Vector3],
    source_pos: Vector3,
    amp: Float,
    phase: Float,
    res: &mut Vec<Complex>,
) {
    for i in 0..observe_points.len() {
        res[i] = transfer(source_pos, observe_points[i], amp, phase)
    }
}

pub struct GreedyBruteForce {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    phase_division: usize,
    amp_division: usize,
    power_opt: bool,
    randamize: bool,
}

impl GreedyBruteForce {
    pub fn new(
        phase_division: usize,
        amp_division: usize,
        power_opt: bool,
        randamize: bool,
    ) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            phase_division,
            amp_division,
            power_opt,
            randamize,
        }
    }
}

impl GreedyBruteForce {
    #[allow(non_snake_case)]
    pub fn optimize_amp_phase<F: Fn(&Complex, &Complex, &Float) -> Float>(
        &self,
        wave_sources: &mut [WaveSource],
        func: F,
    ) {
        let mut field_tmp = vec![Complex::new(0., 0.); self.foci.len()];
        let mut cache = vec![Complex::new(0., 0.); self.foci.len()];
        let mut good_field = vec![Complex::new(0., 0.); self.foci.len()];
        let phases: Vec<_> = (0..self.phase_division)
            .map(|k| 2.0 * PI * k as Float / self.phase_division as Float)
            .collect();
        let amps: Vec<_> = (1..=self.amp_division)
            .map(|k| k as Float / self.amp_division as Float)
            .collect();

        if self.randamize {
            let mut rng = rand::thread_rng();
            use rand::seq::SliceRandom;
            wave_sources.shuffle(&mut rng);
        }

        for wave_source in wave_sources {
            let mut min_phase = 0.0;
            let mut min_amp = 0.0;
            let mut min_v = Float::INFINITY;
            for (&phase, &amp) in iproduct!(&phases, &amps) {
                transfer_buffer(&self.foci, wave_source.pos, amp, phase, &mut field_tmp);
                let v: Float = field_tmp
                    .iter()
                    .zip(cache.iter())
                    .zip(self.amps.iter())
                    .map(|((f, c), a)| func(f, c, a))
                    .map(|v| v.abs())
                    .sum();
                if v < min_v {
                    min_v = v;
                    min_phase = phase;
                    min_amp = amp;
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            field_tmp.as_ptr(),
                            good_field.as_mut_ptr(),
                            field_tmp.len(),
                        );
                    }
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
        let func = if self.power_opt {
            |f: &Complex, c: &Complex, a: &Float| a - (f + c).norm_sqr()
        } else {
            |f: &Complex, c: &Complex, a: &Float| a - c_norm(f + c)
        };

        self.optimize_amp_phase(wave_source, func)
    }

    fn set_target_foci(&mut self, foci: &[Vector3]) {
        self.foci = foci.to_vec();
    }

    fn set_target_amps(&mut self, amps: &[Float]) {
        self.amps = if self.power_opt {
            amps.iter().map(|v| v * v).collect()
        } else {
            amps.to_vec()
        };
    }
}
