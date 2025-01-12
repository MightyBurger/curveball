// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::map::geometry::Brush;
use thiserror::Error;

pub mod curve_classic;
pub use curve_classic::{CurveClassic, CurveClassicError};

pub mod curve_slope;
pub use curve_slope::{CurveSlope, CurveSlopeError};

pub mod rayto;
pub use rayto::{Rayto, RaytoError};

pub mod bank;
pub use bank::{Bank, BankError};

pub mod catenary;
pub use catenary::{Catenary, CatenaryError};

pub mod serpentine;
pub use serpentine::{Serpentine, SerpentineError};

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
}

pub type CurveResult<T> = Result<T, CurveError>;
