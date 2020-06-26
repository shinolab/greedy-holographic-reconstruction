/*
 * File: horn.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/06/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use std::f64::consts::PI;

use crate::wave_source::WaveSource;
use crate::Vector3;

use na::{ComplexField, Dynamic, Matrix, VecStorage, U1};
use rand::{thread_rng, Rng};

type Complex = na::Complex<f64>;
type MatrixXcf = Matrix<Complex, Dynamic, Dynamic, VecStorage<Complex, Dynamic, Dynamic>>;
type VectorXcf = Matrix<Complex, Dynamic, U1, VecStorage<Complex, Dynamic, U1>>;

const REPEAT_SDP: usize = 100;
const LAMBDA_SDP: f64 = 0.8;

pub struct Horn {
    foci: Vec<Vector3>,
    amps: Vec<f32>,
    wave_length: f64,
    repeat: usize,
    lambda: f64,
}

impl Horn {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f32>, wave_length: f64) -> Self {
        Self {
            foci,
            amps,
            wave_length,
            repeat: REPEAT_SDP,
            lambda: LAMBDA_SDP,
        }
    }

    pub fn set_wave_length(&mut self, wave_length: f64) {
        self.wave_length = wave_length;
    }

    pub fn set_repeat(&mut self, repeat: usize) {
        self.repeat = repeat;
    }

    pub fn set_lambda(&mut self, lambda: f64) {
        self.lambda = lambda;
    }
}

impl Horn {
    pub fn transfer(&self, trans_pos: Vector3, target_pos: Vector3) -> Complex {
        let wave_length = self.wave_length;
        let diff = target_pos - trans_pos;
        let dist = diff.norm();

        1.0 / dist as f64 * (Complex::new(0., -2. * PI / wave_length * dist as f64)).exp()
    }

    #[allow(clippy::many_single_char_names)]
    pub fn optimize(&self, wave_source: &mut [WaveSource]) {
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let alpha = 1e-5;
        let m = foci.len();
        let n = num_trans;
        let mut b = MatrixXcf::from_vec(m, n, vec![Complex::new(0., 0.); m * n]);
        let mut p = MatrixXcf::from_vec(m, m, vec![Complex::new(0., 0.); m * m]);

        let mut rng = thread_rng();
        for i in 0..m {
            p[(i, i)] = Complex::new(amps[i] as f64, 0.);
            let tp = foci[i];
            for j in 0..n {
                b[(i, j)] = self.transfer(wave_source[j].pos, tp);
            }
        }
        let svd = b.clone().svd(true, true);
        let mut singular_values_inv = svd.singular_values.clone();
        for i in 0..singular_values_inv.len() {
            singular_values_inv[i] = singular_values_inv[i]
                / (singular_values_inv[i] * singular_values_inv[i] + alpha * alpha);
        }
        let mut singular_values_inv_mat =
            MatrixXcf::from_vec(m, m, vec![Complex::new(0., 0.); m * m]);
        singular_values_inv_mat.set_diagonal(&singular_values_inv.map(|r| Complex::new(r, 0.)));
        let pinv_b = match (&svd.v_t, &svd.u) {
            (Some(v_t), Some(u)) => v_t.adjoint() * singular_values_inv_mat * u.adjoint(),
            _ => unreachable!(),
        };
        let mm = &p * (MatrixXcf::identity(m, m) - b * &pinv_b) * &p;
        let mut x = MatrixXcf::identity(m, m);

        let lambda = self.lambda;
        for _ in 0..(m * self.repeat) {
            let ii = (m as f64 * rng.gen_range(0., 1.)) as usize;
            let xc = x.clone().remove_row(ii).remove_column(ii);
            let mmc = mm.column(ii).remove_row(ii);
            let xb = xc * &mmc;
            let gamma = xb.adjoint() * mmc;
            let gamma = gamma[(0, 0)];
            if gamma.re > 0. {
                let xb = xb.scale(-(lambda / gamma.re).sqrt());
                x.slice_mut((ii, 0), (1, ii))
                    .copy_from(&xb.slice((0, 0), (ii, 1)).adjoint());
                x.slice_mut((ii, ii + 1), (1, m - ii - 1))
                    .copy_from(&xb.slice((ii, 0), (m - 1 - ii, 1)).adjoint());
                x.slice_mut((0, ii), (ii, 1))
                    .copy_from(&xb.slice((0, 0), (ii, 1)));
                x.slice_mut((ii + 1, ii), (m - ii - 1, 1))
                    .copy_from(&xb.slice((ii, 0), (m - 1 - ii, 1)));
            } else {
                let z1 = VectorXcf::from_vec(vec![Complex::new(0., 0.,); ii]);
                let z2 = VectorXcf::from_vec(vec![Complex::new(0., 0.,); m - ii - 1]);
                x.slice_mut((ii, 0), (1, ii)).copy_from(&z1.adjoint());
                x.slice_mut((ii, ii + 1), (1, m - ii - 1))
                    .copy_from(&z2.adjoint());
                x.slice_mut((0, ii), (ii, 1)).copy_from(&z1);
                x.slice_mut((ii + 1, ii), (m - ii - 1, 1)).copy_from(&z2);
            }
        }

        let ces = na::SymmetricEigen::new(x);
        let evs = ces.eigenvalues;
        let mut abs_eiv = 0.;
        let mut idx = 0;
        for j in 0..evs.len() {
            let eiv = evs[j].abs();
            if abs_eiv < eiv {
                abs_eiv = eiv;
                idx = j;
            }
        }

        let u = ces.eigenvectors.column(idx);
        let q = pinv_b * p * u;
        let mut max_coeff: f64 = 0.0;
        for v in q.iter() {
            max_coeff = max_coeff.max(v.abs());
        }
        for j in 0..n {
            let amp = q[j].abs() / max_coeff;
            let phase = q[j].argument() + PI;
            wave_source[j].amp = amp as f32;
            wave_source[j].phase = phase as f32;
        }
    }
}
