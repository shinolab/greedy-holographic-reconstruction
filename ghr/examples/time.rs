/*
 * File: time.rs
 * Project: examples
 * Created Date: 09/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use ghr::{
    calculator::{Calculator, CpuCalculator},
    optimizer::*,
    vec_utils::*,
    wave_source::WaveSource,
    Float, PI,
};

use rand::prelude::*;

use std::time::Instant;

const SOURCE_SIZE: Float = 10.0;
const WAVE_LENGTH: Float = 8.5;

macro_rules! iterate {
    ($x: block, $iter: tt) => {{
        let start = Instant::now();
        for _ in 0..$iter {
            $x;
        }
        start.elapsed().as_millis() / $iter
    }};
}

macro_rules! measure {
    ($opt: ty, $m: tt, $n_sqrt: tt) => {{
        let focus_z = 150.0;
        let focal_pos = [
            SOURCE_SIZE * ($n_sqrt - 1) as Float / 2.0,
            SOURCE_SIZE * ($n_sqrt - 1) as Float / 2.0,
            focus_z,
        ];
        let obs_range = 100.0;

        let mut calculator = CpuCalculator::new();
        calculator.set_wave_number(2.0 * PI / WAVE_LENGTH);

        let mut transducers = Vec::new();
        for y in 0..$n_sqrt {
            for x in 0..$n_sqrt {
                let pos = [SOURCE_SIZE * x as Float, SOURCE_SIZE * y as Float, 0.];
                transducers.push(WaveSource::new(pos, 0.0, 0.0));
            }
        }
        calculator.add_wave_sources(&transducers);

        let mut rng = rand::thread_rng();
        let millis = iterate!(
            {
                for source in calculator.wave_sources() {
                    source.amp = 0.0;
                    source.phase = 0.0;
                }
                let mut target_pos = Vec::with_capacity($m);
                for _ in 0..$m {
                    target_pos.push(add(
                        focal_pos,
                        [
                            (rng.gen::<Float>() - 0.5) * obs_range,
                            (rng.gen::<Float>() - 0.5) * obs_range,
                            0.0,
                        ],
                    ));
                }
                let mut amps = Vec::with_capacity(target_pos.len());
                for _ in 0..target_pos.len() {
                    amps.push(1.0);
                }
                let optimizer = <$opt>::new(target_pos.clone(), amps.clone(), WAVE_LENGTH as f64);
                optimizer.optimize(calculator.wave_sources(), false);
            },
            100
        );
        millis
    }};
}

fn main() {
    let n_sqrt = 10;
    let m_max_pow = 10;
    let n_max_pow = 16;

    // // GHRBF PHASE
    // {
    //     println!("Greedy Holographic Reconstruction with phase");
    //     let file_path = "ghr_p.csv";
    //     let mut wtr = csv::Writer::from_path(file_path).unwrap();
    //     wtr.write_record(&["N", "M", "time[ms]"]).unwrap();
    //     for i in 1..=m_max_pow {
    //         let m = 1 << i;
    //         println!("{}", m);
    //         let millis = measure!(GreedyBruteForce, m, n_sqrt);
    //         wtr.write_record(&[
    //             (n_sqrt * n_sqrt).to_string(),
    //             (m).to_string(),
    //             millis.to_string(),
    //         ])
    //         .unwrap();
    //     }
    // }

    // // Long
    // println!("Long te al, 2014");
    // let file_path = "long.csv";
    // let mut wtr = csv::Writer::from_path(file_path).unwrap();
    // wtr.write_record(&["N", "M", "time[ms]"]).unwrap();
    // for i in 1..=m_max_pow {
    //     let m = 1 << i;
    //     println!("{}", m);
    //     let millis = measure!(Long, m, n_sqrt);
    //     wtr.write_record(&[
    //         (n_sqrt * n_sqrt).to_string(),
    //         (1 << i).to_string(),
    //         millis.to_string(),
    //     ])
    //     .unwrap();
    // }

    // // HORN
    // println!("HORN");
    // let file_path = "horn.csv";
    // let mut wtr = csv::Writer::from_path(file_path).unwrap();
    // wtr.write_record(&["N", "M", "time[ms]"]).unwrap();
    // for i in 1..=m_max_pow {
    //     let m = 1 << i;
    //     println!("{}", m);
    //     let millis = measure!(Horn, m, n_sqrt);
    //     wtr.write_record(&[
    //         (n_sqrt * n_sqrt).to_string(),
    //         (1 << i).to_string(),
    //         millis.to_string(),
    //     ])
    //     .unwrap();
    // }

    // LM
    println!("Levenberg-Marquardt");
    let file_path = "gbs_M.csv";
    let mut wtr = csv::Writer::from_path(file_path).unwrap();
    wtr.write_record(&["N", "M", "time[ms]"]).unwrap();
    for i in 1..=m_max_pow {
        let m = 1 << i;
        println!("m: {}", m);
        let millis = measure!(GreedyBruteForce, m, n_sqrt);
        wtr.write_record(&[
            (n_sqrt * n_sqrt).to_string(),
            (1 << i).to_string(),
            millis.to_string(),
        ])
        .unwrap();
    }

    // LM
    println!("Levenberg-Marquardt");
    let file_path = "gbs_N.csv";
    let mut wtr = csv::Writer::from_path(file_path).unwrap();
    wtr.write_record(&["N", "M", "time[ms]"]).unwrap();
    for i in 1..=n_max_pow {
        let n = 2 * i;
        let m = 128;
        println!("n: {}", n * n);
        let millis = measure!(GreedyBruteForce, 128, n);
        wtr.write_record(&[(n * n).to_string(), m.to_string(), millis.to_string()])
            .unwrap();
    }
}
