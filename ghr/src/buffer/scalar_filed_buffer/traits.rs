/*
 * File: traits.rs
 * Project: scalar_filed_buffer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{buffer::traits::FieldBuffer, Float};

pub trait ScalarFieldBuffer: FieldBuffer<DataType = Float> {
    fn max(&self) -> Float {
        self.buffer().iter().fold(Float::NAN, |m, v| v.max(m))
    }
}
