// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The [Path] trait and a number of structs implementing that trait.

use std::f64::consts::PI;

use glam::{DVec2, DVec3};
use lerp::Lerp;
use thiserror::Error;

use super::FrenetFrame;
use itertools::Itertools;

pub type PathResult<T> = Result<T, PathError>;

/// A trait representing a path in 3D space.
pub trait Path {
    /// Given a parameter `t` that varies between `0.0` and `1.0`, produce a point in 3D space.
    fn point(&self, t: f64) -> DVec3;

    /// Given a parameter `t` that varies between `0.0` and `1.0`, produce a [FrenetFrame]. See
    /// [FrenetFrame] for more information.
    ///
    /// Implementing this function is required, though if you do not plan to use
    /// `ProfileOrientation::FollowPath`, you may have this function return placeholder vectors
    /// like [DVec3::default].
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
    #[error("{0}")]
    BezierError(#[from] BezierError),
    #[error("{0}")]
    CatenaryError(#[from] CatenaryError),
    #[error("{0}")]
    SerpentineError(#[from] SerpentineError),
}

// Tip: the tangent vector in the frenet frame should always be the derivative of the path function
// with respect to the parameter, normalized.

// ==================== Line ====================

/// A line from the origin to another point in 3D space.
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

// ==================== Revolve ====================

/// A circular path around the vertical axis at the origin.
#[derive(Debug, Clone)]
pub struct Revolve {
    radius: f64,
    start_angle_rad: f64,
    end_angle_rad: f64,
}

impl Revolve {
    pub fn new(radius: f64, start_angle: f64, end_angle: f64) -> Self {
        Self {
            radius,
            start_angle_rad: start_angle * PI / 180.0,
            end_angle_rad: end_angle * PI / 180.0,
        }
    }
}

impl Path for Revolve {
    fn point(&self, t: f64) -> DVec3 {
        let theta = self.start_angle_rad.lerp(self.end_angle_rad, t);
        DVec3 {
            x: self.radius * theta.cos(),
            y: self.radius * theta.sin(),
            z: 0.0,
        }
    }
    fn frame(&self, t: f64) -> FrenetFrame {
        let theta = self.start_angle_rad.lerp(self.end_angle_rad, t);
        FrenetFrame {
            tangent: DVec3 {
                x: -theta.sin(),
                y: theta.cos(),
                z: 0.0,
            },
            normal: DVec3 {
                x: -theta.cos(),
                y: -theta.sin(),
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

// ==================== Sinusoid ====================

/// A sinusoidal path with a particular phase, amplitude, and period.
#[derive(Debug, Clone)]
pub struct Sinusoid {
    amplitude: f64,
    period: f64,
    phase: f64,
    start: f64,
    end: f64,
}

impl Sinusoid {
    pub fn new(amplitude: f64, period: f64, phase: f64, start: f64, end: f64) -> PathResult<Self> {
        if !(period > 0.0) {
            return Err(SinusoidError::SinusoidInfiniteFrequency(period))?;
        }
        Ok(Self {
            amplitude,
            period,
            phase,
            start,
            end,
        })
    }
}

impl Path for Sinusoid {
    fn point(&self, t: f64) -> DVec3 {
        let x = self.start.lerp(self.end, t);
        let omega = 2.0 * PI / self.period;
        DVec3 {
            x,
            y: 0.0,
            z: self.amplitude * f64::sin(omega * (x + self.phase)),
        }
    }
    fn frame(&self, t: f64) -> FrenetFrame {
        let x = self.start.lerp(self.end, t);
        let omega = 2.0 * PI / self.period;
        FrenetFrame {
            tangent: DVec3 {
                x: 1.0,
                y: 0.0,
                z: self.amplitude * f64::cos(omega * (x + self.phase)) * omega,
            }
            .normalize_or_zero(),
            normal: DVec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            binormal: DVec3 {
                x: -self.amplitude * f64::cos(omega * (x + self.phase)) * omega,
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

// ==================== Bezier ====================

/// A [Bezier curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) in 3D space, defined by a
/// vector of control points.
#[derive(Debug, Clone)]
pub struct Bezier {
    points: Vec<DVec2>,
}

impl Bezier {
    pub fn new(points: Vec<DVec2>) -> Result<Self, BezierError> {
        if points.len() < 2 {
            return Err(BezierError::NotEnoughPoints(points.len()));
        }
        Ok(Self { points })
    }
}

impl Path for Bezier {
    fn point(&self, t: f64) -> DVec3 {
        let point2d = bezier(&self.points, t);
        DVec3 {
            x: point2d.x,
            y: 0.0,
            z: point2d.y,
        }
    }
    fn frame(&self, t: f64) -> FrenetFrame {
        let point2d = bezier_derivative(&self.points, t);
        let tangent = DVec3 {
            x: point2d.x,
            y: 0.0,
            z: point2d.y,
        }
        .normalize_or_zero();
        let normal = DVec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let binormal = tangent.cross(normal);
        FrenetFrame {
            tangent,
            normal,
            binormal,
        }
    }
}

#[derive(Error, Debug)]
pub enum BezierError {
    #[error("Bezier curve requires at least two points; {0} provided")]
    NotEnoughPoints(usize),
}

fn bezier(points: &Vec<DVec2>, t: f64) -> DVec2 {
    let result = recursive_bezier(points, t);
    result[0]
}

fn recursive_bezier(points: &Vec<DVec2>, t: f64) -> Vec<DVec2> {
    if points.len() == 1 {
        vec![points[0]]
    } else {
        recursive_bezier(
            &points
                .into_iter()
                .tuple_windows()
                .map(|(point1, point2)| point1.lerp(*point2, t))
                .collect(),
            t,
        )
    }
}

fn bezier_derivative(points: &Vec<DVec2>, t: f64) -> DVec2 {
    let n = points.len() as f64;
    let intersparsed_points = points
        .into_iter()
        .tuple_windows()
        .map(|(point1, point2)| n * (point2 - point1))
        .collect();
    bezier(&intersparsed_points, t)
}

// ==================== Catenary ====================

/// A [Catenary](https://en.wikipedia.org/wiki/Catenary) curve; the shape a cable takes on when
/// hung between two points.
#[derive(Debug, Clone)]
pub struct Catenary {
    a: f64,
    k: f64,
    c: f64,
    span: f64,
}

impl Catenary {
    pub fn new(span: f64, height: f64, s: f64) -> Result<Self, CatenaryError> {
        // get delta values
        let v = height;
        let h = span;

        let initial_guess = 1.0
            / f64::sqrt(f64::sqrt(f64::powi(s, 2) - f64::powi(v, 2)) / h - 1.0)
            / (2.0 * f64::sqrt(6.0));

        let min_s = f64::sqrt(f64::powi(height, 2) + f64::powi(span, 2));
        if s <= min_s {
            return Err(CatenaryError::LengthTooShort {
                given: s,
                min: min_s,
            })?;
        }

        let a = newton_a(v, h, s, 0.0, 0.0, span, height, initial_guess)?;

        // Find the other catenary parameters. Thankfully these aren't as bad.
        let k: f64 = 0.5 * (h - a * f64::ln((s + v) / (s - v)));
        let c: f64 = -a * f64::cosh((-k) / a);

        Ok(Self { a, k, c, span })
    }
}

impl Path for Catenary {
    fn point(&self, t: f64) -> DVec3 {
        let x = 0.0.lerp(self.span, t);
        DVec3 {
            x,
            y: 0.0,
            z: catenary(x, self.a, self.k, self.c),
        }
    }
    fn frame(&self, t: f64) -> FrenetFrame {
        let x = 0.0.lerp(self.span, t);
        let tangent = DVec3 {
            x: 1.0,
            y: 0.0,
            z: f64::sinh((x - self.k) / self.a), // Derivative of catenary
        }
        .normalize_or_zero();
        let normal = DVec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let binormal = tangent.cross(normal);
        FrenetFrame {
            tangent,
            normal,
            binormal,
        }
    }
}

fn catenary(x: f64, a: f64, k: f64, c: f64) -> f64 {
    a * f64::cosh((x - k) / a) + c
}

// Newton's method. Has the potential to fail to find a solution.
#[allow(clippy::too_many_arguments)]
fn newton_a(
    v: f64,
    h: f64,
    s: f64,
    x0: f64,
    z0: f64,
    x1: f64,
    z1: f64,
    initial_guess: f64,
) -> Result<f64, CatenaryError> {
    let iteration_limit = 10_000;

    // Limit for how inaccurate our points can be. We only need accuracy to six decimal places,
    // so this should be just fine.

    let epsilon: f64 = f64::powi(10.0, -9);

    // Initial guess
    let mut b_i: f64 = initial_guess;
    let mut b_ip1: f64 = b_i;
    let mut icount: i32 = 0;
    while catenary_bounds_err(x0, z0, x1, z1, b_ip1 * h, s) > epsilon && icount < iteration_limit {
        b_i = b_ip1;
        b_ip1 = b_i - f2b(b_i, v, h, s) / df2b(b_i);
        icount += 1;
    }
    // a = bh
    if icount >= iteration_limit
        || !f64::is_finite(catenary_bounds_err(x0, z0, x1, z1, b_ip1 * h, s))
    {
        Err(CatenaryError::NewtonFail {
            iterations: iteration_limit,
            initial: initial_guess,
        })
    } else {
        Ok(b_ip1 * h)
    }
}

fn f2b(b: f64, v: f64, h: f64, s: f64) -> f64 {
    1.0 / f64::sqrt(2.0 * b * f64::sinh(1.0 / (2.0 * b)) - 1.0)
        - 1.0 / f64::sqrt(f64::sqrt(f64::powi(s, 2) - f64::powi(v, 2)) / h - 1.0)
}

fn df2b(b: f64) -> f64 {
    let m = 1.0 / (2.0 * b);
    (m * f64::cosh(m) - f64::sinh(m)) * f64::powf(1.0 / m * f64::sinh(m) - 1.0, -1.5)
}

fn catenary_bounds_err(x0: f64, z0: f64, x1: f64, z1: f64, a: f64, s: f64) -> f64 {
    let v = z1 - z0;
    let h = x1 - x0;
    let k: f64 = x0 + 0.5 * (h - a * f64::ln((s + v) / (s - v)));
    let c: f64 = z0 - a * f64::cosh((x0 - k) / a);
    let error0 = f64::abs(catenary(x0, a, k, c) - z0);
    let error1 = f64::abs(catenary(x1, a, k, c) - z1);
    f64::max(error0, error1)
}

#[derive(Error, Debug)]
pub enum CatenaryError {
    #[error("Given length {given} is too short; must be greater than {min}.")]
    LengthTooShort { given: f64, min: f64 },
    #[error(
        "Newton's method failed to converge to an accurate solution after {iterations} iterations. The initial guess was {initial}. Change the parameters to a less extreme catenary curve, or try again with a different initial guess."
    )]
    NewtonFail { iterations: i32, initial: f64 },
}

// ==================== Serpentine ====================

/// A curve consisting of two tangent circles.
///
/// Named after a [serpentine belt](https://en.wikipedia.org/wiki/Serpentine_belt) in a car.
#[derive(Debug, Clone)]
pub struct Serpentine {
    x: f64,
    z: f64,
}

impl Serpentine {
    pub fn new(x: f64, z: f64) -> Result<Self, SerpentineError> {
        if z <= 0.0 {
            return Err(SerpentineError::OrderedHeight);
        }
        if z > x {
            return Err(SerpentineError::TooTall);
        }

        Ok(Self { x, z })
    }
}

impl Path for Serpentine {
    // Expected for t to vary between 0 and 1
    fn point(&self, t: f64) -> DVec3 {
        let xm = self.x / 2.0;
        let zm = self.z / 2.0;

        let zd = self.z - zm;
        let xd = self.x - xm;

        let r0 = ((zm * zm) + (xm * xm)) / (2.0 * zm);
        let r1 = ((zd * zd) + (xd * xd)) / (2.0 * zd);

        let theta0_start = -PI / 2.0;
        let theta0_end = f64::asin(xm / r0) - PI / 2.0;

        let theta1_start = PI / 2.0 + f64::asin(xd / r1);
        let theta1_end = PI / 2.0;

        if t < 0.5 {
            let theta = theta0_start.lerp(theta0_end, t * 2.0);
            DVec3 {
                x: r0 * f64::cos(theta),
                y: 0.0,
                z: r0 * f64::sin(theta) + r0,
            }
        } else {
            let theta = theta1_start.lerp(theta1_end, (t - 0.5) * 2.0);
            DVec3 {
                x: r1 * f64::cos(theta) + self.x,
                y: 0.0,
                z: r1 * f64::sin(theta) - r0 + self.z,
            }
        }
    }
    fn frame(&self, t: f64) -> FrenetFrame {
        let xm = self.x / 2.0;
        let zm = self.z / 2.0;

        let zd = self.z - zm;
        let xd = self.x - xm;

        let r0 = ((zm * zm) + (xm * xm)) / (2.0 * zm);
        let r1 = ((zd * zd) + (xd * xd)) / (2.0 * zd);

        let theta0_start = -PI / 2.0;
        let theta0_end = f64::asin(xm / r0) - PI / 2.0;

        let theta1_start = PI / 2.0 + f64::asin(xd / r1);
        let theta1_end = PI / 2.0;

        let tangent = if t < 0.5 {
            let theta = theta0_start.lerp(theta0_end, t * 2.0);
            DVec3 {
                x: -f64::sin(theta),
                y: 0.0,
                z: f64::cos(theta),
            }
        } else {
            let theta = theta1_start.lerp(theta1_end, (t - 0.5) * 2.0);
            DVec3 {
                x: f64::sin(theta),
                y: 0.0,
                z: -f64::cos(theta),
            }
        };
        let normal = DVec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let binormal = tangent.cross(normal);
        FrenetFrame {
            tangent,
            normal,
            binormal,
        }
    }
}

#[derive(Error, Debug)]
pub enum SerpentineError {
    #[error("Ending height must be greater than the starting height.")]
    OrderedHeight,
    #[error("Serpentine curve height cannot be greater than its length.")]
    TooTall,
}
