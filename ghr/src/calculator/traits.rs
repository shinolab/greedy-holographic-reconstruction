/*
 * File: traits.rs
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

use crate::buffer::{AmplitudeFieldBuffer, ComplexFieldBufferScatter, IntensityFieldBuffer};
use crate::wave_source::WaveSource;

pub trait Calculator {
    fn init_wave_sources(&mut self, n: usize);
    fn add_wave_sources(&mut self, sources: &[WaveSource]);
    fn wave_sources(&mut self) -> &mut [WaveSource];
    fn update_amp_phase(&mut self);
    fn update_source_geometry(&mut self);
    fn set_wave_number(&mut self, wave_num: f32);
}

pub trait Calculate<C: ?Sized> {
    fn calculate(&mut self, calculator: &C);
}

pub trait ComplexFieldCalculator {
    fn calc_complex(&self, buffer: &mut ComplexFieldBufferScatter);
}

pub trait IntensityFieldCalculator {
    fn calc_intensity(&self, buffer: &mut dyn IntensityFieldBuffer);
}

pub trait AmplitudeFieldCalculator {
    fn calc_amp(&self, buffer: &mut dyn AmplitudeFieldBuffer);
}
