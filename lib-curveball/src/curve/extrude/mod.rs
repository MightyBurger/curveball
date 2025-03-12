// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Functions to produce curves by extruding a 2D profile along a path in 3D space.

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

/// A [Frenet frame](https://en.wikipedia.org/wiki/Frenet%E2%80%93Serret_formulas) used to describe the orientation of a profile along a path.
///
/// The `tangent` vector is always equal to the derivative of the path's parametric function
/// with respect to time, normalized.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrenetFrame {
    pub tangent: DVec3,
    pub normal: DVec3,
    pub binormal: DVec3,
}

/// Extrude a 2D profile along a 3D path.
///
/// `n`: Number of discrete segments to produce
///
/// `profile`: A 2D profile
///
/// `path`: A path in 3D space
///
/// `profile_orientation`: Determines how the profile should be oriented as it extrudes through
/// space
pub fn extrude<PRF, PTH>(
    n: u32,
    profile: &PRF,
    path: &PTH,
    profile_orientation: ProfileOrientation,
) -> CurveResult<Vec<Brush>>
where
    PRF: Profile,
    PTH: Path,
{
    if n < 1 {
        Err(ExtrudeError::NotEnoughSegments { n })?;
    }
    if n > 4096 {
        Err(ExtrudeError::TooManySegments { n })?;
    }

    let start = 0.0;
    let end = 1.0;

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
            let vertices: Vec<DVec3> = face1.into_iter().chain(face2).collect();
            Brush::try_from_vertices(&vertices, MAX_HULL_ITER)
        })
        .map(|brush_result| brush_result.map_err(CurveError::from))
        .collect()
}

/// Extrude a compound 2D profile along a 3D path.
///
/// `n`: Number of discrete segments to produce
///
/// `profile`: A "compound" 2D profile, i.e. one containing multiple polygons
///
/// `path`: A path in 3D space
///
/// `profile_orientation`: Determines how the profile should be oriented as it extrudes through
/// space
pub fn extrude_multi<CPF, PTH>(
    n: u32,
    compound_profile: &CPF,
    path: &PTH,
    profile_orientation: ProfileOrientation,
) -> CurveResult<Vec<Brush>>
where
    CPF: CompoundProfile,
    PTH: Path,
{
    let n_compound = compound_profile.compound_profile(0.0).len() * n as usize;
    if n_compound < 1 {
        Err(ExtrudeError::NotEnoughSegments {
            n: n_compound as u32,
        })?;
    }
    if n_compound > 4096 {
        Err(ExtrudeError::TooManySegments {
            n: n_compound as u32,
        })?;
    }

    let start = 0.0;
    let end = 1.0;

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
                .zip(faces2)
                .map(|(face1, face2)| {
                    let vertices: Vec<DVec3> = face1.into_iter().chain(face2).collect();
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
}
