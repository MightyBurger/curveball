pub mod rayto;

use crate::brush::Brush;
use std::error;
use std::fmt;

pub trait Curve {
    fn bake(&self) -> Result<Vec<Brush>, CurveError>;
}

#[derive(Debug)]
pub enum CurveError {
    ConvexHullFail(chull::ErrorKind),
}

impl fmt::Display for CurveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConvexHullFail(e) => write!(f, "Error calculating convex hull: {e}"),
        }
    }
}

impl error::Error for CurveError {}

impl From<chull::ErrorKind> for CurveError {
    fn from(item: chull::ErrorKind) -> Self {
        Self::ConvexHullFail(item)
    }
}
