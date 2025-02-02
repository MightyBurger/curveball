// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::curve::{CurveError, CurveResult, MAX_HULL_ITER};
use crate::map::Brush;
use glam::{DVec2, DVec3};
use itertools::Itertools;
use lerp::LerpIter;
use thiserror::Error;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ThicknessMode {
//     Vertical,
//     Orthogonal,
// }
//
// impl Default for ThicknessMode {
//     fn default() -> Self {
//         Self::Vertical
//     }
// }

pub fn extrude<SI, PF>(
    n: u32,
    sketch_xz: SI,
    path_fn: PF,
    path_start: f64,
    path_end: f64,
) -> CurveResult<Vec<Brush>>
where
    SI: IntoIterator<Item = DVec2> + Clone,
    PF: FnMut(f64) -> DVec3,
{
    if n < 1 {
        return Err(ExtrudeError::NotEnoughSegments { n })?;
    }
    if n > 4096 {
        return Err(ExtrudeError::TooManySegments { n })?;
    }
    // Iterate over every point in the path.
    // Work on windows of two consecutive points along the path at a time.
    path_start
        .lerp_iter_closed(path_end, n as usize + 1)
        .map(path_fn)
        .tuple_windows()
        .map(|(path_point1, path_point2)| {
            let face1 = sketch_xz.clone().into_iter().map(
                |DVec2 {
                     x: sketch_x,
                     y: sketch_z,
                 }| {
                    DVec3::from([
                        sketch_x + path_point1.x,
                        path_point1.y,
                        sketch_z + path_point1.z,
                    ])
                },
            );
            let face2 = sketch_xz.clone().into_iter().map(
                |DVec2 {
                     x: sketch_x,
                     y: sketch_z,
                 }| {
                    DVec3::from([
                        sketch_x + path_point2.x,
                        path_point1.y,
                        sketch_z + path_point2.z,
                    ])
                },
            );
            let vertices: Vec<DVec3> = face1.chain(face2).collect();
            Brush::try_from_vertices(&vertices, MAX_HULL_ITER)
        })
        .map(|brush_result| brush_result.map_err(CurveError::from))
        .collect()
}

#[derive(Error, Debug)]
pub enum ExtrudeError {
    #[error("n = {n}. Number of segments must be at least 1.")]
    NotEnoughSegments { n: u32 },
    #[error("n = {n}. Number of segments must be no greater than 4096.")]
    TooManySegments { n: u32 },
}
