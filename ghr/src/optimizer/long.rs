/*
 * File: long.rs
 * Project: optimizer
 * Created Date: 06/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    math_utils::c_norm, optimizer::Optimizer, utils::transfer, wave_source::WaveSource, Complex,
    Float, Vector3, PI,
};

use ndarray::*;
use ndarray_linalg::*;

pub struct Long {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    gamma: Float,
}

impl Long {
    pub fn new(gamma: Float) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            gamma,
        }
    }

    fn adjoint(m: &Array2<Complex>) -> Array2<Complex> {
        m.t().mapv(|c| c.conj())
    }
}

impl Optimizer for Long {
    fn set_target_foci(&mut self, foci: &[Vector3]) {
        self.foci = foci.to_vec();
    }

    fn set_target_amps(&mut self, amps: &[Float]) {
        self.amps = amps.to_vec();
    }

    #[allow(non_snake_case, clippy::many_single_char_names)]
    fn optimize(&self, wave_source: &mut [WaveSource]) {
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
                A[[i, j]] = transfer(wave_source[j].pos, fp, 1.0, 0.0);
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
        let mut max_value = 0.0;
        let mut max_idx = 0;
        for (j, &value) in d.iter().enumerate() {
            let value = value.norm_sqr();
            if value > max_value {
                max_idx = j;
                max_value = value;
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
                sum += c_norm(A[[i, j]]) * amps[i];
            }
            let v = (sum / m as Float).sqrt();
            let v = Float::pow(&v, self.gamma);
            sigma[[j, j]] = Complex::new(v, 0.0);
        }

        let G = stack![Axis(0), A, sigma];
        let mut f = Array::zeros(m + n);
        for i in 0..m {
            f[i] = amps[i] * (Complex::new(0., e_arg[i])).exp();
        }

        let gt = Self::adjoint(&G);
        let gtg = gt.dot(&G);
        let gtf = gt.dot(&f);
        let mut q = gtg.solve(&gtf).unwrap();

        // Correction provided in GS-PAT
        let zc = A.dot(&q);
        let ratio: Float = zc
            .iter()
            .zip(amps.iter())
            .map(|(&az, &a0)| c_norm(az) / a0)
            .sum();
        let avg_err = m as Float / ratio;
        for i in 0..n {
            q[i] = q[i] / avg_err;
        }

        for j in 0..n {
            let amp = c_norm(q[j]).min(1.0);
            let phase = q[j].arg() + PI;
            wave_source[j].amp = amp;
            wave_source[j].phase = phase;
        }
    }
}
