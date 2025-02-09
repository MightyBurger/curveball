// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::PI;

use glam::DVec3;
use thiserror::Error;

use super::FrenetFrame;

type PathResult<T> = Result<T, PathError>;

pub fn revolve(radius: f64) -> PathResult<(impl Fn(f64) -> DVec3, impl Fn(f64) -> FrenetFrame)> {
    let path_fn = move |mut a: f64| {
        a = a * PI / 180.0;
        DVec3 {
            x: radius * a.cos(),
            y: radius * a.sin(),
            z: 0.0,
        }
    };
    let frenet_fn = move |mut a: f64| {
        a = a * PI / 180.0;
        FrenetFrame {
            tangent: DVec3 {
                x: -a.sin(),
                y: a.cos(),
                z: 0.0,
            },
            normal: DVec3 {
                x: -a.cos(),
                y: -a.sin(),
                z: 0.0,
            },
            binormal: DVec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        }
    };
    Ok((path_fn, frenet_fn))
}

#[derive(Error, Debug)]
pub enum PathError {}
