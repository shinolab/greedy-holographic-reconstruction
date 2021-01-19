/*
 * File: profile.rs
 * Project: examples
 * Created Date: 01/01/1970
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use ghr::{
    calculator::{Calculator, CpuCalculator},
    optimizer::*,
    math_utils::*,
    wave_source::WaveSource,
    Float, PI,
};

use std::time::Instant;

const NUM_SOURCE_X: usize = 16;
const NUM_SOURCE_Y: usize = 16;
const SOURCE_SIZE: Float = 10.18;

fn main() {
    let focus_z = 150.0;
    let focal_pos = [
        SOURCE_SIZE * (NUM_SOURCE_X - 1) as Float / 2.0,
        SOURCE_SIZE * (NUM_SOURCE_Y - 1) as Float / 2.0,
        focus_z,
    ];

    let mut calculator = CpuCalculator::new();

    let mut transducers = Vec::new();
    for y in 0..NUM_SOURCE_Y {
        for x in 0..NUM_SOURCE_X {
            let pos = [SOURCE_SIZE * x as Float, SOURCE_SIZE * y as Float, 0.];
            transducers.push(WaveSource::new(pos, 0.0, 0.0));
        }
    }
    calculator.add_wave_sources(&transducers);

    let num = 64;
    let rad = 40.0;
    let mut target_pos = Vec::with_capacity(num);
    for i in 0..num {
        let t = 2. * PI * i as Float / num as Float;
        target_pos.push(add(focal_pos, [rad * t.cos(), rad * t.sin(), 0.]));
    }
    let mut amps = Vec::with_capacity(target_pos.len());
    for _ in 0..target_pos.len() {
        amps.push(1.0);
    }

    let mut optimizer = GreedyBruteForce::new(16, 1, false, false);
    optimizer.set_target_foci(&target_pos);
    optimizer.set_target_amps(&amps);

    for _ in 0..1000 {
        optimizer.optimize(calculator.wave_sources());
    }
    let start = Instant::now();
    optimizer.optimize(calculator.wave_sources());
    println!("GBS: time={} us", start.elapsed().as_micros());
}
