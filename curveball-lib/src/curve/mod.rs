pub mod rayto;

use crate::brush::Brush;
use thiserror::Error;

pub trait Curve {
    fn bake(&self) -> Result<Vec<Brush>, CurveError>;
}

#[derive(Error, Debug)]
pub enum CurveError {
    #[error("Error calculating convex hull: {0}")]
    ConvexHullFail(#[from] chull::ErrorKind),
}

pub type CurveResult<T> = Result<T, CurveError>;
