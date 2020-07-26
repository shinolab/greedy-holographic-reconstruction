/*
 * File: gradient_descent.rs
 * Project: optimizer
 * Created Date: 26/07/2020
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

const EPS: f64 = PI / 256.0;
const K_MAX: usize = 10_000;

pub struct GD {
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    wave_length: f64,
}

impl GD {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, wave_length: f64) -> Self {
        Self {
            foci,
            amps,
            wave_length,
        }
    }
}

impl GD {
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
    fn calc_Jtf(BhB: &Array2<Complex>, T: &Array2<Complex>, n_m: usize) -> Array1<f64> {
        let TTh = T.dot(&Self::adjoint(&T));
        let BhB_TTh = BhB * &TTh;
        let Jtf = Self::sum_col(&BhB_TTh, n_m);
        Jtf
    }
}

impl Optimizer for GD {
    #[allow(non_snake_case, clippy::many_single_char_names)]
    fn optimize(&self, wave_source: &mut [WaveSource], include_amp: bool, normalize: bool) {
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let m = foci.len();
        let n = num_trans;
        let n_param = if include_amp { 2 * n + m } else { n + m };

        let mut x0: ArrayBase<OwnedRepr<f64>, _> = Array::zeros(n_param);
        if include_amp {
            for i in 0..n {
                x0[n + m + i] = 1.0;
            }
        };
        let BhB = Self::make_BhB(amps, foci, wave_source, n, m, include_amp, self.wave_length);

        let mut x = x0;

        #[allow(unused_assignments)]
        let mut found = false;
        for _ in 0..K_MAX {
            let T = Self::make_T(&x, n, m, include_amp);
            let Jtf = Self::calc_Jtf(&BhB, &T, n + m);
            found = Jtf.norm_max() <= EPS;
            if found {
                break;
            }
            x = &x - &(0.1 * Jtf);
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