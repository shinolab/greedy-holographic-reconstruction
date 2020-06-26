/*
 * File: intensity_field_buffer.rs
 * Project: buffer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/06/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use std::ops::DerefMut;

use super::scalar_filed_buffer::{
    ScalarFieldBuffer, ScalarFieldBuffer1D, ScalarFieldBuffer2D, ScalarFieldBuffer3D,
};
use crate::calculator::*;

pub trait IntensityFieldBuffer: ScalarFieldBuffer {}
impl IntensityFieldBuffer for ScalarFieldBuffer1D {}
impl IntensityFieldBuffer for ScalarFieldBuffer2D {}
impl IntensityFieldBuffer for ScalarFieldBuffer3D {}
impl<C> Calculate<C> for dyn IntensityFieldBuffer
where
    C: IntensityFieldCalculator + ?Sized,
{
    fn calculate(&mut self, calculator: &C) {
        calculator.calc_intensity(self);
    }
}
impl<C> Calculate<C> for Box<dyn IntensityFieldBuffer>
where
    C: IntensityFieldCalculator + ?Sized,
{
    fn calculate(&mut self, calculator: &C) {
        calculator.calc_intensity(self.deref_mut());
    }
}
