/*
 * File: cpu_calculator.rs
 * Project: calculator
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/06/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use rayon::prelude::*;

use super::*;
use crate::buffer::{
    AmplitudeFieldBuffer, ComplexFieldBufferScatter, FieldBuffer, IntensityFieldBuffer,
};
use crate::wave_source::WaveSource;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::f32::consts::PI;

#[derive(PartialEq, Debug)]
struct MinFloat(f32);
impl Eq for MinFloat {}
impl PartialOrd for MinFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl Ord for MinFloat {
    fn cmp(&self, other: &MinFloat) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct CpuCalculator {
    sources: Vec<WaveSource>,
    wave_num: f32,
    accurate_mode: bool,
}

impl CpuCalculator {
    pub fn new() -> CpuCalculator {
        CpuCalculator {
            sources: vec![],
            wave_num: 2.0 * PI / 8.5,
            accurate_mode: false,
        }
    }

    pub fn set_accurate_mode(&mut self, active: bool) {
        self.accurate_mode = active;
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
                let mut $val = num::Complex::new(0., 0.);
                for source in $self.sources.iter() {
                    let diff = observe_point - source.pos;
                    let dist = diff.norm();
                    let r = source.amp / dist;
                    let phi = source.phase - wave_num * dist;
                    $val += num::Complex::from_polar(r, phi);
                }
                $exp
            })
            .collect_into_vec($buffer.buffer_mut());
    }};
}

macro_rules! calc_from_complex_wave_accurate {
    ($val: ident, $exp: expr, $self: ident, $buffer: ident) => {{
        let wave_num = $self.wave_num;
        $buffer
            .observe_points()
            .collect::<Vec<_>>()
            .par_iter()
            .map(|&observe_point| {
                let mut re_heap = BinaryHeap::with_capacity($self.sources.len());
                let mut im_heap = BinaryHeap::with_capacity($self.sources.len());
                for source in $self.sources.iter() {
                    let diff = observe_point - source.pos;
                    let dist = diff.norm();
                    let r = source.amp / dist;
                    let phi = source.phase - wave_num * dist;
                    re_heap.push(MinFloat(r * phi.cos()));
                    im_heap.push(MinFloat(r * phi.sin()));
                }

                let mut re = 0.0;
                let mut im = 0.0;
                for (r, i) in re_heap.iter().zip(im_heap.iter()) {
                    re += r.0;
                    im += i.0;
                }
                let $val = num::Complex::new(re, im);
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
        if self.accurate_mode {
            calc_from_complex_wave_accurate!(p, p, self, buffer);
        } else {
            calc_from_complex_wave!(p, p, self, buffer);
        }
    }
}

impl IntensityFieldCalculator for CpuCalculator {
    fn calc_intensity(&self, buffer: &mut dyn IntensityFieldBuffer) {
        if self.accurate_mode {
            calc_from_complex_wave_accurate!(intensity, intensity.norm_sqr(), self, buffer);
        } else {
            calc_from_complex_wave!(p, p.norm_sqr(), self, buffer);
        }
    }
}

impl AmplitudeFieldCalculator for CpuCalculator {
    fn calc_amp(&self, buffer: &mut dyn AmplitudeFieldBuffer) {
        if self.accurate_mode {
            calc_from_complex_wave_accurate!(intensity, intensity.norm(), self, buffer);
        } else {
            calc_from_complex_wave!(p, p.norm_sqr().sqrt(), self, buffer);
        }
    }
}
