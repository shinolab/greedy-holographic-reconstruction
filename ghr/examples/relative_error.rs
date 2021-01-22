/*
 * File: relative_error.rs
 * Project: examples
 * Created Date: 27/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use ghr::{
    buffer::{generator::*, BufferBuilder, ComplexFieldBufferScatter, FieldBuffer},
    calculator::{Calculate, Calculator, CpuCalculator},
    consts::WAVE_LENGTH,
    math_utils::*,
    optimizer::*,
    wave_source::WaveSource,
    Complex, Float, Vector3, PI,
};

use ndarray_linalg::*;
use rand::prelude::*;

const SOURCE_SIZE: Float = 10.0;

const N_SQRT: usize = 20;

fn calc_p1(focus: Vector3) -> Float {
    let mut calculator = CpuCalculator::new();

    let mut transducers = Vec::new();
    for y in 0..N_SQRT {
        for x in 0..N_SQRT {
            let pos = [SOURCE_SIZE * x as Float, SOURCE_SIZE * y as Float, 0.];
            let phase = (norm(sub(pos, focus)) % WAVE_LENGTH) / WAVE_LENGTH;
            transducers.push(WaveSource::new(
                pos,
                1.0,
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
    buffer.buffer()[0]
}

fn set_up() -> CpuCalculator {
    let mut calculator = CpuCalculator::new();

    let mut transducers = Vec::new();
    for y in 0..N_SQRT {
        for x in 0..N_SQRT {
            let pos = [SOURCE_SIZE * x as Float, SOURCE_SIZE * y as Float, 0.];
            transducers.push(WaveSource::new(pos, 0.0, Complex::new(0., 0.)));
        }
    }
    calculator.add_wave_sources(&transducers);
    calculator
}

fn calc_relative_error<T: Optimizer>(
    optimizer: &mut T,
    calculator: &mut CpuCalculator,
    foci: &[Vector3],
    amps: &[Float],
) -> (Float, Float) {
    let mut buffer = ComplexFieldBufferScatter::new();
    for &p in foci.iter() {
        buffer.add_observe_point(p, Complex::new(0., 0.));
    }
    optimizer.set_target_foci(&foci);
    optimizer.set_target_amps(&amps);
    optimizer.optimize(calculator.wave_sources());
    buffer.calculate(calculator);

    let mut max_v = Float::NEG_INFINITY;
    for b in buffer.buffer() {
        max_v = max_v.max(b.abs());
    }

    let demoni: Float = amps.iter().sum();
    let mut numerator = 0.0;
    let mut mean_v = 0.0;
    for (b, amp) in buffer.buffer().iter().zip(amps.iter()) {
        numerator += (b.abs() - amp).abs();
        let norm_v = b.abs() / max_v;
        mean_v += norm_v;
    }
    mean_v /= buffer.buffer().len() as Float;

    let mut var = 0.0;
    for b in buffer.buffer() {
        let norm_v = b.abs() / max_v;
        var += (norm_v - mean_v) * (norm_v - mean_v);
    }
    var /= buffer.buffer().len() as Float;
    (numerator / demoni * 100.0, var.sqrt())
}

fn relative_errors<T: Optimizer>(
    optimizer: &mut T,
    calculator: &mut CpuCalculator,
    target_foci_set: &[Vec<Vector3>],
    target_amps_set: &[Vec<Float>],
) -> Vec<(Float, Float)> {
    target_foci_set
        .iter()
        .zip(target_amps_set.iter())
        .map(|(target_foci, target_amps)| {
            calc_relative_error(optimizer, calculator, target_foci, target_amps)
        })
        .collect()
}

fn write_data<T: std::io::Write>(wtr: &mut csv::Writer<T>, data: &[(Float, Float)]) {
    for v in data {
        wtr.write_record(&[v.0.to_string(), v.1.to_string()])
            .unwrap();
    }
}

fn test<T: Optimizer>(
    opt: T,
    name: &str,
    m: usize,
    calculator: &mut CpuCalculator,
    foci_set: &[Vec<Vector3>],
    amps_set: &[Vec<Float>],
) {
    let mut opt = opt;
    let errors = relative_errors(&mut opt, calculator, &foci_set, &amps_set);
    let mut wtr = csv::Writer::from_path(format!("relative_errors/{}_M{}.csv", name, m)).unwrap();
    write_data(&mut wtr, &errors);
    println!("\t{} done", name);
}

fn generate_test_set(
    center: Vector3,
    obs_range: Float,
    m: usize,
    iter: usize,
) -> (Vec<Vec<Vector3>>, Vec<Vec<Float>>) {
    let p1 = calc_p1(center);

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

fn main() {
    let iter = 10;

    let focus_z = 150.0;
    let center = [
        SOURCE_SIZE * (N_SQRT - 1) as Float / 2.0,
        SOURCE_SIZE * (N_SQRT - 1) as Float / 2.0,
        focus_z,
    ];

    let mut calculator = set_up();

    std::fs::create_dir("relative_errors").unwrap_or(());

    let test_foci_nums: Vec<usize> = vec![2, 4, 8, 16, 32];

    for m in test_foci_nums {
        println!("testing {}", m);

        let (foci_set, amps_set) = generate_test_set(center, 100.0, m, iter);

        test(
            GreedyBruteForce::new(16, 16, false),
            "gbf_16_16",
            m,
            &mut calculator,
            &foci_set,
            &amps_set,
        );

        test(
            Horn::new(1000, 1e-3, 0.9),
            "horn",
            m,
            &mut calculator,
            &foci_set,
            &amps_set,
        );

        test(
            Long::new(1.0),
            "long",
            m,
            &mut calculator,
            &foci_set,
            &amps_set,
        );

        test(
            LM::new(1e-8, 1e-8, 1e-3, 200),
            "lm",
            m,
            &mut calculator,
            &foci_set,
            &amps_set,
        );

        test(
            GSPAT::new(100),
            "gspat",
            m,
            &mut calculator,
            &foci_set,
            &amps_set,
        );
    }
}
