/*
 * File: bounds.rs
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

use std::ops::Index;

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    x: usize,
    y: usize,
    z: usize,
}

impl Bounds {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn z(&self) -> usize {
        self.z
    }

    pub fn size(&self) -> usize {
        self.x * self.y * self.z
    }
}

impl Index<usize> for Bounds {
    type Output = usize;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index is out of bounds."),
        }
    }
}
