// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::curve::{CurveError, CurveResult, MAX_HULL_ITER};
use crate::map::Brush;
use glam::{DVec2, DVec3, Mat3, Vec3};
use itertools::Itertools;
use lerp::LerpIter;
use thiserror::Error;

pub mod path;
pub mod profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileOrientation {
    Constant,
    FollowPath,
}

impl Default for ProfileOrientation {
    fn default() -> Self {
        Self::Constant
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathPlane {
    XZ,
    XY,
}

fn dir_vec<PF>(path: &PF, about: f64) -> DVec2
where
    PF: Fn(f64) -> DVec2,
{
    let h = 0.000001;
    ((path(about + h) - path(about - h)) / (2.0 * h)).normalize()
}

pub fn extrude_plane_curve<SI, PF>(
    n: u32,
    profile_2d: SI, // sketch in the YZ plane
    path: PF,       // path in the XZ plane - why? it is more complex to do it outside a plane
    path_start: f64,
    path_end: f64,
    profile_orientation: ProfileOrientation,
    path_plane: PathPlane,
) -> CurveResult<Vec<Brush>>
where
    SI: IntoIterator<Item = DVec2> + Clone,
    PF: Fn(f64) -> DVec2,
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
        .map(|t| {
            let path_point = path(t);
            let face: Vec<_> = profile_2d
                .clone()
                .into_iter()
                .map(|profile_point| {
                    let mut this_point;
                    match path_plane {
                        PathPlane::XZ => {
                            let DVec2 {
                                x: sketch_y,
                                y: sketch_z,
                            } = profile_point;
                            let DVec2 {
                                x: path_x,
                                y: path_z,
                            } = path_point;
                            this_point = DVec3::from([path_x, sketch_y, sketch_z + path_z]);

                            // Apply an additional rotation step, if desired.
                            if matches!(profile_orientation, ProfileOrientation::FollowPath) {
                                let dirx_2d = dir_vec(&path, t);
                                let dirx = Vec3 {
                                    x: dirx_2d.x as f32,
                                    y: 0.0,
                                    z: dirx_2d.y as f32,
                                };
                                let diry = Vec3 {
                                    x: 0.0,
                                    y: 1.0,
                                    z: 0.0,
                                };
                                let dirz = dirx.cross(diry);
                                let rmat = Mat3::from_cols(dirx.into(), diry.into(), dirz.into());
                                this_point = rmat
                                    .mul_vec3(Vec3 {
                                        x: (this_point.x - path_x) as f32,
                                        y: (this_point.y) as f32,
                                        z: (this_point.z - path_z) as f32,
                                    })
                                    .into();
                                this_point = DVec3 {
                                    x: this_point.x + path_x,
                                    y: this_point.y,
                                    z: this_point.z + path_z,
                                };
                            }
                        }
                        PathPlane::XY => {
                            let DVec2 {
                                x: sketch_y,
                                y: sketch_z,
                            } = profile_point;
                            let DVec2 {
                                x: path_x,
                                y: path_y,
                            } = path_point;
                            this_point = DVec3::from([path_x, sketch_y + path_y, sketch_z]);

                            // Apply an additional rotation step, if desired.
                            if matches!(profile_orientation, ProfileOrientation::FollowPath) {
                                let dirx_2d = dir_vec(&path, t);
                                let dirx = Vec3 {
                                    x: dirx_2d.x as f32,
                                    y: dirx_2d.y as f32,
                                    z: 0.0,
                                };
                                let dirz = Vec3 {
                                    x: 0.0,
                                    y: 0.0,
                                    z: 1.0,
                                };
                                let diry = dirz.cross(dirx);
                                let rmat = Mat3::from_cols(dirx.into(), diry.into(), dirz.into());
                                this_point = rmat
                                    .mul_vec3(Vec3 {
                                        x: (this_point.x - path_x) as f32,
                                        y: (this_point.y - path_y) as f32,
                                        z: (this_point.z) as f32,
                                    })
                                    .into();
                                this_point = DVec3 {
                                    x: this_point.x + path_x,
                                    y: this_point.y + path_y,
                                    z: this_point.z,
                                };
                            }
                        }
                    }

                    this_point
                })
                .collect();
            face
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
