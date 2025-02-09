// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::curve::{CurveError, CurveResult, MAX_HULL_ITER};
use crate::map::geometry::Brush;
use glam::{DVec3, Mat3, Vec3};
use itertools::Itertools;
use lerp::LerpIter;
use thiserror::Error;

pub mod path;
pub mod profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProfileOrientation {
    Constant,
    FollowPath,
}

impl Default for ProfileOrientation {
    fn default() -> Self {
        Self::Constant
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrenetFrame {
    pub tangent: DVec3,
    pub normal: DVec3,
    pub binormal: DVec3,
}

fn dvec3_to_vec3(vecin: DVec3) -> Vec3 {
    Vec3 {
        x: vecin.x as f32,
        y: vecin.y as f32,
        z: vecin.z as f32,
    }
}

// Extrude along a parameterized curve.
// n: number of segments of the path
// profile: a function that, given a parameter t, returns an iterable of points in a face.
//  The profile is expected to be constant (independent of the parameter) for almost
//  all curves, but the functionality exists nonetheless.
//  The points are expected to lie in the YZ plane - and all of the default Curveball
//  profiles do this - but it isn't strictly required.
// path: a function that, given a parameter t, returns a point in 3D space and the Frenet Frame
//  at that point. The Frenet Frame is used when profile_orientation is set to FollowPath, and
//  is ignored otherwise. The Frenet Frame is a set of three basis vectors representing the
//  orientation at any point along the curve. The Frenet Frame should evaluate to
//  {tangent: [1, 0, 0], normal: [0, 1, 0], binormal: [0, 0, 1]]} when the parameter is zero.
//  start, end: the start and end values of the parameter
pub fn extrude<PRF, PI, PPF, PFF>(
    n: u32,
    profile_fn: PRF,
    path_point_fn: PPF,
    path_frenet_frame_fn: PFF,
    start: f64,
    end: f64,
    profile_orientation: ProfileOrientation,
) -> CurveResult<Vec<Brush>>
where
    PRF: Fn(f64) -> PI,
    PI: IntoIterator<Item = DVec3>,
    PPF: Fn(f64) -> DVec3,
    PFF: Fn(f64) -> FrenetFrame,
{
    // Iterate over every point in the path.
    // Work on windows of two consecutive points along the path at a time.
    start
        .lerp_iter_closed(end, n as usize + 1)
        .map(|t| {
            let path_point = path_point_fn(t);
            let frenet_frame = path_frenet_frame_fn(t);
            let this_profile = profile_fn(t);
            let face: Vec<_> = this_profile
                .into_iter()
                .map(|mut profile_point| {
                    if matches!(profile_orientation, ProfileOrientation::FollowPath) {
                        let rmat = Mat3::from_cols(
                            dvec3_to_vec3(frenet_frame.tangent),
                            dvec3_to_vec3(frenet_frame.normal),
                            dvec3_to_vec3(frenet_frame.binormal),
                        );
                        profile_point = rmat.mul_vec3(dvec3_to_vec3(profile_point)).into();
                    }
                    profile_point = profile_point + path_point;
                    profile_point
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum PathPlane {
//     XZ,
//     XY,
// }
//
// fn dir_vec<PF>(path: &PF, about: f64) -> DVec2
// where
//     PF: Fn(f64) -> DVec2,
// {
//     let h = 0.000001;
//     ((path(about + h) - path(about - h)) / (2.0 * h)).normalize()
// }
//
// pub fn extrude_planecurve_many<SII, SI, PF>(
//     n: u32,
//     profiles: SII,
//     path: PF,
//     path_start: f64,
//     path_end: f64,
//     profile_orientation: ProfileOrientation,
//     path_plane: PathPlane,
// ) -> CurveResult<Vec<Brush>>
// where
//     SII: IntoIterator<Item = SI> + Clone,
//     SI: IntoIterator<Item = DVec2> + Clone,
//     PF: Fn(f64) -> DVec2,
// {
//     let mut result = Vec::new();
//     for profile in profiles.into_iter() {
//         for brush in extrude_planecurve_once(
//             n,
//             profile,
//             &path,
//             path_start,
//             path_end,
//             profile_orientation,
//             path_plane,
//         )? {
//             result.push(brush);
//         }
//     }
//     Ok(result)
// }
//
// pub fn extrude_planecurve_once<SI, PF>(
//     n: u32,
//     profile: SI,
//     path: PF,
//     path_start: f64,
//     path_end: f64,
//     profile_orientation: ProfileOrientation,
//     path_plane: PathPlane,
// ) -> CurveResult<Vec<Brush>>
// where
//     SI: IntoIterator<Item = DVec2> + Clone,
//     PF: Fn(f64) -> DVec2,
// {
//     if n < 1 {
//         return Err(ExtrudeError::NotEnoughSegments { n })?;
//     }
//     if n > 4096 {
//         return Err(ExtrudeError::TooManySegments { n })?;
//     }
//
//     // Iterate over every point in the path.
//     // Work on windows of two consecutive points along the path at a time.
//     path_start
//         .lerp_iter_closed(path_end, n as usize + 1)
//         .map(|t| {
//             let path_point = path(t);
//             let face: Vec<_> = profile
//                 .clone()
//                 .into_iter()
//                 .map(|profile_point| {
//                     let mut this_point;
//                     match path_plane {
//                         PathPlane::XZ => {
//                             let DVec2 {
//                                 x: sketch_y,
//                                 y: sketch_z,
//                             } = profile_point;
//                             let DVec2 {
//                                 x: path_x,
//                                 y: path_z,
//                             } = path_point;
//                             this_point = DVec3::from([path_x, sketch_y, sketch_z + path_z]);
//
//                             // Apply an additional rotation step, if desired.
//                             if matches!(profile_orientation, ProfileOrientation::FollowPath) {
//                                 let dirx_2d = dir_vec(&path, t);
//                                 let dirx = Vec3 {
//                                     x: dirx_2d.x as f32,
//                                     y: 0.0,
//                                     z: dirx_2d.y as f32,
//                                 };
//                                 let diry = Vec3 {
//                                     x: 0.0,
//                                     y: 1.0,
//                                     z: 0.0,
//                                 };
//                                 let dirz = dirx.cross(diry);
//                                 let rmat = Mat3::from_cols(dirx.into(), diry.into(), dirz.into());
//                                 this_point = rmat
//                                     .mul_vec3(Vec3 {
//                                         x: (this_point.x - path_x) as f32,
//                                         y: (this_point.y) as f32,
//                                         z: (this_point.z - path_z) as f32,
//                                     })
//                                     .into();
//                                 this_point = DVec3 {
//                                     x: this_point.x + path_x,
//                                     y: this_point.y,
//                                     z: this_point.z + path_z,
//                                 };
//                             }
//                         }
//                         PathPlane::XY => {
//                             let DVec2 {
//                                 x: sketch_y,
//                                 y: sketch_z,
//                             } = profile_point;
//                             let DVec2 {
//                                 x: path_x,
//                                 y: path_y,
//                             } = path_point;
//                             this_point = DVec3::from([path_x, sketch_y + path_y, sketch_z]);
//
//                             // Apply an additional rotation step, if desired.
//                             if matches!(profile_orientation, ProfileOrientation::FollowPath) {
//                                 let dirx_2d = dir_vec(&path, t);
//                                 let dirx = Vec3 {
//                                     x: dirx_2d.x as f32,
//                                     y: dirx_2d.y as f32,
//                                     z: 0.0,
//                                 };
//                                 let dirz = Vec3 {
//                                     x: 0.0,
//                                     y: 0.0,
//                                     z: 1.0,
//                                 };
//                                 let diry = dirz.cross(dirx);
//                                 let rmat = Mat3::from_cols(dirx.into(), diry.into(), dirz.into());
//                                 this_point = rmat
//                                     .mul_vec3(Vec3 {
//                                         x: (this_point.x - path_x) as f32,
//                                         y: (this_point.y - path_y) as f32,
//                                         z: (this_point.z) as f32,
//                                     })
//                                     .into();
//                                 this_point = DVec3 {
//                                     x: this_point.x + path_x,
//                                     y: this_point.y + path_y,
//                                     z: this_point.z,
//                                 };
//                             }
//                         }
//                     }
//
//                     this_point
//                 })
//                 .collect();
//             face
//         })
//         .tuple_windows()
//         .map(|(face1, face2)| {
//             let vertices: Vec<DVec3> = face1.into_iter().chain(face2.into_iter()).collect();
//             Brush::try_from_vertices(&vertices, MAX_HULL_ITER)
//         })
//         .map(|brush_result| brush_result.map_err(CurveError::from))
//         .collect()
// }

#[derive(Error, Debug)]
pub enum ExtrudeError {
    #[error("n = {n}. Number of segments must be at least 1.")]
    NotEnoughSegments { n: u32 },
    #[error("n = {n}. Number of segments must be no greater than 4096.")]
    TooManySegments { n: u32 },
    #[error("This path does not support FollowPath yet.")]
    FollowPathNotImplemented,
}
