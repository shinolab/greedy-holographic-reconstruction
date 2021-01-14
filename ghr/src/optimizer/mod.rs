/*
 * File: mod.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

pub mod ghrbf;
mod gradient_descent;
mod gs_pat;
mod horn;
mod levenberg_marquardt;
mod long;

pub use ghrbf::*;
pub use gradient_descent::GD;
pub use gs_pat::GSPAT;
pub use horn::Horn;
pub use levenberg_marquardt::LM;
pub use long::Long;

use crate::wave_source::WaveSource;

pub trait Optimizer {
    fn optimize(&self, wave_source: &mut [WaveSource], include_amp: bool, normalize: bool);
}
