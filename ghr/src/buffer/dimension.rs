/*
 * File: dimension.rs
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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Dimension {
    None,
    One(Axis),
    Two(Axis, Axis),
    Three(Axis, Axis, Axis),
}

impl Dimension {
    pub fn append(&mut self, new_axis: Axis) {
        match self {
            Dimension::None => *self = Dimension::One(new_axis),
            Dimension::One(f) => *self = Dimension::Two(*f, new_axis),
            Dimension::Two(f, s) => *self = Dimension::Three(*f, *s, new_axis),
            Dimension::Three(_, _, _) => unreachable!(),
        }
    }

    pub fn contains(self, axis: Axis) -> bool {
        match self {
            Dimension::None => false,
            Dimension::One(f) => f == axis,
            Dimension::Two(f, s) => f == axis || s == axis,
            Dimension::Three(f, s, t) => f == axis || s == axis || t == axis,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}
