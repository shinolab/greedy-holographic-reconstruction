/*
 * File: traits.rs
 * Project: scalar_filed_buffer
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/06/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::buffer::traits::FieldBuffer;
pub trait ScalarFieldBuffer: FieldBuffer<DataType = f32> {
    fn max(&self) -> f32 {
        self.buffer().iter().fold(f32::NAN, |m, v| v.max(m))
    }
}
