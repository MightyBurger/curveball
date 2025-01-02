// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::curve::{Curve, CurveResult, MAX_HULL_ITER};
use crate::map::Brush;
use glam::DVec3;
use thiserror::Error;

#[derive(Debug, Default, Clone)]
pub struct Catenary {
    pub n: u32,
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
    pub s: f64,
    pub w: f64,
    pub t: f64,
    pub initial_guess: Option<f64>,
}

impl Curve for Catenary {
    fn bake(&self) -> CurveResult<Vec<Brush>> {
        if self.n < 1 {
            return Err(CatenaryError::NotEnoughSegments { n: self.n })?;
        }
        if self.n > 4096 {
            return Err(CatenaryError::TooManySegments { n: self.n })?;
        }

        // get delta values
        let v = self.z1 - self.z0;
        let h = self.x1 - self.x0;

        let initial_guess = match self.initial_guess {
            Some(guess) => guess,
            None => {
                1.0 / f64::sqrt(f64::sqrt(f64::powi(self.s, 2) - f64::powi(v, 2)) / h - 1.0)
                    / (2.0 * f64::sqrt(6.0))
            }
        };

        let min_s = f64::sqrt(f64::powi(self.z1 - self.z0, 2) + f64::powi(self.x1 - self.x0, 2));
        if self.s <= min_s {
            return Err(CatenaryError::LengthTooShort {
                given: self.s,
                min: min_s,
            })?;
        }

        let mut brushes = Vec::new();

        let a = newton_a(
            v,
            h,
            self.s,
            self.x0,
            self.z0,
            self.x1,
            self.z1,
            initial_guess,
        )?;

        // Find the other catenary parameters. Thankfully these aren't as bad.
        let k: f64 = self.x0 + 0.5 * (h - a * f64::ln((self.s + v) / (self.s - v)));
        let c: f64 = self.z0 - a * f64::cosh((self.x0 - k) / a);

        // Split up into discrete segments.
        let dx = (self.x1 - self.x0) / (self.n as f64);

        for i in 0..self.n {
            // xs = x start, xe = x end
            // zs0/ze0 = z bottom, zs1/ze1 = z top
            let xs = self.x0 + dx * (i as f64);
            let xe = self.x0 + dx * (i as f64 + 1.0);
            let zs0 = catenary(xs, a, k, c) - self.t;
            let zs1 = catenary(xs, a, k, c);
            let ze0 = catenary(xe, a, k, c) - self.t;
            let ze1 = catenary(xe, a, k, c);

            let pa = DVec3 {
                x: xe,
                y: 0.0,
                z: ze0,
            };
            let pb = DVec3 {
                x: xs,
                y: 0.0,
                z: zs0,
            };
            let pc = DVec3 {
                x: xs,
                y: self.w,
                z: zs0,
            };
            let pd = DVec3 {
                x: xe,
                y: self.w,
                z: ze0,
            };
            let pe = DVec3 {
                x: xe,
                y: 0.0,
                z: ze1,
            };
            let pf = DVec3 {
                x: xs,
                y: 0.0,
                z: zs1,
            };
            let pg = DVec3 {
                x: xs,
                y: self.w,
                z: zs1,
            };
            let ph = DVec3 {
                x: xe,
                y: self.w,
                z: ze1,
            };

            brushes.push(Brush::try_from_vertices(
                &[pa, pb, pc, pd, pe, pf, pg, ph],
                MAX_HULL_ITER,
            )?);
        }

        Ok(brushes)
    }
}

#[derive(Error, Debug)]
pub enum CatenaryError {
    #[error("n = {n}. Number of segments must be at least 1.")]
    NotEnoughSegments { n: u32 },
    #[error("n = {n}. Number of segments must be no greater than 4096.")]
    TooManySegments { n: u32 },
    #[error("Given length {given} is too short; must be greater than {min}.")]
    LengthTooShort { given: f64, min: f64 },
    #[error("Newton's method failed to converge to an accurate solution after {iterations} iterations. The initial guess was {initial}. Change the parameters to a less extreme catenary curve, or try again with a different initial guess.")]
    NewtonFail { iterations: i32, initial: f64 },
}

// ---------- FINDING THE CATENARY PARAMETER "a" ----------
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
