use crate::float::Float;
use crate::optimizer::Optimizer;
use crate::utils::transfer;
use crate::wave_source::WaveSource;
use crate::Complex;
use crate::Vector3;
use crate::PI;

use ndarray::*;
use ndarray_linalg::*;

const REPEAT: usize = 100;

/// GS-PAT
pub struct GSPAT {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    wave_length: Float,
}

/// Reference
/// * Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on Graphics (TOG), 39(4):138–1, 2020.
///
/// Not yet been implemented with GPU.
impl GSPAT {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>, wave_length: Float) -> Self {
        Self {
            foci,
            amps,
            wave_length,
        }
    }
    pub fn set_wave_length(&mut self, wave_length: Float) {
        self.wave_length = wave_length;
    }
}

impl Optimizer for GSPAT {
    #[allow(non_snake_case, clippy::many_single_char_names)]
    fn optimize(&self, wave_source: &mut [WaveSource], include_amp: bool, normalize: bool) {
        let num_trans = wave_source.len();
        let foci = &self.foci;
        let amps = &self.amps;

        let wave_num = 2.0 * PI / self.wave_length;

        let m = foci.len();
        let n = num_trans;

        let mut G = Array::zeros((m, n));
        for i in 0..m {
            let fp = foci[i];
            for j in 0..n {
                G[[i, j]] = transfer(wave_source[j].pos, fp, wave_num);
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

        for _ in 0..REPEAT {
            for (i, v) in gamma
                .iter()
                .zip(p0.iter())
                .map(|(g, &p)| g / g.abs() * p)
                .enumerate()
            {
                p[i] = v;
            }
            gamma = R.dot(&p);
        }

        for (i, v) in gamma
            .iter()
            .zip(p0.iter())
            .map(|(g, &p)| g / (g.abs() * g.abs()) * p * p)
            .enumerate()
        {
            p[i] = v;
        }

        let q = B.dot(&p);

        let mut max_coeff: Float = 0.0;
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
