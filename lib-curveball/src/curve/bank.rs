// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::curve::{Curve, CurveResult, MAX_HULL_ITER};
use crate::map::geometry::Brush;
use glam::DVec3;
use itertools::Itertools;
use lerp::LerpIter;
use thiserror::Error;

use std::f64::consts::PI;

use super::CurveError;

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

        self.theta0
            .lerp_iter_closed(self.theta1, self.n as usize + 1)
            .map(|dtheta| {
                let pa = DVec3 {
                    x: self.ro * f64::cos(deg2rad(dtheta)),
                    y: self.ro * f64::sin(deg2rad(dtheta)),
                    z: { if self.fill { -self.t } else { self.h - self.t } },
                };
                let pb = DVec3 {
                    x: self.ri * f64::cos(deg2rad(dtheta)),
                    y: self.ri * f64::sin(deg2rad(dtheta)),
                    z: -self.t,
                };

                let pc = DVec3 {
                    x: self.ro * f64::cos(deg2rad(dtheta)),
                    y: self.ro * f64::sin(deg2rad(dtheta)),
                    z: self.h,
                };
                let pd = DVec3 {
                    x: self.ri * f64::cos(deg2rad(dtheta)),
                    y: self.ri * f64::sin(deg2rad(dtheta)),
                    z: 0.0,
                };
                [pa, pb, pc, pd]
            })
            .tuple_windows()
            .map(|(f1, f2)| {
                let vertices: Vec<DVec3> = f1.into_iter().chain(f2).collect();
                Brush::try_from_vertices(&vertices, MAX_HULL_ITER)
            })
            .map(|brush_result| brush_result.map_err(CurveError::from))
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum BankError {
    #[error("n = {n}. Number of segments must be at least 1.")]
    NotEnoughSegments { n: u32 },
    #[error("n = {n}. Number of segments must be no greater than 4096.")]
    TooManySegments { n: u32 },
}
