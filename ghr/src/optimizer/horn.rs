/*
 * File: horn.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    optimizer::Optimizer, utils::transfer, wave_source::WaveSource, Complex, Float, Vector3, PI,
};

use num_traits::identities::Zero;
use rand::{thread_rng, Rng};

use ndarray::*;
use ndarray_linalg::*;

pub struct Horn {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    wave_length: Float,
    repeat: usize,
    alpha: Float,
    lambda: Float,
}

impl Horn {
    pub fn new(repeat: usize, alpha: Float, lambda: Float, wave_length: Float) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            wave_length,
            repeat,
            alpha,
            lambda,
        }
    }
}

impl Horn {
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
    fn set_target_foci(&mut self, foci: &[Vector3]) {
        self.foci = foci.to_vec();
    }

    fn set_target_amps(&mut self, amps: &[Float]) {
        self.amps = amps.to_vec();
    }

    #[allow(clippy::many_single_char_names)]
    fn optimize(&self, wave_source: &mut [WaveSource]) {
        let mut rng = thread_rng();
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let alpha = self.alpha;
        let m = foci.len();
        let n = num_trans;
        let mut b = Array::zeros((m, n));
        let mut p = Array::zeros((m, m));
        let wave_num = 2.0 * PI / self.wave_length;
        for i in 0..m {
            p[[i, i]] = Complex::new(amps[i], 0.);
            let tp = foci[i];
            for j in 0..n {
                b[[i, j]] = transfer(wave_source[j].pos, tp, 1.0, 0.0, wave_num);
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
            let ii = rng.gen_range(0..m) as isize;
            let xc = Self::remove_row(&x, ii);
            let xc = Self::remove_col(&xc, ii);
            let mmc = Self::remove_row_1d(&mm.column(ii as usize), ii);
            let l = mmc.len();
            let xb = xc.dot(&mmc).into_shape((l, 1)).unwrap();
            let gamma = Self::adjoint(&xb).dot(&mmc);
            let gamma = gamma[0];
            if gamma.re > 0.0 {
                let xb = xb * (-(lambda / gamma.re).sqrt());
                x.slice_mut(s![ii, 0..ii])
                    .assign(&xb.slice(s![0..ii, 0]).mapv(|c| c.conj()));
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

        let (evs, vecs) = x.eig().unwrap();
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

        let max_coef = q
            .iter()
            .fold(Float::NEG_INFINITY, |acc, x| acc.max(x.abs()));
        for j in 0..n {
            let amp = q[j].abs() / max_coef;
            let phase = q[j].arg() + PI;
            wave_source[j].amp = amp;
            wave_source[j].phase = phase;
        }
    }
}
