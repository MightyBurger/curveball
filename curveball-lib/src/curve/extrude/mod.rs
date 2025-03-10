// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::curve::extrude::path::Path;
use crate::curve::{CurveError, CurveResult, MAX_HULL_ITER};
use crate::map::geometry::Brush;
use glam::{DMat3, DVec2, DVec3};
use itertools::Itertools;
use lerp::LerpIter;
use profile::{CompoundProfile, Profile};
use thiserror::Error;

pub mod path;
pub mod profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProfileOrientation {
    Constant(ProfilePlane),
    FollowPath,
}

impl std::fmt::Display for ProfileOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(ProfilePlane::XZ) => write!(f, "Constant (XZ)"),
            Self::Constant(ProfilePlane::YZ) => write!(f, "Constant (YZ)"),
            Self::Constant(ProfilePlane::XY) => write!(f, "Constant (XY)"),
            Self::FollowPath => write!(f, "Follow Path"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProfilePlane {
    XZ,
    YZ,
    XY,
}

fn make_3d(point_2d: DVec2, plane: ProfilePlane) -> DVec3 {
    match plane {
        ProfilePlane::XZ => DVec3 {
            x: point_2d.x,
            y: 0.0,
            z: point_2d.y,
        },
        ProfilePlane::YZ => DVec3 {
            x: 0.0,
            y: -point_2d.x,
            z: point_2d.y,
        },
        ProfilePlane::XY => DVec3 {
            x: point_2d.x,
            y: point_2d.y,
            z: 0.0,
        },
    }
}

impl Default for ProfileOrientation {
    fn default() -> Self {
        Self::FollowPath
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrenetFrame {
    pub tangent: DVec3,
    pub normal: DVec3,
    pub binormal: DVec3,
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
pub fn extrude<PRF, PTH>(
    n: u32,
    profile: &PRF,
    path: &PTH,
    start: f64,
    end: f64,
    profile_orientation: ProfileOrientation,
) -> CurveResult<Vec<Brush>>
where
    PRF: Profile,
    PTH: Path,
{
    // Iterate over every point in the path.
    // Work on windows of two consecutive points along the path at a time.
    start
        .lerp_iter_closed(end, n as usize + 1)
        .map(|t| {
            let path_point = path.point(t);
            let frenet_frame = path.frame(t);
            let this_face = profile.profile(t);
            let face: Vec<DVec3> = this_face
                .iter()
                .map(|profile_point_2d| {
                    let profile_point_3d = match profile_orientation {
                        ProfileOrientation::Constant(plane) => make_3d(*profile_point_2d, plane),
                        ProfileOrientation::FollowPath => {
                            let unrotated_point_3d = make_3d(*profile_point_2d, ProfilePlane::YZ);
                            let rmat = DMat3::from_cols(
                                frenet_frame.tangent,
                                frenet_frame.normal,
                                frenet_frame.binormal,
                            );
                            rmat.mul_vec3(unrotated_point_3d)
                        }
                    };
                    profile_point_3d + path_point
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

// Extrude along a parameterized curve with a path with multiple components.
// The arguments are the same as extrude, except the profile function is now an iterator over
// profile functions, each corresponding to a convex 2D profile.
pub fn extrude_multi<CPF, PTH>(
    n: u32,
    compound_profile: &CPF,
    path: &PTH,
    start: f64,
    end: f64,
    profile_orientation: ProfileOrientation,
) -> CurveResult<Vec<Brush>>
where
    CPF: CompoundProfile,
    PTH: Path,
{
    // Iterate over every point in the path.
    // Work on windows of two consecutive points along the path at a time.
    let brushes: Result<Vec<Vec<_>>, _> = start
        .lerp_iter_closed(end, n as usize + 1)
        .map(|t| {
            let path_point = path.point(t);
            let frenet_frame = path.frame(t);
            let these_faces = compound_profile.compound_profile(t);
            let mut faces: Vec<Vec<DVec3>> = Vec::new();
            for this_face in these_faces {
                let face: Vec<DVec3> = this_face
                    .iter()
                    .map(|profile_point_2d| {
                        let profile_point_3d = match profile_orientation {
                            ProfileOrientation::Constant(plane) => {
                                make_3d(*profile_point_2d, plane)
                            }
                            ProfileOrientation::FollowPath => {
                                let unrotated_point_3d =
                                    make_3d(*profile_point_2d, ProfilePlane::YZ);
                                let rmat = DMat3::from_cols(
                                    frenet_frame.tangent,
                                    frenet_frame.normal,
                                    frenet_frame.binormal,
                                );
                                rmat.mul_vec3(unrotated_point_3d)
                            }
                        };
                        profile_point_3d + path_point
                    })
                    .collect();
                faces.push(face);
            }
            faces
        })
        .tuple_windows()
        .map(|(faces1, faces2)| {
            let brushes: Result<Vec<_>, _> = faces1
                .into_iter()
                .zip(faces2.into_iter())
                .map(|(face1, face2)| {
                    let vertices: Vec<DVec3> = face1.into_iter().chain(face2.into_iter()).collect();
                    Brush::try_from_vertices(&vertices, MAX_HULL_ITER)
                })
                .collect();
            brushes
        })
        .map(|brush_result| brush_result.map_err(CurveError::from))
        .collect();
    brushes.map(|brushes| brushes.into_iter().flatten().collect())
}

#[derive(Error, Debug)]
pub enum ExtrudeError {
    #[error("n = {n}. Number of segments must be at least 1.")]
    NotEnoughSegments { n: u32 },
    #[error("n = {n}. Number of segments must be no greater than 4096.")]
    TooManySegments { n: u32 },
    #[error("This path does not support FollowPath yet.")]
    FollowPathNotImplemented,
}
