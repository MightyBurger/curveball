use crate::curve::{Curve, CurveError, CurveResult, MAX_HULL_ITER};
use crate::map::Brush;
use glam::DVec3;
use itertools::Itertools;
use thiserror::Error;

use lerp::LerpIter;

use std::f64::consts::PI;

#[derive(Debug, Default, Clone)]
pub struct Serpentine {
    pub n_each: u32,
    pub x: f64,
    pub z: f64,
    pub w: f64,
    pub t: f64,
    pub offset: SerpentineOffsetMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SerpentineOffsetMode {
    Top,
    Middle,
    Bottom,
}

impl Default for SerpentineOffsetMode {
    fn default() -> Self {
        Self::Middle
    }
}

impl Curve for Serpentine {
    fn bake(&self) -> CurveResult<Vec<Brush>> {
        if self.n_each < 1 {
            return Err(SerpentineError::NotEnoughSegments {
                n_each: self.n_each,
            })?;
        }
        if self.n_each > 2048 {
            return Err(SerpentineError::TooManySegments {
                n_each: self.n_each,
            })?;
        }
        if !(self.z > 0.0) {
            return Err(SerpentineError::OrderedHeight)?;
        }
        if self.z > self.x {
            return Err(SerpentineError::TooTall)?;
        }

        let xm = self.x / 2.0;
        let zm = self.z / 2.0;

        let r0 = ((zm * zm) + (xm * xm)) / (2.0 * zm);

        let zd = self.z - zm;
        let xd = self.x - xm;
        let r1 = ((zd * zd) + (xd * xd)) / (2.0 * zd);

        let theta0_start = -PI / 2.0;
        let theta0_end = f64::asin(xm / r0) - PI / 2.0;

        let theta1_start = PI / 2.0 + f64::asin(xd / r1);
        let theta1_end = PI / 2.0;

        let arc0_iter = theta0_start.lerp_iter_closed(theta0_end, self.n_each as usize + 1);
        let arc1_iter = theta1_start.lerp_iter_closed(theta1_end, self.n_each as usize + 1);

        let brush_iter0 = arc0_iter
            .map(|dtheta| {
                let r_in = match self.offset {
                    SerpentineOffsetMode::Top => r0,
                    SerpentineOffsetMode::Middle => (r0 + (r0 - self.t)) / 2.0,
                    SerpentineOffsetMode::Bottom => r0 - self.t,
                };

                let r_out = match self.offset {
                    SerpentineOffsetMode::Top => r0 + self.t,
                    SerpentineOffsetMode::Middle => (r0 + (r0 + self.t)) / 2.0,
                    SerpentineOffsetMode::Bottom => r0,
                };

                let pa = DVec3 {
                    x: r_in * f64::cos(dtheta),
                    y: 0.0,
                    z: r_in * f64::sin(dtheta) + r_in,
                };

                let pb = DVec3 {
                    x: r_out * f64::cos(dtheta),
                    y: 0.0,
                    z: r_out * f64::sin(dtheta) + r_in,
                };

                let pc = DVec3 { y: self.w, ..pa };
                let pd = DVec3 { y: self.w, ..pb };

                [pa, pb, pc, pd]
            })
            .tuple_windows()
            .map(|(f1, f2)| {
                let vertices: Vec<DVec3> = f1.into_iter().chain(f2.into_iter()).collect();
                Brush::try_from_vertices(&vertices, MAX_HULL_ITER)
            });

        let brush_iter1 = arc1_iter
            .map(|dtheta| {
                let r_in = match self.offset {
                    SerpentineOffsetMode::Top => r1 - self.t,
                    SerpentineOffsetMode::Middle => (r1 + (r1 - self.t)) / 2.0,
                    SerpentineOffsetMode::Bottom => r1,
                };

                let r_out = match self.offset {
                    SerpentineOffsetMode::Top => r1,
                    SerpentineOffsetMode::Middle => (r1 + (r1 + self.t)) / 2.0,
                    SerpentineOffsetMode::Bottom => r1 + self.t,
                };

                let pa = DVec3 {
                    x: r_out * f64::cos(dtheta) + self.x,
                    y: 0.0,
                    z: r_out * f64::sin(dtheta) - r_out + self.z,
                };

                let pb = DVec3 {
                    x: r_in * f64::cos(dtheta) + self.x,
                    y: 0.0,
                    z: r_in * f64::sin(dtheta) - r_out + self.z,
                };

                let pc = DVec3 { y: self.w, ..pa };
                let pd = DVec3 { y: self.w, ..pb };

                [pa, pb, pc, pd]
            })
            .tuple_windows()
            .map(|(f1, f2)| {
                let vertices: Vec<DVec3> = f1.into_iter().chain(f2.into_iter()).collect();
                Brush::try_from_vertices(&vertices, MAX_HULL_ITER)
            });

        brush_iter0
            .chain(brush_iter1)
            .map(|brush_result| brush_result.map_err(|err| CurveError::from(err)))
            .collect()
    }
}
#[derive(Error, Debug)]
pub enum SerpentineError {
    #[error("n_each = {n_each}. Number of segments in either side must be at least 1.")]
    NotEnoughSegments { n_each: u32 },
    #[error("n_each = {n_each}. Number of segments in either side must be no greater than 2048.")]
    TooManySegments { n_each: u32 },
    #[error("Ending height must be greater than the starting height.")]
    OrderedHeight,
    #[error("Serpentine curve height cannot be greater than its length.")]
    TooTall,
}
