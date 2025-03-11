// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::curve::{Curve, CurveResult, MAX_HULL_ITER};
use crate::map::geometry::Brush;
use glam::DVec3;
use itertools::{Itertools, izip};
use lerp::LerpIter;
use thiserror::Error;

use std::f64::consts::PI;

use super::CurveError;

#[derive(Debug, Default, Clone)]
pub struct CurveClassic {
    pub n: u32,
    pub ri0: f64,
    pub ro0: f64,
    pub ri1: f64,
    pub ro1: f64,
    pub theta0: f64,
    pub theta1: f64,
    pub t: f64,
}

fn deg2rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

impl Curve for CurveClassic {
    fn bake(&self) -> CurveResult<Vec<Brush>> {
        if self.n < 1 {
            return Err(CurveClassicError::NotEnoughSegments { n: self.n })?;
        }
        if self.n > 4096 {
            return Err(CurveClassicError::TooManySegments { n: self.n })?;
        }

        let n_iter = self.n as usize + 1;

        let ri_iter = self.ri0.lerp_iter_closed(self.ri1, n_iter);
        let ro_iter = self.ro0.lerp_iter_closed(self.ro1, n_iter);
        let theta_iter = self.theta0.lerp_iter_closed(self.theta1, n_iter);

        izip!(ri_iter, ro_iter, theta_iter)
            .map(|(i_ri, i_ro, i_theta)| {
                let p_ri_top = DVec3 {
                    x: i_ri * deg2rad(i_theta).cos(),
                    y: i_ri * deg2rad(i_theta).sin(),
                    z: self.t,
                };

                let p_ri_bot = DVec3 {
                    x: i_ri * deg2rad(i_theta).cos(),
                    y: i_ri * deg2rad(i_theta).sin(),
                    z: 0.0,
                };

                let p_ro_top = DVec3 {
                    x: i_ro * deg2rad(i_theta).cos(),
                    y: i_ro * deg2rad(i_theta).sin(),
                    z: self.t,
                };

                let p_ro_bot = DVec3 {
                    x: i_ro * deg2rad(i_theta).cos(),
                    y: i_ro * deg2rad(i_theta).sin(),
                    z: 0.0,
                };

                [p_ri_top, p_ri_bot, p_ro_top, p_ro_bot]
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
pub enum CurveClassicError {
    #[error("n = {n}. Number of segments must be at least 1.")]
    NotEnoughSegments { n: u32 },
    #[error("n = {n}. Number of segments must be no greater than 4096.")]
    TooManySegments { n: u32 },
}
