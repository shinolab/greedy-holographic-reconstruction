/*
 * File: levenberg_marquardt.rs
 * Project: optimizer
 * Created Date: 06/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    optimizer::Optimizer, utils::transfer, wave_source::WaveSource, Complex, Float, Vector3, PI,
};

use ndarray::{linalg::*, *};
use ndarray_linalg::*;

pub struct LM {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    eps_1: Float,
    eps_2: Float,
    tau: Float,
    k_max: usize,
}

impl LM {
    pub fn new(eps_1: Float, eps_2: Float, tau: Float, k_max: usize) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            eps_1,
            eps_2,
            tau,
            k_max,
        }
    }

    fn adjoint(m: &Array2<Complex>) -> Array2<Complex> {
        m.t().mapv(|c| c.conj())
    }

    #[allow(non_snake_case)]
    fn make_BhB(
        amps: &[Float],
        foci: &[Vector3],
        wave_source: &mut [WaveSource],
        n: usize,
        m: usize,
    ) -> Array2<Complex> {
        let mut P = Array::zeros((m, m));
        let mut G = Array::zeros((m, n));
        for i in 0..m {
            P[[i, i]] = Complex::new(amps[i], 0.0);
            let fp = foci[i];
            for j in 0..n {
                G[[i, j]] = transfer(wave_source[j].pos, fp);
            }
        }
        let B = stack![Axis(1), G, -P];
        Self::adjoint(&B).dot(&B)
    }

    #[allow(non_snake_case)]
    fn make_T(T: &mut Array2<Complex>, x: &Array1<Float>, n_m: usize) {
        for i in 0..n_m {
            T[[i, 0]] = Complex::new(0.0, -x[i]).exp();
        }
    }

    #[allow(non_snake_case)]
    fn calc_JtJ_Jtf(
        JtJ: &mut Array2<Float>,
        Jtf: &mut Array1<Float>,
        BhB: &Array2<Complex>,
        T: &Array2<Complex>,
        n_m: usize,
        tmp_mat: &mut Array2<Complex>,
    ) {
        general_mat_mul(
            Complex::new(1., 0.),
            &T,
            &Self::adjoint(&T),
            Complex::new(0., 0.),
            tmp_mat,
        );
        for row in 0..n_m {
            let mut im = 0.0;
            for col in 0..n_m {
                let tmp = BhB[[row, col]] * tmp_mat[[row, col]];
                JtJ[[row, col]] = tmp.re;
                im += tmp.im;
            }
            Jtf[row] = im;
        }
    }

    #[allow(non_snake_case)]
    fn calc_Fx(BhB: &Array2<Complex>, x: &Array1<Float>, n_m: usize) -> Float {
        let mut t = Array2::zeros((n_m, 1));
        for i in 0..n_m {
            t[[i, 0]] = Complex::new(0.0, x[i]).exp();
        }
        Self::adjoint(&t).dot(BhB).dot(&t)[[0, 0]].re
    }
}

impl Optimizer for LM {
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

        let n_param = n + m;

        let mut x0: ArrayBase<OwnedRepr<Float>, _> = Array::zeros(n_param);

        use rand::Rng;
        let mut rng = rand::thread_rng();
        for i in 0..n_param {
            x0[i] = rng.gen::<Float>() * 2.0 * PI;
        }

        let I: ArrayBase<OwnedRepr<Float>, _> = Array::eye(n_param);

        let BhB = Self::make_BhB(amps, foci, wave_source, n, m);

        let mut x = x0;
        let mut nu = 2.0;

        let mut T = Array::zeros((n_param, 1));
        Self::make_T(&mut T, &x, n_param);

        let mut A = Array::zeros((n_param, n_param));
        let mut g = Array::zeros(n_param);
        let mut tmp = Array::zeros((n_param, n_param));
        Self::calc_JtJ_Jtf(&mut A, &mut g, &BhB, &T, n_param, &mut tmp);
        let A_max: Float = {
            let mut tmp = Float::NEG_INFINITY;
            for i in 0..n_param {
                tmp = tmp.max(A[[i, i]]);
            }
            tmp
        };
        let mut mu = self.tau * A_max;
        let mut found = g.norm_max() <= self.eps_1;
        let mut Fx = Self::calc_Fx(&BhB, &x, n_param);
        let mut x_new;
        for _ in 0..self.k_max {
            if found {
                break;
            }

            let h_lm = -(&A + &(mu * &I)).solveh(&g).unwrap();
            if h_lm.norm() <= self.eps_2 * (x.norm() + self.eps_2) {
                found = true;
            } else {
                x_new = &x + &h_lm;
                let Fx_new = Self::calc_Fx(&BhB, &x_new, n_param);
                let L0_Lhlm = 0.5 * h_lm.t().dot(&(mu * &h_lm - &g));
                let rho = (Fx - Fx_new) / L0_Lhlm;
                Fx = Fx_new;
                if rho > 0.0 {
                    x = x_new;
                    Self::make_T(&mut T, &x, n_param);
                    Self::calc_JtJ_Jtf(&mut A, &mut g, &BhB, &T, n_param, &mut tmp);
                    found = g.norm_max() <= self.eps_1;
                    mu *= (1f64 / 3.).max(1. - (2. * rho - 1.).pow(3.));
                    nu = 2.0;
                } else {
                    mu *= nu;
                    nu *= 2.0;
                }
            }
        }

        for j in 0..n {
            wave_source[j].q = Complex::new(0., x[j]).exp();
        }
    }
}
