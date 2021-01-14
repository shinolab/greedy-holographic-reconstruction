/*
 * File: cpu_calculator.rs
 * Project: calculator
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use rayon::prelude::*;

use super::*;
use crate::{
    buffer::{AmplitudeFieldBuffer, ComplexFieldBufferScatter, FieldBuffer, IntensityFieldBuffer},
    utils::transfer,
    wave_source::WaveSource,
    Complex, PI,
};

pub struct CpuCalculator {
    sources: Vec<WaveSource>,
    wave_num: f32,
}

impl CpuCalculator {
    pub fn new() -> CpuCalculator {
        CpuCalculator {
            sources: vec![],
            wave_num: 2.0 * PI / 8.5,
        }
    }
}

impl std::default::Default for CpuCalculator {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! calc_from_complex_wave {
    ($val: ident, $exp: expr, $self: ident, $buffer: ident) => {{
        let wave_num = $self.wave_num;
        $buffer
            .observe_points()
            .collect::<Vec<_>>()
            .par_iter()
            .map(|&observe_point| {
                let mut $val = Complex::new(0., 0.);
                for source in $self.sources.iter() {
                    $val += transfer(source.pos, observe_point, wave_num);
                }
                $exp
            })
            .collect_into_vec($buffer.buffer_mut());
    }};
}

impl Calculator for CpuCalculator {
    fn init_wave_sources(&mut self, n: usize) {
        self.sources = vec![WaveSource::default(); n];
    }

    fn add_wave_sources(&mut self, sources: &[WaveSource]) {
        self.sources.extend_from_slice(sources);
    }

    fn wave_sources(&mut self) -> &mut [WaveSource] {
        &mut self.sources
    }

    fn update_amp_phase(&mut self) {}
    fn update_source_geometry(&mut self) {}

    fn set_wave_number(&mut self, wave_num: f32) {
        self.wave_num = wave_num;
    }
}

impl ComplexFieldCalculator for CpuCalculator {
    fn calc_complex(&self, buffer: &mut ComplexFieldBufferScatter) {
        calc_from_complex_wave!(p, p, self, buffer);
    }
}

impl IntensityFieldCalculator for CpuCalculator {
    fn calc_intensity(&self, buffer: &mut dyn IntensityFieldBuffer) {
        calc_from_complex_wave!(p, p.norm_sqr(), self, buffer);
    }
}

impl AmplitudeFieldCalculator for CpuCalculator {
    fn calc_amp(&self, buffer: &mut dyn AmplitudeFieldBuffer) {
        calc_from_complex_wave!(p, p.norm_sqr().sqrt(), self, buffer);
    }
}
