/*
 * File: levenberg_marquardt.rs
 * Project: optimizer
 * Created Date: 06/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

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

    #[allow(non_snake_case)]
    fn calc_JhJ_Jhfx(x: &Array2<Complex>, theta: &Array1<f64>) -> (Array2<f64>, Array1<f64>) {
        let T = theta
            .mapv(|a| Complex::new(0.0, -a).exp())
            .into_shape((theta.len(), 1))
            .unwrap();
        let TTt = T.dot(&Self::adjoint(&T));
        let X = x * &TTt;
        let JhJ = X.mapv(|c| c.re);
        let Jhfx = X.mapv(|c| c.im).sum_axis(Axis(1));
        (JhJ, Jhfx)
    }

    #[allow(non_snake_case)]
    fn calc_Fx(x: &Array2<Complex>, theta: &Array1<f64>) -> f64 {
        let T = theta
            .mapv(|a| Complex::new(0.0, a).exp())
            .into_shape((theta.len(), 1))
            .unwrap();
        Self::adjoint(&T).dot(x).dot(&T)[[0, 0]].re
    }

    #[allow(non_snake_case, clippy::many_single_char_names)]
    pub fn optimize(&self, wave_source: &mut [WaveSource]) {
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
        let (mut A, mut g) = Self::calc_JhJ_Jhfx(&BhB, &x);

        let mut A_max: f64 = 0.0;
        for i in 0..(n + m) {
            A_max = A_max.max(A[[i, i]]);
        }
        let mut mu = TAU * A_max;

        let mut found = g.norm_max() <= EPS_1;
        let mut Fx = Self::calc_Fx(&BhB, &x);
        for _ in 0..K_MAX {
            if found {
                break;
            }
            let h_lm = -(&A + &(mu * &I)).solve(&g).unwrap();
            if h_lm.norm() <= EPS_2 * (x.norm() + EPS_2) {
                found = true;
            } else {
                let x_new = &x + &h_lm;
                let Fx_new = Self::calc_Fx(&BhB, &x_new);
                let L0_Lhlm = 0.5 * h_lm.t().dot(&(mu * &h_lm - &g));
                let rho = (Fx - Fx_new) / L0_Lhlm;
                Fx = Fx_new;
                if rho > 0.0 {
                    x = x_new;
                    let (A_new, g_new) = Self::calc_JhJ_Jhfx(&BhB, &x);
                    A = A_new;
                    g = g_new;
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
}
