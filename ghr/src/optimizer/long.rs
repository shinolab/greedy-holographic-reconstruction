/*
 * File: long.rs
 * Project: optimizer
 * Created Date: 06/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use std::f64::consts::PI;

use super::Optimizer;
use crate::vec_utils::*;
use crate::wave_source::WaveSource;
use crate::Vector3;

use ndarray::*;
use ndarray_linalg::*;

type Complex = c64;

pub struct Long {
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    wave_length: f64,
}

impl Long {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, wave_length: f64) -> Self {
        Self {
            foci,
            amps,
            wave_length,
        }
    }
}

impl Long {
    pub fn transfer(&self, trans_pos: Vector3, target_pos: Vector3) -> Complex {
        let wave_length = self.wave_length;
        let diff = sub(target_pos, trans_pos);
        let dist = norm(diff);

        1.0 / dist as f64 * (Complex::new(0., -2. * PI / wave_length * dist as f64)).exp()
    }

    fn adjoint(m: &Array2<Complex>) -> Array2<Complex> {
        m.t().mapv(|c| c.conj())
    }
}

impl Optimizer for Long {
    #[allow(non_snake_case, clippy::many_single_char_names)]
    fn optimize(&self, wave_source: &mut [WaveSource], include_amp: bool, normalize: bool) {
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let m = foci.len();
        let n = num_trans;

        let mut X = Array::zeros((n, m));
        let mut A = Array::zeros((m, n));

        for i in 0..m {
            let fp = foci[i];
            for j in 0..n {
                A[[i, j]] = self.transfer(wave_source[j].pos, fp);
            }
        }

        for i in 0..m {
            let mut denomi = 0.0;
            for j in 0..n {
                denomi += A[[i, j]].norm_sqr();
            }
            for j in 0..n {
                X[[j, i]] = Complex::new(amps[i], 0.0) * A[[i, j]].conj() / denomi;
            }
        }

        let R = A.dot(&X);

        let (d, V) = R.eig().unwrap();
        let mut max_idx = 0;
        for (j, &value) in d.iter().enumerate() {
            if value.abs() > d[max_idx].abs() {
                max_idx = j;
            }
        }

        let em_V = V.index_axis(Axis(0), max_idx);
        let mut e_arg = Array::zeros(m);
        for i in 0..m {
            e_arg[i] = em_V[i].arg();
        }

        let mut sigma = Array::zeros((n, n));
        for j in 0..n {
            let mut sum = 0.0;
            for i in 0..m {
                sum += A[[i, j]].abs() * amps[i];
            }
            sigma[[j, j]] = Complex::new((sum / m as f64).sqrt(), 0.0);
        }

        let G = stack![Axis(0), A, sigma];
        let mut f = Array::zeros(m + n);
        for i in 0..m {
            f[i] = amps[i] * (Complex::new(0., e_arg[i])).exp();
        }

        let gt = Self::adjoint(&G);
        let gtg = gt.dot(&G);
        let gtf = gt.dot(&f);
        let q = gtg.solve(&gtf).unwrap();

        let mut max_coeff: f64 = 0.0;
        for v in q.iter() {
            max_coeff = max_coeff.max(v.abs());
        }
        for j in 0..n {
            let amp = match (include_amp, normalize) {
                (false, _) => 1.0,
                (_, true) => q[j].abs() / max_coeff,
                (_, false) => q[j].abs().min(1.0),
            };
            let phase = q[j].arg() + PI;
            wave_source[j].amp = amp as f32;
            wave_source[j].phase = phase as f32;
        }
    }
}
