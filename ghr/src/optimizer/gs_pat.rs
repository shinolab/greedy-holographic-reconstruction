/*
 * File: gs_pat.rs
 * Project: optimizer
 * Created Date: 01/01/1970
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use crate::{
    float::Float, math_utils::c_norm, optimizer::Optimizer, utils::transfer,
    wave_source::WaveSource, Complex, Vector3,
};

use ndarray::*;

/// GS-PAT
pub struct GSPAT {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    repeat: usize,
}

/// Reference
/// * Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on Graphics (TOG), 39(4):138–1, 2020.
///
/// Not yet been implemented with GPU.
impl GSPAT {
    pub fn new(repeat: usize) -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            repeat,
        }
    }
}

impl Optimizer for GSPAT {
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

        let mut G = Array::zeros((m, n));
        for i in 0..m {
            let fp = foci[i];
            for j in 0..n {
                G[[i, j]] = transfer(wave_source[j].pos, fp);
            }
        }

        let mut B = Array::zeros((n, m));
        for i in 0..m {
            let mut denomi = 0.0;
            for j in 0..n {
                denomi += G[[i, j]].norm_sqr();
            }
            for j in 0..n {
                B[[j, i]] = Complex::new(amps[i], 0.0) * G[[i, j]].conj() / denomi;
            }
        }

        let R = G.dot(&B);

        let mut p0: ArrayBase<OwnedRepr<Complex>, _> = Array::zeros(m);
        for i in 0..m {
            p0[i] = Complex::new(amps[i], 0.);
        }
        let mut p = p0.clone();
        let mut gamma = R.dot(&p);

        for _ in 0..self.repeat {
            for (i, v) in gamma
                .iter()
                .zip(p0.iter())
                .map(|(&g, &p)| g / c_norm(g) * p)
                .enumerate()
            {
                p[i] = v;
            }
            gamma = R.dot(&p);
        }

        for (i, v) in gamma
            .iter()
            .zip(p0.iter())
            .map(|(g, &p)| g / (g.norm_sqr()) * p * p)
            .enumerate()
        {
            p[i] = v;
        }

        let q = B.dot(&p);

        for j in 0..n {
            let amp = c_norm(q[j]).min(1.0);
            wave_source[j].amp = amp;
            wave_source[j].phase = q[j];
        }
    }
}
