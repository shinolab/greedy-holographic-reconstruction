/*
 * File: traits.rs
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

use super::bounds::Bounds;
use super::dimension::Dimension;
use crate::Vector3;

pub trait FieldBuffer {
    type DataType;
    fn buffer(&self) -> &[Self::DataType];
    fn buffer_mut(&mut self) -> &mut Vec<Self::DataType>;
    fn observe_points(&self) -> Box<dyn Iterator<Item = Vector3>>;
    fn bounds(&self) -> Bounds;
    fn dimension(&self) -> Dimension;
}
