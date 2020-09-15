/*
 * File: relative_error.rs
 * Project: examples
 * Created Date: 27/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use ghr::buffer::{generator::*, BufferBuilder, ComplexFieldBufferScatter, FieldBuffer};
use ghr::calculator::{Calculate, Calculator, CpuCalculator};
use ghr::optimizer::*;
use ghr::vec_utils::*;
use ghr::wave_source::WaveSource;

use ndarray_linalg::*;
use rand::prelude::*;

type Complex = c32;

use std::f32::consts::PI;
const SOURCE_SIZE: f32 = 10.0;
const WAVE_LENGTH: f32 = 8.5;

const N_SQRT: usize = 20;

macro_rules! calc_p1 {
    ($f: ident) => {{
        let mut calculator = CpuCalculator::new();
        calculator.set_wave_number(2.0 * PI / WAVE_LENGTH);

        let mut transducers = Vec::new();
        for y in 0..N_SQRT {
            for x in 0..N_SQRT {
                let pos = [SOURCE_SIZE * x as f32, SOURCE_SIZE * y as f32, 0.];
                let phase = (norm(sub(pos, $f)) % WAVE_LENGTH) / WAVE_LENGTH;
                transducers.push(WaveSource::new(pos, 1.0, 2.0 * PI * phase));
            }
        }
        calculator.add_wave_sources(&transducers);
        let mut buffer = BufferBuilder::new()
            .x_at($f[0])
            .y_at($f[1])
            .z_at($f[2])
            .resolution(1.)
            .generate::<Amplitude>();

        buffer.calculate(&calculator);
        buffer.buffer()[0] as f64
    }};
}

macro_rules! calc_relative_error {
    ($opt: ty, $target_pos: tt, $amps: tt, $calculator: expr, $m: tt) => {{
        let amp = $amps[0];
        let mut buffer = ComplexFieldBufferScatter::new();
        for &p in $target_pos.iter() {
            buffer.add_observe_point(p, Complex::new(0., 0.));
        }
        let optimizer = <$opt>::new($target_pos.clone(), $amps.clone(), WAVE_LENGTH as f64);
        optimizer.optimize($calculator.wave_sources(), true, true);
        buffer.calculate(&$calculator);
        let demoni = amp * $m as f64;
        let mut numerator = 0.0;
        let mut max_v = f64::NEG_INFINITY;
        for b in buffer.buffer() {
            max_v = max_v.max(b.abs() as f64);
        }
        let mut mean_v = 0.0;
        for b in buffer.buffer() {
            numerator += (b.abs() as f64 - amp).abs();
            let norm_v = b.abs() as f64 / max_v;
            mean_v += norm_v;
        }
        mean_v /= buffer.buffer().len() as f64;

        let mut var = 0.0;
        for b in buffer.buffer() {
            let norm_v = b.abs() as f64 / max_v;
            var += (norm_v - mean_v) * (norm_v - mean_v);
        }
        var /= buffer.buffer().len() as f64;
        (numerator / demoni * 100.0, var)
    }};
}

macro_rules! relative_error {
    ($m: tt, $iter: tt) => {{
        let focus_z = 150.0;
        let center = [
            SOURCE_SIZE * (N_SQRT - 1) as f32 / 2.0,
            SOURCE_SIZE * (N_SQRT - 1) as f32 / 2.0,
            focus_z,
        ];
        let obs_range = 100.0;

        let mut calculator = CpuCalculator::new();
        calculator.set_wave_number(2.0 * PI / WAVE_LENGTH);

        let mut transducers = Vec::new();
        for y in 0..N_SQRT {
            for x in 0..N_SQRT {
                let pos = [SOURCE_SIZE * x as f32, SOURCE_SIZE * y as f32, 0.];
                transducers.push(WaveSource::new(pos, 0.0, 0.0));
            }
        }
        calculator.add_wave_sources(&transducers);

        let p1 = calc_p1!(center);

        let mut rng = rand::thread_rng();
        let mut gbf_es = Vec::with_capacity($iter);
        let mut horn_es = Vec::with_capacity($iter);
        let mut long_es = Vec::with_capacity($iter);
        let mut lm_es = Vec::with_capacity($iter);

        let mut gbf_vars = Vec::with_capacity($iter);
        let mut horn_vars = Vec::with_capacity($iter);
        let mut long_vars = Vec::with_capacity($iter);
        let mut lm_vars = Vec::with_capacity($iter);

        for _ in 0..$iter {
            let mut target_pos = Vec::with_capacity($m);
            for _ in 0..$m {
                target_pos.push(add(
                    center,
                    [
                        (rng.gen::<f32>() - 0.5) * obs_range,
                        (rng.gen::<f32>() - 0.5) * obs_range,
                        0.0,
                    ],
                ));
            }
            let mut amps = Vec::with_capacity(target_pos.len());
            let amp = p1 / ($m as f64).sqrt();
            for _ in 0..target_pos.len() {
                amps.push(amp);
            }

            let (bgf_e, bgf_var) =
                calc_relative_error!(GreedyBruteForce, target_pos, amps, calculator, $m);
            let (horn_e, horn_var) = calc_relative_error!(Horn, target_pos, amps, calculator, $m);
            let (long_e, long_var) = calc_relative_error!(Long, target_pos, amps, calculator, $m);
            let (lm_e, lm_var) = calc_relative_error!(LM, target_pos, amps, calculator, $m);
            gbf_es.push(bgf_e);
            horn_es.push(horn_e);
            long_es.push(long_e);
            lm_es.push(lm_e);

            gbf_vars.push(bgf_var);
            horn_vars.push(horn_var);
            long_vars.push(long_var);
            lm_vars.push(lm_var);
        }
        (
            vec![gbf_es, horn_es, long_es, lm_es],
            vec![gbf_vars, horn_vars, long_vars, lm_vars],
        )
    }};
}

fn get_mean(vec: &[f64]) -> f64 {
    let n = vec.len();
    let mut tmp = 0.0;
    for v in vec {
        tmp += v;
    }
    tmp / n as f64
}

fn get_max(vec: &[f64]) -> f64 {
    let mut tmp = f64::NEG_INFINITY;
    for &v in vec {
        tmp = tmp.max(v);
    }
    tmp
}

fn get_min(vec: &[f64]) -> f64 {
    let mut tmp = f64::INFINITY;
    for &v in vec {
        tmp = tmp.min(v);
    }
    tmp
}

fn write_header<T: std::io::Write>(wtr: &mut csv::Writer<T>, header: &[&str]) {
    let mut vh = Vec::with_capacity(1 + header.len() * 3);
    vh.push("M".to_owned());
    for &h in header {
        vh.push(h.to_owned() + "_mean");
        vh.push(h.to_owned() + "_max");
        vh.push(h.to_owned() + "_min");
    }
    wtr.write_record(&vh).unwrap();
}

fn format_data<T: std::io::Write>(wtr: &mut csv::Writer<T>, m: usize, vec: &Vec<Vec<f64>>) {
    let mut vh = Vec::with_capacity(1 + vec.len() * 3);
    vh.push(m.to_string());
    for v in vec {
        vh.push(get_mean(v).to_string());
        vh.push(get_max(v).to_string());
        vh.push(get_min(v).to_string());
    }
    wtr.write_record(&vh).unwrap();
}

fn main() {
    let mut wtr_error = csv::Writer::from_path("relative_error.csv").unwrap();
    let mut wtr_var = csv::Writer::from_path("var.csv").unwrap();
    let header = ["GBF16", "HORN", "LONG", "LM"];
    write_header(&mut wtr_error, &header);
    write_header(&mut wtr_var, &header);

    use std::time::Instant;
    for m in (1..=2).map(|i| i * 2) {
        let start = Instant::now();
        println!("M: {}...", m);
        let (es_vec, vars_vec) = relative_error!(m, 100);
        let end = start.elapsed();
        println!(
            "  calc: {}.{:03}",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
        format_data(&mut wtr_error, m, &es_vec);
        format_data(&mut wtr_var, m, &vars_vec);

        let end = start.elapsed();
        println!(
            "  write: {}.{:03}",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
    }
}
