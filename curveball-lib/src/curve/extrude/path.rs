// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::PI;

use glam::DVec3;
use thiserror::Error;

use super::FrenetFrame;

type PathResult<T> = Result<T, PathError>;

// Tip: the tangent vector in the frenet frame should always be the derivative of the path function
// with respect to the parameter, normalized.

pub fn line(
    x: f64,
    y: f64,
    z: f64,
) -> PathResult<(impl Fn(f64) -> DVec3, impl Fn(f64) -> FrenetFrame)> {
    let path_fn = move |a: f64| DVec3 { x, y, z } * a;
    let frenet_fn = move |_a: f64| {
        let tangent = DVec3 { x, y, z }.normalize_or_zero();
        let normal = DVec3 {
            x: -y,
            y: x,
            z: 0.0,
        }
        .normalize_or_zero();
        let binormal = tangent.cross(normal);
        FrenetFrame {
            tangent,
            normal,
            binormal,
        }
    };
    Ok((path_fn, frenet_fn))
}

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

// Period is in units of space.
// Phase is also in units of space.
pub fn sinusoid(
    amplitude: f64,
    period: f64,
    phase: f64,
) -> PathResult<(impl Fn(f64) -> DVec3, impl Fn(f64) -> FrenetFrame)> {
    if !(period > 0.0) {
        return Err(SinusoidError::SinusoidInfiniteFrequency(period))?;
    }
    let omega = 2.0 * PI / period;
    let path_fn = move |a: f64| DVec3 {
        x: a,
        y: 0.0,
        z: amplitude * f64::sin(omega * (a + phase)),
    };
    let frenet_fn = move |a: f64| FrenetFrame {
        tangent: DVec3 {
            x: 1.0,
            y: 0.0,
            z: amplitude * f64::cos(omega * (a + phase)) * omega,
        }
        .normalize_or_zero(),
        normal: DVec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        binormal: DVec3 {
            x: -amplitude * f64::cos(omega * (a + phase)) * omega,
            y: 0.0,
            z: 1.0,
        }
        .normalize_or_zero(),
    };
    Ok((path_fn, frenet_fn))
}

#[derive(Error, Debug)]
pub enum SinusoidError {
    #[error("Period of {0} is invalid; must be positive")]
    SinusoidInfiniteFrequency(f64),
}

#[derive(Error, Debug)]
pub enum PathError {
    #[error("{0}")]
    SinusoidError(#[from] SinusoidError),
}
