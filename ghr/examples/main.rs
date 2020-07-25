/*
 * File: main.rs
 * Project: examples
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use ghr::buffer::{generator::*, BufferBuilder};
use ghr::calculator::{Calculate, Calculator, CpuCalculator};
use ghr::optimizer::*;
use ghr::vec_utils::*;
use ghr::wave_source::WaveSource;

use image::png::PNGEncoder;
use image::ColorType;

use std::f32::consts::PI;
use std::fs::File;

const NUM_SOURCE_X: usize = 18;
const NUM_SOURCE_Y: usize = 14;
const SOURCE_SIZE: f32 = 10.18;
const WAVE_LENGTH: f32 = 8.5;

macro_rules! write_image {
    ($filename: tt, $buffer: ident, $bb: ident) => {
        let output = File::create($filename).unwrap();
        let max = $buffer.max();
        let pixels: Vec<_> = $buffer
            .buffer()
            .chunks_exact($bb.0)
            .rev()
            .flatten()
            .map(|v| ((v / max) * 255.) as u8)
            .collect();

        let encoder = PNGEncoder::new(output);
        encoder
            .encode(&pixels, $bb.0 as u32, $bb.1 as u32, ColorType::L8)
            .unwrap();
    };
}

fn main() {
    let focus_z = 150.0;
    let focal_pos = [
        SOURCE_SIZE * (NUM_SOURCE_X - 1) as f32 / 2.0,
        SOURCE_SIZE * (NUM_SOURCE_Y - 1) as f32 / 2.0,
        focus_z,
    ];
    let obs_range = 100.0;

    let mut calculator = CpuCalculator::new();
    // calculator.set_accurate_mode(true);
    calculator.set_wave_number(2.0 * PI / WAVE_LENGTH);

    let mut transducers = Vec::new();
    for y in 0..NUM_SOURCE_Y {
        for x in 0..NUM_SOURCE_X {
            let pos = [SOURCE_SIZE * x as f32, SOURCE_SIZE * y as f32, 0.];
            transducers.push(WaveSource::new(pos, 0.0, 0.0));
        }
    }
    calculator.add_wave_sources(&transducers);

    let num = 5;
    let rad = 40.0;
    let mut target_pos = Vec::with_capacity(num);
    for i in 0..num {
        let t = 2. * PI * i as f32 / num as f32;
        target_pos.push(add(focal_pos, [rad * t.cos(), rad * t.sin(), 0.]));
    }
    let mut amps = Vec::with_capacity(target_pos.len());
    for _ in 0..target_pos.len() {
        amps.push(1.0);
    }

    let mut buffer = BufferBuilder::new()
        .x_range(
            focal_pos[0] - obs_range / 2.0,
            focal_pos[0] + obs_range / 2.0,
        )
        .y_range(
            focal_pos[1] - obs_range / 2.0,
            focal_pos[1] + obs_range / 2.0,
        )
        .z_at(focus_z)
        .resolution(1.)
        .generate::<Amplitude>();

    let bounds = buffer.bounds();
    let bb = (bounds.x(), bounds.y());

    let optimizer = GreedyBruteForce::new(target_pos.clone(), amps.clone(), WAVE_LENGTH as f64);
    optimizer.optimize(calculator.wave_sources(), true, true);
    buffer.calculate(&calculator);
    println!("GBS: {}", buffer.max());
    write_image!("xy_ghr_p.png", buffer, bb);

    let horn = Horn::new(target_pos.clone(), amps.clone(), WAVE_LENGTH as f64);
    horn.optimize(calculator.wave_sources(), true, true);
    buffer.calculate(&calculator);
    println!("HORN: {}", buffer.max());
    write_image!("xy_horn.png", buffer, bb);

    let long = Long::new(target_pos.clone(), amps.clone(), WAVE_LENGTH as f64);
    long.optimize(calculator.wave_sources(), true, true);
    buffer.calculate(&calculator);
    println!("LONG: {}", buffer.max());
    write_image!("xy_long.png", buffer, bb);

    let lm = LM::new(target_pos, amps, WAVE_LENGTH as f64);
    lm.optimize(calculator.wave_sources(), false, true);
    buffer.calculate(&calculator);
    println!("LM: {}", buffer.max());
    write_image!("xy_lm.png", buffer, bb);
}
