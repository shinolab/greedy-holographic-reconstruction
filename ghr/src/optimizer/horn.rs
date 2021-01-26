/*
 * File: horn.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    math_utils::c_norm, optimizer::Optimizer, utils::transfer, wave_source::WaveSource, Complex,
    Float, Vector3,
};

use rand::{thread_rng, Rng};

use ndarray::*;
use ndarray_linalg::*;

pub struct Horn {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    repeat: usize,
    alpha: Float,
    lambda: Float,
}

impl Horn {
    pub fn new(repeat: usize, alpha: Float, lambda: Float) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
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

    fn remove_row_1d<T>(dst: &mut Array1<T>, m: &ArrayBase<ViewRepr<&T>, Dim<[usize; 1]>>, i: isize)
    where
        T: Clone,
    {
        let shape = m.shape();
        let row = shape[0] - 1;
        dst.slice_mut(s![0..i]).assign(&m.slice(s![0..i]));
        dst.slice_mut(s![i..row as isize])
            .assign(&m.slice(s![(i + 1)..(row as isize + 1)]));
    }

    fn remove_row_col<T>(dst: &mut Array2<T>, m: &Array2<T>, i: isize)
    where
        T: Clone,
    {
        let shape = m.shape();
        let row = shape[0] as isize - 1;
        let col = shape[1] as isize - 1;
        dst.slice_mut(s![0..i, 0..i])
            .assign(&m.slice(s![0..i, 0..i]));
        dst.slice_mut(s![i..row, 0..i])
            .assign(&m.slice(s![(i + 1)..(row + 1), 0..i]));
        dst.slice_mut(s![0..i, i..col])
            .assign(&m.slice(s![0..i, (i + 1)..(col as isize + 1)]));
        dst.slice_mut(s![i..row, i..col])
            .assign(&m.slice(s![(i + 1)..(row + 1), (i + 1)..(col as isize + 1)]));
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
        for i in 0..m {
            p[[i, i]] = Complex::new(amps[i], 0.);
            let tp = foci[i];
            for j in 0..n {
                b[[i, j]] = transfer(wave_source[j].pos, tp);
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
        let mut mmc = Array::zeros(m - 1);

        let mut x = Array::eye(m);
        let mut xc = Array::zeros((m - 1, m - 1));
        let zero = Array::zeros(m);
        for _ in 0..self.repeat {
            let ii = rng.gen_range(0..m) as isize;
            Self::remove_row_col(&mut xc, &x, ii);
            Self::remove_row_1d(&mut mmc, &mm.column(ii as usize), ii);
            let xb = xc.dot(&mmc).into_shape((mmc.len(), 1)).unwrap();
            let gamma = Self::adjoint(&xb).dot(&mmc)[0];
            if gamma.re > 0.0 {
                let xb = xb * (-(self.lambda / gamma.re).sqrt());
                x.slice_mut(s![ii, 0..ii])
                    .assign(&xb.slice(s![0..ii, 0]).mapv(|c| c.conj()));
                x.slice_mut(s![ii, (ii + 1)..])
                    .assign(&xb.slice(s![ii.., 0]).mapv(|c| c.conj()));
                x.slice_mut(s![0..ii, ii]).assign(&xb.slice(s![0..ii, 0]));
                x.slice_mut(s![(ii + 1).., ii])
                    .assign(&xb.slice(s![ii.., 0]));
            } else {
                x.slice_mut(s![ii, 0..ii]).assign(&zero.slice(s![0..ii]));
                x.slice_mut(s![ii, (ii + 1)..])
                    .assign(&zero.slice(s![(ii + 1)..]));
                x.slice_mut(s![0..ii, ii]).assign(&zero.slice(s![0..ii]));
                x.slice_mut(s![(ii + 1).., ii])
                    .assign(&zero.slice(s![(ii + 1)..]));
            }
        }

        let (evs, vecs) = x.eig().unwrap();
        let mut abs_eiv = 0.;
        let mut idx = 0;
        for j in 0..evs.len() {
            let eiv = evs[j].norm_sqr();
            if abs_eiv < eiv {
                abs_eiv = eiv;
                idx = j;
            }
        }

        let u = vecs.column(idx);
        let q = pinv_b.dot(&p).dot(&u);

        let max_coef = q
            .iter()
            .fold(Float::NEG_INFINITY, |acc, x| acc.max(c_norm(*x)))
            .sqrt();
        for j in 0..n {
            wave_source[j].q = q[j] / max_coef;
        }
    }
}
