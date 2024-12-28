pub mod rayto;
pub use rayto::Rayto;

pub mod bank;
pub use bank::Bank;

pub mod catenary;
pub use catenary::Catenary;

const MAX_HULL_ITER: Option<usize> = Some(10_000);

use crate::map::geometry::Brush;
use thiserror::Error;

pub trait Curve {
    fn bake(&self) -> Result<Vec<Brush>, CurveError>;
}

#[derive(Error, Debug)]
pub enum CurveError {
    #[error("Failed to find convex hull: {0}")]
    ConvexHullFail(#[from] chull::ErrorKind),
    #[error("Given length is too short")]
    CatenaryTooShort,
    #[error("Newton's method did not converge")]
    CatenaryNetwonFail,
}

pub type CurveResult<T> = Result<T, CurveError>;
