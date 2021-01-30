/*
 * File: ghrbf.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    math_utils::*, optimizer::Optimizer, utils::transfer, wave_source::WaveSource, Complex, Float,
    Vector3, PI,
};
use ndarray::*;

pub struct GreedyBruteForce {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    phase_division: usize,
    amp_division: usize,
    randomize: bool,
}

impl GreedyBruteForce {
    pub fn new(phase_division: usize, amp_division: usize, randomize: bool) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            phase_division,
            amp_division,
            randomize,
        }
    }
}

impl GreedyBruteForce {
    #[allow(non_snake_case)]
    pub fn optimize_amp_phase(&self, wave_sources: &mut [WaveSource]) {
        let m = self.foci.len();

        let mut cache: ArrayBase<OwnedRepr<Complex>, _> = Array::zeros(m);
        let mut good_field = Array::zeros(m);

        let mut amps = Array::zeros(m);
        for i in 0..m {
            amps[i] = self.amps[i];
        }

        let amp_step = Complex::new(1.0 / self.amp_division as Float, 0.);
        let phase_step = Complex::new(0.0, 2.0 * PI / self.phase_division as Float).exp();

        if self.randomize {
            let mut rng = rand::thread_rng();
            use rand::seq::SliceRandom;
            wave_sources.shuffle(&mut rng);
        }

        let mut g: ArrayBase<OwnedRepr<Complex>, _> = Array::zeros(m);
        let mut gt: ArrayBase<OwnedRepr<Complex>, _> = Array::zeros(m);
        for wave_source in wave_sources {
            for i in 0..m {
                g[i] = transfer(wave_source.pos, self.foci[i]);
            }
            let mut min_q = Complex::new(0., 0.);
            let mut min_v = Float::INFINITY;
            for i in 1..=self.amp_division {
                let mut q = i as Float * amp_step;
                for _ in 0..self.phase_division {
                    let mut v = 0.0;
                    for j in 0..m {
                        let tmp = g[j] * q;
                        gt[j] = tmp;
                        v += (c_norm((tmp) + cache[j]) - amps[j]).abs();
                    }
                    if v < min_v {
                        min_v = v;
                        min_q = q;
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                gt.as_ptr(),
                                good_field.as_mut_ptr(),
                                gt.len(),
                            );
                        }
                    }
                    q = q * phase_step;
                }
            }

            cache = cache + &good_field;
            wave_source.q = min_q;
        }
    }
}

impl Optimizer for GreedyBruteForce {
    fn optimize(&self, wave_source: &mut [WaveSource]) {
        self.optimize_amp_phase(wave_source)
    }

    fn set_target_foci(&mut self, foci: &[Vector3]) {
        self.foci = foci.to_vec();
    }

    fn set_target_amps(&mut self, amps: &[Float]) {
        self.amps = amps.to_vec();
    }
}
