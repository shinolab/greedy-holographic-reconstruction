/*
 * File: main.rs
 * Project: examples
 * Created Date: 26/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use ghr::{
    buffer::{generator::*, BufferBuilder},
    calculator::{Calculate, Calculator, CpuCalculator},
    math_utils::*,
    optimizer::*,
    wave_source::WaveSource,
    Complex, Float, PI,
};

const NUM_SOURCE_X: usize = 16;
const NUM_SOURCE_Y: usize = 16;
const SOURCE_SIZE: Float = 10.18;

macro_rules! write_image {
    ($filename: tt, $buffer: ident, $bb: ident) => {
        let output = std::fs::File::create($filename).unwrap();
        let max = $buffer.max();
        let pixels: Vec<_> = $buffer
            .buffer()
            .chunks_exact($bb.0)
            .rev()
            .flatten()
            .map(|v| ((v / max) * 255.) as u8)
            .collect();

        let encoder = image::png::PngEncoder::new(output);
        encoder
            .encode(&pixels, $bb.0 as u32, $bb.1 as u32, image::ColorType::L8)
            .unwrap();
    };
}

fn main() {
    let focus_z = 150.0;
    let focal_pos = [
        SOURCE_SIZE * (NUM_SOURCE_X - 1) as Float / 2.0,
        SOURCE_SIZE * (NUM_SOURCE_Y - 1) as Float / 2.0,
        focus_z,
    ];
    let obs_range = 100.0;

    let mut calculator = CpuCalculator::new();

    let mut transducers = Vec::new();
    for y in 0..NUM_SOURCE_Y {
        for x in 0..NUM_SOURCE_X {
            let pos = [SOURCE_SIZE * x as Float, SOURCE_SIZE * y as Float, 0.];
            transducers.push(WaveSource::new(pos, Complex::new(0., 0.)));
        }
    }
    calculator.add_wave_sources(&transducers);

    let num = 5;
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

    std::fs::create_dir("img").unwrap_or(());

    let mut optimizer = GreedyBruteForce::new(16, 16, false);
    optimizer.set_target_foci(&target_pos);
    optimizer.set_target_amps(&amps);
    optimizer.optimize(calculator.wave_sources());
    buffer.calculate(&calculator);
    println!("GBS: {}", buffer.max());
    write_image!("img/xy_gbs.png", buffer, bb);

    let mut horn = Horn::new(1000, 1e-3, 0.9);
    horn.set_target_foci(&target_pos);
    horn.set_target_amps(&amps);
    horn.optimize(calculator.wave_sources());
    buffer.calculate(&calculator);
    println!("HORN: {}", buffer.max());
    write_image!("img/xy_horn.png", buffer, bb);

    let mut long = Long::new(1.0);
    long.set_target_foci(&target_pos);
    long.set_target_amps(&amps);
    long.optimize(calculator.wave_sources());
    buffer.calculate(&calculator);
    println!("LONG: {}", buffer.max());
    write_image!("img/xy_long.png", buffer, bb);

    let mut lm = LM::new(1e-8, 1e-8, 1e-3, 200);
    lm.set_target_foci(&target_pos);
    lm.set_target_amps(&amps);
    lm.optimize(calculator.wave_sources());
    buffer.calculate(&calculator);
    println!("LM: {}", buffer.max());
    write_image!("img/xy_lm.png", buffer, bb);

    let mut gspat = GSPAT::new(100);
    gspat.set_target_foci(&target_pos);
    gspat.set_target_amps(&amps);
    gspat.optimize(calculator.wave_sources());
    buffer.calculate(&calculator);
    println!("GS-PAT: {}", buffer.max());
    write_image!("img/xy_gspat.png", buffer, bb);
}
