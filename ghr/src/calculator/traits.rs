/*
 * File: traits.rs
 * Project: calculator
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
    buffer::{AmplitudeFieldBuffer, ComplexFieldBufferScatter, IntensityFieldBuffer},
    wave_source::WaveSource,
};

pub trait Calculator {
    fn init_wave_sources(&mut self, n: usize);
    fn add_wave_sources(&mut self, sources: &[WaveSource]);
    fn wave_sources(&mut self) -> &mut [WaveSource];
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
