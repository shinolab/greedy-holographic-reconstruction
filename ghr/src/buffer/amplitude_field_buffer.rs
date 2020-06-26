/*
 * File: amplitude_field_buffer.rs
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

pub trait AmplitudeFieldBuffer: ScalarFieldBuffer {}
impl AmplitudeFieldBuffer for ScalarFieldBuffer1D {}
impl AmplitudeFieldBuffer for ScalarFieldBuffer2D {}
impl AmplitudeFieldBuffer for ScalarFieldBuffer3D {}
impl<C> Calculate<C> for dyn AmplitudeFieldBuffer
where
    C: AmplitudeFieldCalculator + ?Sized,
{
    fn calculate(&mut self, calculator: &C) {
        calculator.calc_amp(self);
    }
}
impl<C> Calculate<C> for Box<dyn AmplitudeFieldBuffer>
where
    C: AmplitudeFieldCalculator + ?Sized,
{
    fn calculate(&mut self, calculator: &C) {
        calculator.calc_amp(self.deref_mut());
    }
}
