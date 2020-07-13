/*
 * File: levenberg_marquardt.rs
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

use crate::optimizer::Optimizer;
use std::f64::consts::PI;

use crate::vec_utils::*;
use crate::wave_source::WaveSource;
use crate::Vector3;

use ndarray::*;
use ndarray_linalg::*;

type Complex = c64;

const EPS_1: f64 = 1e-8;
const EPS_2: f64 = 1e-8;
const TAU: f64 = 1e-3;
const K_MAX: usize = 200;

pub struct LM {
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    wave_length: f64,
}

impl LM {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, wave_length: f64) -> Self {
        Self {
            foci,
            amps,
            wave_length,
        }
    }
}

impl LM {
    pub fn transfer(&self, trans_pos: Vector3, target_pos: Vector3) -> Complex {
        let wave_length = self.wave_length;
        let diff = sub(target_pos, trans_pos);
        let dist = norm(diff);

        1.0 / dist as f64 * (Complex::new(0., -2. * PI / wave_length * dist as f64)).exp()
    }

    fn adjoint(m: &Array2<Complex>) -> Array2<Complex> {
        m.t().mapv(|c| c.conj())
    }

    fn sum_col(x: &Array2<Complex>, n: usize) -> Array1<f64> {
        let mut res = Array::zeros(x.nrows());
        for i in 0..x.nrows() {
            let mut a = 0.0;
            for j in 0..n {
                a += x[[i, j]].im;
            }
            res[i] = a;
        }
        res
    }

    #[allow(non_snake_case, clippy::many_single_char_names)]
    pub fn optimize_phase(&self, wave_source: &mut [WaveSource]) {
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let m = foci.len();
        let n = num_trans;

        let x0: ArrayBase<OwnedRepr<f64>, _> = Array::zeros(n + m);
        let I: ArrayBase<OwnedRepr<f64>, _> = Array::eye(n + m);

        let mut P = Array::zeros((m, m));
        let mut G = Array::zeros((m, n));
        for i in 0..m {
            P[[i, i]] = Complex::new(amps[i], 0.0);
            let fp = foci[i];
            for j in 0..n {
                G[[i, j]] = self.transfer(wave_source[j].pos, fp);
            }
        }
        let B = stack![Axis(1), G, -P];
        let BhB = Self::adjoint(&B).dot(&B);

        let mut x = x0;
        let mut nu = 0.0;

        let T = x
            .mapv(|a| Complex::new(0.0, -a).exp())
            .into_shape((x.len(), 1))
            .unwrap();
        let TTt = T.dot(&Self::adjoint(&T));
        let BhB_TTt = &BhB * &TTt;
        let mut A = BhB_TTt.mapv(|c| c.re);
        let mut g = BhB_TTt.mapv(|c| c.im).sum_axis(Axis(1));

        let mut A_max: f64 = 0.0;
        for i in 0..(n + m) {
            A_max = A_max.max(A[[i, i]]);
        }
        let mut mu = TAU * A_max;

        let mut found = g.norm_max() <= EPS_1;
        let theta = x
            .mapv(|a| Complex::new(0.0, a).exp())
            .into_shape((x.len(), 1))
            .unwrap();
        let mut Fx = Self::adjoint(&theta).dot(&BhB).dot(&theta)[[0, 0]].abs();
        for _ in 0..K_MAX {
            if found {
                break;
            }
            let h_lm = -(&A + &(mu * &I)).solve(&g).unwrap();
            if h_lm.norm() <= EPS_2 * (x.norm() + EPS_2) {
                found = true;
            } else {
                let x_new = &x + &h_lm;
                let theta = x_new
                    .mapv(|a| Complex::new(0.0, a).exp())
                    .into_shape((x.len(), 1))
                    .unwrap();
                let Fx_new = Self::adjoint(&theta).dot(&BhB).dot(&theta)[[0, 0]].abs();
                let L0_Lhlm = 0.5 * h_lm.t().dot(&(mu * &h_lm - &g));
                let rho = (Fx - Fx_new) / L0_Lhlm;
                Fx = Fx_new;
                if rho > 0.0 {
                    x = x_new;
                    let T = x
                        .mapv(|a| Complex::new(0.0, -a).exp())
                        .into_shape((x.len(), 1))
                        .unwrap();
                    let TTt = T.dot(&Self::adjoint(&T));
                    let BhB_TTt = &BhB * &TTt;
                    A = BhB_TTt.mapv(|c| c.re);
                    g = BhB_TTt.mapv(|c| c.im).sum_axis(Axis(1));

                    found = g.norm_max() <= EPS_1;
                    mu *= (1f64 / 3.).max(1. - (2. * rho - 1.).pow(3.));
                    nu = 2.0;
                } else {
                    mu *= nu;
                    nu *= 2.0;
                }
            }
        }

        for j in 0..n {
            let amp = 1.0;
            let phase = (x[j] + PI) % (2.0 * PI);
            wave_source[j].amp = amp as f32;
            wave_source[j].phase = phase as f32;
        }
    }

    #[allow(non_snake_case, clippy::many_single_char_names)]
    pub fn optimize_amp_phase(&self, wave_source: &mut [WaveSource], normalize: bool) {
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let m = foci.len();
        let n = num_trans;

        let mut x0: ArrayBase<OwnedRepr<f64>, _> = Array::zeros(n + m + n);
        let I: ArrayBase<OwnedRepr<f64>, _> = Array::eye(n + m + n);
        for i in 0..n {
            x0[n + m + i] = 1.0;
        }

        let mut P = Array::zeros((m, m));
        let mut G = Array::zeros((m, n));
        for i in 0..m {
            P[[i, i]] = Complex::new(amps[i], 0.0);
            let fp = foci[i];
            for j in 0..n {
                G[[i, j]] = self.transfer(wave_source[j].pos, fp);
            }
        }

        let mut x = x0;
        let mut nu = 0.0;

        let A = Array2::from_diag(&x.slice(s![-(n as isize)..]).mapv(|r| Complex::new(r, 0.0)));

        let M = stack![Axis(1), G.dot(&A), -&P];
        let MhM = Self::adjoint(&M).dot(&M);

        let B = stack![Axis(1), M, -Complex::new(0., 1.) * &G];
        let BhB = Self::adjoint(&B).dot(&B);

        let T = x
            .mapv(|a| Complex::new(0.0, -a).exp())
            .into_shape((x.len(), 1))
            .unwrap();
        let TTt = T.dot(&Self::adjoint(&T));
        let BhB_TTt = &BhB * &TTt;
        let mut JtJ = BhB_TTt.mapv(|c| c.re);
        let mut g = Self::sum_col(&BhB_TTt, n + m);

        let mut A_max: f64 = 0.0;
        for i in 0..(n + m) {
            A_max = A_max.max(JtJ[[i, i]]);
        }
        let mut mu = TAU * A_max;

        let mut found = g.norm_max() <= EPS_1;
        let theta = x
            .slice(s![0..(n + m)])
            .mapv(|a| Complex::new(0.0, a).exp())
            .into_shape((n + m, 1))
            .unwrap();
        let mut Fx = Self::adjoint(&theta).dot(&MhM).dot(&theta)[[0, 0]].abs();
        for _ in 0..K_MAX {
            if found {
                break;
            }
            let h_lm = -(&JtJ + &(mu * &I)).solve(&g).unwrap();
            if h_lm.norm() <= EPS_2 * (x.norm() + EPS_2) {
                found = true;
            } else {
                let x_new = &x + &h_lm;
                let theta = x_new
                    .slice(s![0..(n + m)])
                    .mapv(|a| Complex::new(0.0, a).exp())
                    .into_shape((n + m, 1))
                    .unwrap();
                let A =
                    Array2::from_diag(&x.slice(s![-(n as isize)..]).mapv(|r| Complex::new(r, 0.0)));
                let M = stack![Axis(1), G.dot(&A), -&P];
                let MhM = Self::adjoint(&M).dot(&M);
                let Fx_new = Self::adjoint(&theta).dot(&MhM).dot(&theta)[[0, 0]].abs();
                let L0_Lhlm = 0.5 * h_lm.t().dot(&(mu * &h_lm - &g));
                let rho = (Fx - Fx_new) / L0_Lhlm;
                Fx = Fx_new;
                if rho > 0.0 {
                    x = x_new;
                    let T = x
                        .mapv(|a| Complex::new(0.0, -a).exp())
                        .into_shape((x.len(), 1))
                        .unwrap();
                    let TTt = T.dot(&Self::adjoint(&T));
                    let B = stack![Axis(1), M, -Complex::new(0., 1.) * G.clone()];
                    let BhB = Self::adjoint(&B).dot(&B);
                    let BhB_TTt = &BhB * &TTt;
                    JtJ = BhB_TTt.mapv(|c| c.re);
                    g = Self::sum_col(&BhB_TTt, n + m);

                    found = g.norm_max() <= EPS_1;
                    mu *= (1f64 / 3.).max(1. - (2. * rho - 1.).pow(3.));
                    nu = 2.0;
                } else {
                    mu *= nu;
                    nu *= 2.0;
                }
            }
        }

        let mut max_coeff: f64 = 0.0;
        for v in x.slice(s![-(n as isize)..]) {
            max_coeff = max_coeff.max(v.abs());
        }
        for j in 0..n {
            let amp = if normalize {
                x.slice(s![-(n as isize)..])[j].abs() / max_coeff
            } else {
                x.slice(s![-(n as isize)..])[j].abs().min(1.0)
            };
            let phase = (x[j] + PI) % (2.0 * PI);
            wave_source[j].amp = amp as f32;
            wave_source[j].phase = phase as f32;
        }
    }
}

impl Optimizer for LM {
    fn optimize(&self, wave_source: &mut [WaveSource], include_amp: bool, normalize: bool) {
        if include_amp {
            self.optimize_amp_phase(wave_source, normalize)
        } else {
            self.optimize_phase(wave_source)
        }
    }
}
