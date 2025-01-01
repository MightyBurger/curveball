use crate::curve::{Curve, CurveResult, MAX_HULL_ITER};
use crate::map::Brush;
use glam::DVec3;
use thiserror::Error;

use std::f64::consts::PI;

#[derive(Debug, Default, Clone)]
pub struct Bank {
    pub n: u32,
    pub ri: f64,
    pub ro: f64,
    pub theta0: f64,
    pub theta1: f64,
    pub h: f64,
    pub t: f64,
    pub fill: bool,
}

fn deg2rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

impl Curve for Bank {
    fn bake(&self) -> CurveResult<Vec<Brush>> {
        if self.n < 1 {
            return Err(BankError::NotEnoughSegments { n: self.n })?;
        }
        if self.n > 4096 {
            return Err(BankError::TooManySegments { n: self.n })?;
        }

        let dtheta = (self.theta1 - self.theta0) / (self.n as f64);

        let mut brushes = Vec::new();

        for i in 0..self.n {
            // bounds for this differential segment
            // s = start, e = end
            // o = outer, i = inner
            let thetas = self.theta0 + dtheta * (i as f64);
            let thetae = self.theta0 + dtheta * (i as f64 + 1.0);

            // points for this differential segment
            // starting with pa at bottom going counter-clockwise,
            // then moving up to pe and going counter-clockwise again,
            // ending with ph

            let pa = DVec3 {
                x: self.ro * f64::cos(deg2rad(thetas)),
                y: self.ro * f64::sin(deg2rad(thetas)),
                z: {
                    if self.fill {
                        -self.t
                    } else {
                        self.h - self.t
                    }
                },
            };

            let pb = DVec3 {
                x: self.ro * f64::cos(deg2rad(thetae)),
                y: self.ro * f64::sin(deg2rad(thetae)),
                z: {
                    if self.fill {
                        -self.t
                    } else {
                        self.h - self.t
                    }
                },
            };

            let pc = DVec3 {
                x: self.ri * f64::cos(deg2rad(thetae)),
                y: self.ri * f64::sin(deg2rad(thetae)),
                z: -self.t,
            };

            let pd = DVec3 {
                x: self.ri * f64::cos(deg2rad(thetas)),
                y: self.ri * f64::sin(deg2rad(thetas)),
                z: -self.t,
            };

            let pe = DVec3 {
                x: self.ro * f64::cos(deg2rad(thetas)),
                y: self.ro * f64::sin(deg2rad(thetas)),
                z: self.h,
            };

            let pf = DVec3 {
                x: self.ro * f64::cos(deg2rad(thetae)),
                y: self.ro * f64::sin(deg2rad(thetae)),
                z: self.h,
            };

            let pg = DVec3 {
                x: self.ri * f64::cos(deg2rad(thetae)),
                y: self.ri * f64::sin(deg2rad(thetae)),
                z: 0.0,
            };

            let ph = DVec3 {
                x: self.ri * f64::cos(deg2rad(thetas)),
                y: self.ri * f64::sin(deg2rad(thetas)),
                z: 0.0,
            };

            brushes.push(Brush::try_from_vertices(
                &vec![pa, pb, pc, pd, pe, pf, pg, ph],
                MAX_HULL_ITER,
            )?);
        }

        Ok(brushes)
    }
}
#[derive(Error, Debug)]
pub enum BankError {
    #[error("n = {n}. Number of segments must be at least 1.")]
    NotEnoughSegments { n: u32 },
    #[error("n = {n}. Number of segments must be no greater than 4096.")]
    TooManySegments { n: u32 },
}
