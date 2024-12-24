#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<[f64; 3]> for Point {
    fn from(value: [f64; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

// Not Copy to keep adding a texture in the realm of possibilities
#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct Side(pub [Point; 3]);

use chull::ConvexHullWrapper;

#[derive(Debug, Clone)]
pub struct Lump(pub ConvexHullWrapper<f64>);

impl Lump {
    pub fn try_from_points(
        points: impl IntoIterator<Item = Point>,
        max_iter: Option<usize>,
    ) -> Result<Self, chull::convex::ErrorKind> {
        let points: Vec<Vec<f64>> = points
            .into_iter()
            .map(|point| vec![point.x, point.y, point.z])
            .collect();

        let hull = ConvexHullWrapper::try_new(&points, max_iter)?;
        Ok(Self(hull))
    }

    pub fn sides(&self) -> impl Iterator<Item = Side> {
        let (vertices, indices) = self.0.vertices_indices();

        use itertools::Itertools;
        indices.into_iter().tuples().map(move |(idx1, idx2, idx3)| {
            Side([
                Point {
                    x: vertices[idx1][0],
                    y: vertices[idx1][1],
                    z: vertices[idx1][2],
                },
                Point {
                    x: vertices[idx2][0],
                    y: vertices[idx2][1],
                    z: vertices[idx2][2],
                },
                Point {
                    x: vertices[idx3][0],
                    y: vertices[idx3][1],
                    z: vertices[idx3][2],
                },
            ])
        })
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        self.sides().flat_map(|side| side.0.into_iter())
    }
}

impl From<ConvexHullWrapper<f64>> for Lump {
    fn from(value: ConvexHullWrapper<f64>) -> Self {
        Self(value)
    }
}

impl Into<ConvexHullWrapper<f64>> for Lump {
    fn into(self) -> ConvexHullWrapper<f64> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
