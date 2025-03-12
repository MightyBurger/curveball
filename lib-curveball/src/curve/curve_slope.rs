// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A highly configurable circular arc.

use crate::curve::{Curve, CurveResult, MAX_HULL_ITER};
use crate::map::geometry::Brush;
use glam::DVec3;
use itertools::{Itertools, izip};
use lerp::LerpIter;
use thiserror::Error;

use std::f64::consts::PI;

use super::CurveError;

#[derive(Debug, Default, Clone)]
pub struct CurveSlope {
    pub n: u32,
    pub ri0: f64,
    pub ro0: f64,
    pub ri1: f64,
    pub ro1: f64,
    pub theta0: f64,
    pub theta1: f64,
    pub height_inner_top_0: f64,
    pub height_inner_bot_0: f64,
    pub height_outer_top_0: f64,
    pub height_outer_bot_0: f64,
    pub height_inner_top_1: f64,
    pub height_inner_bot_1: f64,
    pub height_outer_top_1: f64,
    pub height_outer_bot_1: f64,
    pub hill_inner_top: f64,
    pub hill_inner_bot: f64,
    pub hill_outer_top: f64,
    pub hill_outer_bot: f64,
}

fn deg2rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

impl Curve for CurveSlope {
    fn bake(&self) -> CurveResult<Vec<Brush>> {
        if self.n < 1 {
            return Err(CurveSlopeError::NotEnoughSegments { n: self.n })?;
        }
        if self.n > 4096 {
            return Err(CurveSlopeError::TooManySegments { n: self.n })?;
        }

        let n_iter = self.n as usize + 1;

        let ri_iter = self.ri0.lerp_iter_closed(self.ri1, n_iter);
        let ro_iter = self.ro0.lerp_iter_closed(self.ro1, n_iter);
        let theta_iter = self.theta0.lerp_iter_closed(self.theta1, n_iter);
        let height_inner_top_iter = self
            .height_inner_top_0
            .lerp_iter_closed(self.height_inner_top_1, n_iter);
        let height_inner_bot_iter = self
            .height_inner_bot_0
            .lerp_iter_closed(self.height_inner_bot_1, n_iter);
        let height_outer_top_iter = self
            .height_outer_top_0
            .lerp_iter_closed(self.height_outer_top_1, n_iter);
        let height_outer_bot_iter = self
            .height_outer_bot_0
            .lerp_iter_closed(self.height_outer_bot_1, n_iter);
        let hill_iter = (-PI).lerp_iter_closed(PI, n_iter);

        izip!(
            ri_iter,
            ro_iter,
            theta_iter,
            height_inner_top_iter,
            height_inner_bot_iter,
            height_outer_top_iter,
            height_outer_bot_iter,
            hill_iter
        )
        .map(
            |(
                i_ri,
                i_ro,
                i_theta,
                i_height_inner_top,
                i_height_inner_bot,
                i_height_outer_top,
                i_height_outer_bot,
                i_hill,
            )| {
                let i_hill_inner_top = self.hill_inner_top * (1.0 + i_hill.cos()) / 2.0;
                let i_hill_inner_bot = self.hill_inner_bot * (1.0 + i_hill.cos()) / 2.0;
                let i_hill_outer_top = self.hill_outer_top * (1.0 + i_hill.cos()) / 2.0;
                let i_hill_outer_bot = self.hill_outer_bot * (1.0 + i_hill.cos()) / 2.0;

                let p_ri_top = DVec3 {
                    x: i_ri * deg2rad(i_theta).cos(),
                    y: i_ri * deg2rad(i_theta).sin(),
                    z: i_height_inner_top + i_hill_inner_top,
                };

                let p_ri_bot = DVec3 {
                    x: i_ri * deg2rad(i_theta).cos(),
                    y: i_ri * deg2rad(i_theta).sin(),
                    z: i_height_inner_bot + i_hill_inner_bot,
                };

                let p_ro_top = DVec3 {
                    x: i_ro * deg2rad(i_theta).cos(),
                    y: i_ro * deg2rad(i_theta).sin(),
                    z: i_height_outer_top + i_hill_outer_top,
                };

                let p_ro_bot = DVec3 {
                    x: i_ro * deg2rad(i_theta).cos(),
                    y: i_ro * deg2rad(i_theta).sin(),
                    z: i_height_outer_bot + i_hill_outer_bot,
                };

                [p_ri_top, p_ri_bot, p_ro_top, p_ro_bot]
            },
        )
        .tuple_windows()
        .flat_map(
            |(
                [f1_ri_top, f1_ri_bot, f1_ro_top, f1_ro_bot],
                [f2_ri_top, f2_ri_bot, f2_ro_top, f2_ro_bot],
            )| {
                let brush1 = Brush::try_from_vertices(
                    &[
                        f1_ri_top, f1_ri_bot, f1_ro_top, f1_ro_bot, f2_ro_top, f2_ro_bot,
                    ],
                    MAX_HULL_ITER,
                );
                let brush2 = Brush::try_from_vertices(
                    &[
                        f1_ri_top, f1_ri_bot, f2_ri_top, f2_ri_bot, f2_ro_top, f2_ro_bot,
                    ],
                    MAX_HULL_ITER,
                );
                [brush1, brush2].into_iter()
            },
        )
        .map(|brush_result| brush_result.map_err(CurveError::from))
        .collect()
    }
}

#[derive(Error, Debug)]
pub enum CurveSlopeError {
    #[error("n = {n}. Number of segments must be at least 1.")]
    NotEnoughSegments { n: u32 },
    #[error("n = {n}. Number of segments must be no greater than 4096.")]
    TooManySegments { n: u32 },
}
