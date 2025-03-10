// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::PI;

use glam::DVec3;
use thiserror::Error;

use super::FrenetFrame;

pub type PathResult<T> = Result<T, PathError>;

pub trait Path {
    fn point(&self, t: f64) -> DVec3;
    fn frame(&self, t: f64) -> FrenetFrame;
}

// Make Box<dyn Path> implement Path
impl Path for Box<dyn Path + '_> {
    fn point(&self, t: f64) -> DVec3 {
        (**self).point(t)
    }
    fn frame(&self, t: f64) -> FrenetFrame {
        (**self).frame(t)
    }
}

#[derive(Error, Debug)]
pub enum PathError {
    #[error("{0}")]
    SinusoidError(#[from] SinusoidError),
}

// Tip: the tangent vector in the frenet frame should always be the derivative of the path function
// with respect to the parameter, normalized.

#[derive(Debug, Clone)]
pub struct Line {
    x: f64,
    y: f64,
    z: f64,
}

impl Line {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Path for Line {
    fn point(&self, t: f64) -> DVec3 {
        DVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        } * t
    }
    fn frame(&self, _t: f64) -> FrenetFrame {
        let tangent = DVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
        .normalize_or_zero();
        let normal = DVec3 {
            x: -self.y,
            y: self.x,
            z: 0.0,
        }
        .normalize_or_zero();
        let binormal = tangent.cross(normal);
        FrenetFrame {
            tangent,
            normal,
            binormal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Revolve {
    radius: f64,
}

impl Revolve {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Path for Revolve {
    fn point(&self, mut t: f64) -> DVec3 {
        t = t * PI / 180.0;
        DVec3 {
            x: self.radius * t.cos(),
            y: self.radius * t.sin(),
            z: 0.0,
        }
    }
    fn frame(&self, mut t: f64) -> FrenetFrame {
        t = t * PI / 180.0;
        FrenetFrame {
            tangent: DVec3 {
                x: -t.sin(),
                y: t.cos(),
                z: 0.0,
            },
            normal: DVec3 {
                x: -t.cos(),
                y: -t.sin(),
                z: 0.0,
            },
            binormal: DVec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        }
    }
}

// Period is in units of space.
// Phase is also in units of space.

#[derive(Debug, Clone)]
pub struct Sinusoid {
    amplitude: f64,
    period: f64,
    phase: f64,
}

impl Sinusoid {
    pub fn new(amplitude: f64, period: f64, phase: f64) -> PathResult<Self> {
        if !(period > 0.0) {
            return Err(SinusoidError::SinusoidInfiniteFrequency(period))?;
        }
        Ok(Self {
            amplitude,
            period,
            phase,
        })
    }
}

impl Path for Sinusoid {
    fn point(&self, t: f64) -> DVec3 {
        let omega = 2.0 * PI / self.period;
        DVec3 {
            x: t,
            y: 0.0,
            z: self.amplitude * f64::sin(omega * (t + self.phase)),
        }
    }
    fn frame(&self, t: f64) -> FrenetFrame {
        let omega = 2.0 * PI / self.period;
        FrenetFrame {
            tangent: DVec3 {
                x: 1.0,
                y: 0.0,
                z: self.amplitude * f64::cos(omega * (t + self.phase)) * omega,
            }
            .normalize_or_zero(),
            normal: DVec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            binormal: DVec3 {
                x: -self.amplitude * f64::cos(omega * (t + self.phase)) * omega,
                y: 0.0,
                z: 1.0,
            }
            .normalize_or_zero(),
        }
    }
}

#[derive(Error, Debug)]
pub enum SinusoidError {
    #[error("Period of {0} is invalid; must be positive")]
    SinusoidInfiniteFrequency(f64),
}
