/*
 * File: horn.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use std::f64::consts::PI;

use crate::optimizer::Optimizer;
use crate::vec_utils::*;
use crate::wave_source::WaveSource;
use crate::Vector3;

use num_traits::identities::Zero;
use rand::{thread_rng, Rng};

use ndarray::*;
use ndarray_linalg::*;

type Complex = c64;

const REPEAT_SDP: usize = 1000;
const LAMBDA_SDP: f64 = 0.8;

pub struct Horn {
    foci: Vec<Vector3>,
    amps: Vec<f64>,
    wave_length: f64,
    repeat: usize,
    lambda: f64,
}

impl Horn {
    pub fn new(foci: Vec<Vector3>, amps: Vec<f64>, wave_length: f64) -> Self {
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
        let diff = sub(target_pos, trans_pos);
        let dist = norm(diff);

        1.0 / dist as f64 * (Complex::new(0., -2. * PI / wave_length * dist as f64)).exp()
    }

    fn adjoint(m: &Array2<Complex>) -> Array2<Complex> {
        m.t().mapv(|c| c.conj())
    }

    fn remove_row<T>(m: &Array2<T>, i: isize) -> Array2<T>
    where
        T: Clone + Zero,
    {
        let shape = m.shape();
        let row = shape[0] - 1;
        let col = shape[1];
        let mut r = Array::zeros((row, col));
        r.slice_mut(s![0..i, ..]).assign(&m.slice(s![0..i, ..]));
        r.slice_mut(s![i..row as isize, ..])
            .assign(&m.slice(s![(i + 1)..(row as isize + 1), ..]));
        r
    }

    fn remove_row_1d<T>(m: &ArrayBase<ViewRepr<&T>, Dim<[usize; 1]>>, i: isize) -> Array1<T>
    where
        T: Clone + Zero,
    {
        let shape = m.shape();
        let row = shape[0] - 1;
        let mut r = Array::zeros(row);
        r.slice_mut(s![0..i]).assign(&m.slice(s![0..i]));
        r.slice_mut(s![i..row as isize])
            .assign(&m.slice(s![(i + 1)..(row as isize + 1)]));
        r
    }

    fn remove_col<T>(m: &Array2<T>, i: isize) -> Array2<T>
    where
        T: Clone + Zero,
    {
        let shape = m.shape();
        let row = shape[0];
        let col = shape[1] - 1;
        let mut r = Array::zeros((row, col));
        r.slice_mut(s![.., 0..i]).assign(&m.slice(s![.., 0..i]));
        r.slice_mut(s![.., i..col as isize])
            .assign(&m.slice(s![.., (i + 1)..(col as isize + 1)]));
        r
    }
}
impl Optimizer for Horn {
    #[allow(clippy::many_single_char_names)]
    fn optimize(&self, wave_source: &mut [WaveSource], include_amp: bool, normalize: bool) {
        let mut rng = thread_rng();
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let alpha = 1e-3;
        let m = foci.len();
        let n = num_trans;
        let mut b = Array::zeros((m, n));
        let mut p = Array::zeros((m, m));
        for i in 0..m {
            p[[i, i]] = Complex::new(amps[i], 0.);
            let tp = foci[i];
            for j in 0..n {
                b[[i, j]] = self.transfer(wave_source[j].pos, tp);
            }
        }

        let (u, s, vt) = b.svd(true, true).unwrap();
        let mut singular_values_inv_mat = Array::zeros((n, m));
        for i in 0..m.min(n) {
            let r = s[i] / (s[i] * s[i] + alpha * alpha);
            singular_values_inv_mat[[i, i]] = Complex::new(r, 0.0);
        }
        let u = u.unwrap();
        let vt = vt.unwrap();
        let pinv_b = Self::adjoint(&vt)
            .dot(&singular_values_inv_mat)
            .dot(&Self::adjoint(&u));

        let mm = p.dot(&(Array::eye(m) - b.dot(&pinv_b))).dot(&p);
        let mut x = Array::eye(m);

        let lambda = self.lambda;
        for _ in 0..self.repeat {
            let ii = (m as f64 * rng.gen_range(0., 1.)) as isize;
            let xc = Self::remove_row(&x, ii);
            let xc = Self::remove_col(&xc, ii);
            let mmc = Self::remove_row_1d(&mm.column(ii as usize), ii);
            let xb = xc * &mmc;
            let gamma = Self::adjoint(&xb).dot(&mmc);
            let gamma = gamma[0];
            if gamma.re > 0. {
                let xb = xb * (-(lambda / gamma.re).sqrt());
                x.slice_mut(s![ii, 0..ii])
                    .assign(&xb.slice(s![0, 0..ii]).mapv(|c| c.conj()));
                x.slice_mut(s![ii, (ii + 1)..])
                    .assign(&xb.slice(s![ii.., 0]).mapv(|c| c.conj()));
                x.slice_mut(s![0..ii, ii]).assign(&xb.slice(s![0..ii, 0]));
                x.slice_mut(s![(ii + 1).., ii])
                    .assign(&xb.slice(s![ii.., 0]));
            } else {
                let z1 = Array::zeros(ii as usize);
                let z2 = Array::zeros(m - ii as usize - 1);
                x.slice_mut(s![ii, 0..ii]).assign(&z1);
                x.slice_mut(s![ii, (ii + 1)..]).assign(&z2);
                x.slice_mut(s![0..ii, ii]).assign(&z1);
                x.slice_mut(s![(ii + 1).., ii]).assign(&z2);
            }
        }

        let (evs, vecs) = x.eigh(UPLO::Upper).unwrap();
        let mut abs_eiv = 0.;
        let mut idx = 0;
        for j in 0..evs.len() {
            let eiv = evs[j].abs();
            if abs_eiv < eiv {
                abs_eiv = eiv;
                idx = j;
            }
        }

        let u = vecs.column(idx);
        let q = pinv_b.dot(&p).dot(&u);
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
