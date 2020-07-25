/*
 * File: levenberg_marquardt.rs
 * Project: optimizer
 * Created Date: 06/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/07/2020
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
    fn transfer(trans_pos: Vector3, target_pos: Vector3, wave_length: f64) -> Complex {
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

    #[allow(non_snake_case)]
    fn make_BhB(
        amps: &[f64],
        foci: &[Vector3],
        wave_source: &mut [WaveSource],
        n: usize,
        m: usize,
        include_amp: bool,
        wave_length: f64,
    ) -> Array2<Complex> {
        let mut P = Array::zeros((m, m));
        let mut G = Array::zeros((m, n));
        for i in 0..m {
            P[[i, i]] = Complex::new(amps[i], 0.0);
            let fp = foci[i];
            for j in 0..n {
                G[[i, j]] = Self::transfer(wave_source[j].pos, fp, wave_length);
            }
        }
        let B = if include_amp {
            let m = stack![Axis(1), G, -P];
            stack![Axis(1), m, Complex::new(0.0, -1.0) * G]
        } else {
            stack![Axis(1), G, -P]
        };
        Self::adjoint(&B).dot(&B)
    }

    #[allow(non_snake_case)]
    fn make_T(x: &Array1<f64>, n: usize, m: usize, include_amp: bool) -> Array2<Complex> {
        if include_amp {
            let mut T = Array2::zeros((2 * n + m, 1));
            for i in 0..n {
                T[[i, 0]] = x[n + m + i] * Complex::new(0.0, -x[i]).exp();
            }
            for i in 0..m {
                T[[n + i, 0]] = Complex::new(0.0, -x[n + i]).exp();
            }
            for i in 0..n {
                T[[n + m + i, 0]] = Complex::new(0.0, -x[i]).exp();
            }
            T
        } else {
            let mut T = Array2::zeros((n + m, 1));
            for i in 0..(n + m) {
                T[[i, 0]] = Complex::new(0.0, -x[i]).exp();
            }
            T
        }
    }

    #[allow(non_snake_case)]
    fn calc_JtJ_Jtf(
        BhB: &Array2<Complex>,
        T: &Array2<Complex>,
        n_m: usize,
    ) -> (Array2<f64>, Array1<f64>) {
        let TTh = T.dot(&Self::adjoint(&T));
        let BhB_TTh = BhB * &TTh;
        let JtJ = BhB_TTh.mapv(|c| c.re);
        let Jtf = Self::sum_col(&BhB_TTh, n_m);
        (JtJ, Jtf)
    }

    #[allow(non_snake_case)]
    fn calc_Fx(
        BhB: &Array2<Complex>,
        x: &Array1<f64>,
        include_amp: bool,
        n: usize,
        m: usize,
    ) -> f64 {
        let mut t = Array2::zeros((n + m, 1));
        if include_amp {
            for i in 0..n {
                t[[i, 0]] = x[n + m + i] * Complex::new(0.0, x[i]).exp();
            }
            for i in 0..m {
                t[[n + i, 0]] = Complex::new(0.0, x[n + i]).exp();
            }
            Self::adjoint(&t)
                .dot(&BhB.slice(s![..(n + m), ..(n + m)]))
                .dot(&t)[[0, 0]]
            .re
        } else {
            for i in 0..(n + m) {
                t[[i, 0]] = Complex::new(0.0, x[i]).exp();
            }
            Self::adjoint(&t).dot(BhB).dot(&t)[[0, 0]].re
        }
    }
}

impl Optimizer for LM {
    #[allow(non_snake_case, clippy::many_single_char_names)]
    fn optimize(&self, wave_source: &mut [WaveSource], include_amp: bool, normalize: bool) {
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let m = foci.len();
        let n = num_trans;

        let n_param = if include_amp { 2 * n + m } else { n + m };

        let x0: ArrayBase<OwnedRepr<f64>, _> = Array::zeros(n_param);
        let I: ArrayBase<OwnedRepr<f64>, _> = Array::eye(n_param);

        let BhB = Self::make_BhB(amps, foci, wave_source, n, m, include_amp, self.wave_length);

        let mut x = x0;
        let mut nu = 0.0;

        let T = Self::make_T(&x, n, m, include_amp);
        let (mut A, mut g) = Self::calc_JtJ_Jtf(&BhB, &T, n + m);
        let A_max: f64 = {
            let mut tmp = f64::NEG_INFINITY;
            for i in 0..(n + m) {
                tmp = tmp.max(A[[i, i]]);
            }
            tmp
        };
        let mut mu = TAU * A_max;
        let mut found = g.norm_max() <= EPS_1;
        let mut Fx = Self::calc_Fx(&BhB, &x, include_amp, n, m);
        for _ in 0..K_MAX {
            if found {
                break;
            }

            let h_lm = -(&A + &(mu * &I)).solve(&g).unwrap();
            if h_lm.norm() <= EPS_2 * (x.norm() + EPS_2) {
                found = true;
            } else {
                let x_new = &x + &h_lm;
                let Fx_new = Self::calc_Fx(&BhB, &x_new, include_amp, n, m);
                let L0_Lhlm = 0.5 * h_lm.t().dot(&(mu * &h_lm - &g));
                let rho = (Fx - Fx_new) / L0_Lhlm;
                Fx = Fx_new;
                if rho > 0.0 {
                    x = x_new;
                    let T = Self::make_T(&x, n, m, include_amp);
                    let (A_new, g_new) = Self::calc_JtJ_Jtf(&BhB, &T, n + m);
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
            let amp = match (include_amp, normalize) {
                (false, _) => 1.0,
                (_, true) => {
                    let mut max_coeff: f64 = f64::NEG_INFINITY;
                    for i in 0..n {
                        max_coeff = max_coeff.max(x[n + m + i].abs());
                    }
                    x[n + m + j] / max_coeff
                }
                (_, false) => x[n + m + j].min(1.0).max(-1.0),
            };
            let phase = (x[j] + PI) % (2.0 * PI);
            wave_source[j].amp = amp as f32;
            wave_source[j].phase = phase as f32;
        }
    }
}
