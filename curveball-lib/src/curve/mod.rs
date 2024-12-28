pub mod rayto;
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
}

pub type CurveResult<T> = Result<T, CurveError>;
