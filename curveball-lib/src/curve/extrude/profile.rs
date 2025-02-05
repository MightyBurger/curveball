// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::PI;

use glam::DVec2;
use lerp::LerpIter;

pub fn circle(n: u32, radius: f64) -> Vec<DVec2> {
    0f64.lerp_iter_closed(2.0 * PI, n as usize)
        .map(|theta| DVec2 {
            x: radius * theta.cos(),
            y: radius * theta.sin(),
        })
        .collect()
}
