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
    sketch_yz: SI,
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
    // for point in sketch_xz.clone().into_iter() {
    //     dbg!(point);
    // }

    // Iterate over every point in the path.
    // Work on windows of two consecutive points along the path at a time.
    path_start
        .lerp_iter_closed(path_end, n as usize + 1)
        .map(path_fn)
        .map(|path_point| {
            let a: Vec<_> = sketch_yz
                .clone()
                .into_iter()
                .map(
                    |DVec2 {
                         x: sketch_y,
                         y: sketch_z,
                     }| {
                        DVec3::from([
                            path_point.x,
                            sketch_y + path_point.y,
                            sketch_z + path_point.z,
                        ])
                    },
                )
                .collect();
            a
        })
        .tuple_windows()
        .map(|(face1, face2)| {
            let vertices: Vec<DVec3> = face1.into_iter().chain(face2.into_iter()).collect();
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
