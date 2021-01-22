/*
 * File: time.rs
 * Project: examples
 * Created Date: 09/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use ghr::{
    buffer::{generator::Amplitude, BufferBuilder},
    calculator::{Calculate, Calculator, CpuCalculator},
    consts::WAVE_LENGTH,
    math_utils::*,
    optimizer::*,
    wave_source::WaveSource,
    Complex, Float, Vector3, PI,
};

use rand::prelude::*;

use std::time::Instant;

const SOURCE_SIZE: Float = 10.0;

fn calc_p1(focus: Vector3, n_sqrt: usize) -> Float {
    let mut calculator = CpuCalculator::new();

    let mut transducers = Vec::new();
    for y in 0..n_sqrt {
        for x in 0..n_sqrt {
            let pos = [SOURCE_SIZE * x as Float, SOURCE_SIZE * y as Float, 0.];
            let phase = (norm(sub(pos, focus)) % WAVE_LENGTH) / WAVE_LENGTH;
            transducers.push(WaveSource::new(
                pos,
                Complex::new(0., 2.0 * PI * (1.0 - phase)).exp(),
            ));
        }
    }
    calculator.add_wave_sources(&transducers);
    let mut buffer = BufferBuilder::new()
        .x_at(focus[0])
        .y_at(focus[1])
        .z_at(focus[2])
        .resolution(1.)
        .generate::<Amplitude>();

    buffer.calculate(&calculator);
    buffer.buffer()[0] as f64
}

fn generate_test_set(
    center: Vector3,
    obs_range: Float,
    n_sqrt: usize,
    m: usize,
    iter: usize,
) -> (Vec<Vec<Vector3>>, Vec<Vec<Float>>) {
    let p1 = calc_p1(center, n_sqrt);

    let mut rng = rand::thread_rng();
    let mut foci_set = Vec::new();
    let mut amps_set = Vec::new();
    for _ in 0..iter {
        let mut foci = Vec::with_capacity(m);
        for _ in 0..m {
            foci.push(add(
                center,
                [
                    (rng.gen::<Float>() - 0.5) * obs_range,
                    (rng.gen::<Float>() - 0.5) * obs_range,
                    0.0,
                ],
            ));
        }
        let mut amps = Vec::with_capacity(foci.len());
        let amp = p1 / (m as f64).sqrt();
        for _ in 0..foci.len() {
            amps.push(amp);
        }
        foci_set.push(foci);
        amps_set.push(amps);
    }
    (foci_set, amps_set)
}

fn measure_time<T: Optimizer>(
    opt: T,
    name: &str,
    n_sqrt: usize,
    m: usize,
    foci_set: &[Vec<Vector3>],
    amps_set: &[Vec<Float>],
) {
    let mut opt = opt;

    let mut calculator = CpuCalculator::new();

    let mut transducers = Vec::new();
    for y in 0..n_sqrt {
        for x in 0..n_sqrt {
            let pos = [SOURCE_SIZE * x as Float, SOURCE_SIZE * y as Float, 0.];
            transducers.push(WaveSource::new(pos, Complex::new(0., 0.)));
        }
    }
    calculator.add_wave_sources(&transducers);

    let mut elasped = Vec::new();
    for (foci, amps) in foci_set.iter().zip(amps_set.iter()) {
        for source in calculator.wave_sources() {
            source.q = Complex::new(1., 0.);
        }
        opt.set_target_foci(foci);
        opt.set_target_amps(amps);

        let start = Instant::now();
        opt.optimize(calculator.wave_sources());
        elasped.push(start.elapsed().as_micros());
    }

    let mut wtr = csv::Writer::from_path(format!(
        "times_foci/{}_M{}_N{}.csv",
        name,
        m,
        n_sqrt * n_sqrt
    ))
    .unwrap();
    write_data(&mut wtr, &elasped);
    println!("\t{} done", name);
}

fn write_data<T: std::io::Write>(wtr: &mut csv::Writer<T>, data: &[u128]) {
    for &v in data {
        wtr.write_record(&[v.to_string()]).unwrap();
    }
}

fn main() {
    let n_sqrt = 16;
    let iter = 10;

    let m_max_pow = 10;

    let focus_z = 150.0;
    let center = [
        SOURCE_SIZE * (n_sqrt - 1) as Float / 2.0,
        SOURCE_SIZE * (n_sqrt - 1) as Float / 2.0,
        focus_z,
    ];

    std::fs::create_dir("times_foci").unwrap_or(());

    for i in 1..=m_max_pow {
        let m = 1 << i;

        println!("testing: M={}, N={}", m, n_sqrt * n_sqrt);

        let (foci_set, amps_set) = generate_test_set(center, 100.0, n_sqrt, m, iter);

        measure_time(
            GreedyBruteForce::new(16, 1, false),
            "gbf_16_1",
            n_sqrt,
            m,
            &foci_set,
            &amps_set,
        );

        measure_time(
            Horn::new(1000, 1e-3, 0.9),
            "horn",
            n_sqrt,
            m,
            &foci_set,
            &amps_set,
        );

        measure_time(Long::new(1.0), "long", n_sqrt, m, &foci_set, &amps_set);

        measure_time(
            LM::new(1e-8, 1e-8, 1e-3, 200),
            "lm",
            n_sqrt,
            m,
            &foci_set,
            &amps_set,
        );

        measure_time(GSPAT::new(100), "gspat", n_sqrt, m, &foci_set, &amps_set);
    }
}
