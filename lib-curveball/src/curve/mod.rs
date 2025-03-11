// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::map::geometry::Brush;
use thiserror::Error;

pub mod bank;
pub mod curve_classic;
pub mod curve_slope;
pub mod extrude;
pub mod rayto;

const MAX_HULL_ITER: Option<usize> = Some(10_000);

pub trait Curve {
    fn bake(&self) -> Result<Vec<Brush>, CurveError>;
}

#[derive(Error, Debug)]
pub enum CurveError {
    #[error("{0}")]
    CurveClassicError(#[from] curve_classic::CurveClassicError),

    #[error("{0}")]
    CurveSlopeError(#[from] curve_slope::CurveSlopeError),

    #[error("Failed to find convex hull: {0}")]
    ConvexHullFail(#[from] chull::ErrorKind),

    #[error("{0}")]
    RaytoError(#[from] rayto::RaytoError),

    #[error("{0}")]
    BankError(#[from] bank::BankError),

    #[error("{0}")]
    ExtrudeError(#[from] extrude::ExtrudeError),

    #[error("{0}")]
    ProfileError(#[from] extrude::profile::ProfileError),

    #[error("{0}")]
    PathError(#[from] extrude::path::PathError),
}

pub type CurveResult<T> = Result<T, CurveError>;
