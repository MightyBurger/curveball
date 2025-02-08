// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::map::geometry::Brush;
use thiserror::Error;

pub mod curve_classic;
use curve_classic::CurveClassicError;

pub mod curve_slope;
use curve_slope::CurveSlopeError;

pub mod rayto;
use rayto::RaytoError;

pub mod bank;
use bank::BankError;

pub mod catenary;
use catenary::CatenaryError;

pub mod serpentine;
use serpentine::SerpentineError;

pub mod extrude;
use extrude::{profile, ExtrudeError};

use profile::ProfileError;

const MAX_HULL_ITER: Option<usize> = Some(10_000);

pub trait Curve {
    fn bake(&self) -> Result<Vec<Brush>, CurveError>;
}

#[derive(Error, Debug)]
pub enum CurveError {
    #[error("{0}")]
    CurveClassicError(#[from] CurveClassicError),
    #[error("{0}")]
    CurveSlopeError(#[from] CurveSlopeError),
    #[error("Failed to find convex hull: {0}")]
    ConvexHullFail(#[from] chull::ErrorKind),
    #[error("{0}")]
    RaytoError(#[from] RaytoError),
    #[error("{0}")]
    BankError(#[from] BankError),
    #[error("{0}")]
    CatenaryError(#[from] CatenaryError),
    #[error("{0}")]
    SerpentineError(#[from] SerpentineError),
    #[error("{0}")]
    ExtrudeError(#[from] ExtrudeError),
    #[error("{0}")]
    ProfileError(#[from] ProfileError),
}

pub type CurveResult<T> = Result<T, CurveError>;
