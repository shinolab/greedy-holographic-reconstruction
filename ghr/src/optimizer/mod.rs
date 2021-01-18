/*
 * File: mod.rs
 * Project: optimizer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

pub mod ghrbf;
mod gs_pat;
mod horn;
mod levenberg_marquardt;
mod long;

pub use ghrbf::*;
pub use gs_pat::GSPAT;
pub use horn::Horn;
pub use levenberg_marquardt::LM;
pub use long::Long;

use crate::wave_source::WaveSource;

pub trait Optimizer {
    fn optimize(&self, wave_source: &mut [WaveSource], include_amp: bool);
}
