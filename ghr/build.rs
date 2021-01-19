/*
 * File: build.rs
 * Project: ghr
 * Created Date: 01/01/1970
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/01/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rustc-link-search=/usr/local/lib/");
    println!("cargo:rustc-link-search=/opt/openblas/lib");
}
