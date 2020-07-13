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

    #[allow(non_snake_case)]
    fn make_JtJ_Jtf_theta(
        BhB: &Array2<Complex>,
        theta: &Array1<f64>,
    ) -> (Array2<f64>, Array1<f64>) {
        let T = theta
            .mapv(|a| Complex::new(0.0, -a).exp())
            .into_shape((theta.len(), 1))
            .unwrap();
        let TTt = T.dot(&Self::adjoint(&T));
        let BhB_TTt = BhB * &TTt;
        let JtJ = BhB_TTt.mapv(|c| c.re);
        let Jtf = Self::sum_col(&BhB_TTt, BhB_TTt.shape()[1]);
        (JtJ, Jtf)
    }

    #[allow(non_snake_case)]
    fn make_JtJ_Jtf_a(
        GhG: &Array2<Complex>,
        GhP: &Array2<Complex>,
        theta: &Array1<f64>,
        a: &Array1<f64>,
    ) -> (Array2<f64>, Array1<f64>) {
        let bphi = theta
            .slice(s![..a.len()])
            .mapv(|a| Complex::new(0.0, a).exp());
        let bphii = Array::from_diag(
            &theta
                .slice(s![..a.len()])
                .mapv(|a| Complex::new(0.0, -a).exp()),
        );
        let bpsi = theta
            .slice(s![a.len()..])
            .mapv(|a| Complex::new(0.0, a).exp());
        let JtJ = bphii.dot(GhG).dot(&Array2::from_diag(&bphi)).mapv(|a| a.re);
        let Jtf = bphii
            .dot(GhG)
            .dot(&Array2::from_diag(&a).mapv(|c| Complex::new(c, 0.)))
            .dot(&bphi)
            .mapv(|c| c.re)
            - &bphii.dot(GhP).dot(&bpsi).mapv(|c| c.re);
        (JtJ, Jtf)
    }

    #[allow(non_snake_case, clippy::many_single_char_names)]
    pub fn optimize_amp_phase(&self, wave_source: &mut [WaveSource], normalize: bool) {
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let m = foci.len();
        let n = num_trans;

        let theta_0: ArrayBase<OwnedRepr<f64>, _> = Array::zeros(n + m);
        let Inm: ArrayBase<OwnedRepr<f64>, _> = Array::eye(n + m);
        let a_0: ArrayBase<OwnedRepr<f64>, _> = Array::ones(n);
        let In: ArrayBase<OwnedRepr<f64>, _> = Array::eye(n);

        let mut P = Array::zeros((m, m));
        let mut G = Array::zeros((m, n));
        for i in 0..m {
            P[[i, i]] = Complex::new(amps[i], 0.0);
            let fp = foci[i];
            for j in 0..n {
                G[[i, j]] = self.transfer(wave_source[j].pos, fp);
            }
        }

        let mut theta = theta_0;
        let mut a = a_0;

        let mut nu_theta = 0.0;
        let mut nu_a = 0.0;

        let A = Array2::from_diag(&a.mapv(|r| Complex::new(r, 0.0)));

        let GhG = Self::adjoint(&G).dot(&G);
        let GhP = Self::adjoint(&G).dot(&P);

        let B = stack![Axis(1), G.dot(&A), -&P];
        let BhB = Self::adjoint(&B).dot(&B);

        let (mut JtJ_theta, mut Jtf_theta) = Self::make_JtJ_Jtf_theta(&BhB, &theta);
        let (mut JtJ_a, mut Jtf_a) = Self::make_JtJ_Jtf_a(&GhG, &GhP, &theta, &a);

        let mut mu_theta = {
            let mut tmp: f64 = 0.0;
            for i in 0..(n + m) {
                tmp = tmp.max(JtJ_theta[[i, i]]);
            }
            TAU * tmp
        };
        let mut mu_a = {
            let mut tmp: f64 = 0.0;
            for i in 0..n {
                tmp = tmp.max(JtJ_a[[i, i]]);
            }
            TAU * tmp
        };

        let mut found_theta = Jtf_theta.norm_max() <= EPS_1;
        let mut found_a = Jtf_a.norm_max() <= EPS_1;
        let exp_theta = theta
            .slice(s![0..(n + m)])
            .mapv(|a| Complex::new(0.0, a).exp())
            .into_shape((n + m, 1))
            .unwrap();
        let mut Fx = Self::adjoint(&exp_theta).dot(&BhB).dot(&exp_theta)[[0, 0]].abs();
        for _ in 0..K_MAX {
            if found_theta && found_a {
                break;
            }
            let h_lm_theta = -(&JtJ_theta + &(mu_theta * &Inm)).solve(&Jtf_theta).unwrap();
            let h_lm_a = -(&JtJ_a + &(mu_a * &In)).solve(&Jtf_a).unwrap();
            {
                let theta_new = &theta + &h_lm_theta;
                let a_new = (&a + &h_lm_a).mapv(|v| v.max(0.0));
                let exp_theta = theta_new
                    .slice(s![0..(n + m)])
                    .mapv(|a| Complex::new(0.0, a).exp())
                    .into_shape((n + m, 1))
                    .unwrap();
                let A = Array2::from_diag(&a_new.mapv(|r| Complex::new(r, 0.0)));
                let B = stack![Axis(1), G.dot(&A), -&P];
                let BhB = Self::adjoint(&B).dot(&B);

                let Fx_new = Self::adjoint(&exp_theta).dot(&BhB).dot(&exp_theta)[[0, 0]].abs();
                if h_lm_theta.norm() <= EPS_2 * (theta.norm() + EPS_2) {
                    found_theta = true;
                } else {
                    let L0_Lhlm = 0.5 * h_lm_theta.t().dot(&(mu_theta * &h_lm_theta - &Jtf_theta));
                    let rho_theta = (Fx - Fx_new) / L0_Lhlm;
                    if rho_theta > 0.0 {
                        theta = theta_new;
                        let (JtJ_theta_new, Jtf_theta_new) = Self::make_JtJ_Jtf_theta(&BhB, &theta);
                        JtJ_theta = JtJ_theta_new;
                        Jtf_theta = Jtf_theta_new;
                        found_theta = Jtf_theta.norm_max() <= EPS_1;
                        mu_theta *= (1f64 / 3.).max(1. - (2. * rho_theta - 1.).pow(3.));
                        nu_theta = 2.0;
                    } else {
                        mu_theta *= nu_theta;
                        nu_theta *= 2.0;
                    }
                }

                if h_lm_a.norm() <= EPS_2 * (a.norm() + EPS_2) {
                    found_a = true;
                } else {
                    let L0_Lhlm = 0.5 * h_lm_a.t().dot(&(mu_a * &h_lm_a - &Jtf_a));
                    let rho = (Fx - Fx_new) / L0_Lhlm;
                    if rho > 0.0 {
                        a = a_new;
                        let (JtJ_a_new, Jtf_a_new) = Self::make_JtJ_Jtf_a(&GhG, &GhP, &theta, &a);
                        JtJ_a = JtJ_a_new;
                        Jtf_a = Jtf_a_new;
                        found_a = Jtf_a.norm_max() <= EPS_1;
                        mu_a *= (1f64 / 3.).max(1. - (2. * rho - 1.).pow(3.));
                        nu_a = 2.0;
                    } else {
                        mu_a *= nu_a;
                        nu_a *= 2.0;
                    }
                }
                Fx = Fx_new;
            }
        }

        let mut max_coeff: f64 = 0.0;
        for v in a.iter() {
            max_coeff = max_coeff.max(v.abs());
        }
        for j in 0..n {
            let amp = if normalize {
                a[j].abs() / max_coeff
            } else {
                a[j].abs().min(1.0)
            };
            let phase = (theta[j] + PI) % (2.0 * PI);
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
