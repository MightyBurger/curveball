const TEX_DEFAULT: &str = "mtrl/invisible";
use crate::map::MapElement;
use core::fmt;
use std::fmt::{Display, Formatter};

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

impl MapElement for Point {
    fn bake(self) -> impl Display {
        struct PointDisp(Point);
        impl Display for PointDisp {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{:.6} {:.6} {:.6}", self.0.x, self.0.y, self.0.z)
            }
        }
        PointDisp(self)
    }
}

// Not Copy to keep adding a texture in the realm of possibilities
#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct Face {
    pub points: [Point; 3],
    pub texture: String,
}

impl MapElement for Face {
    fn bake(self) -> impl Display {
        struct FaceDisp(Face);
        impl Display for FaceDisp {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(
                    f,
                    "( {} ) ( {} ) ( {} ) {} 0 0 0 0.5 0.5 0",
                    self.0.points[0].bake(),
                    self.0.points[1].bake(),
                    self.0.points[2].bake(),
                    self.0.texture
                )
            }
        }
        FaceDisp(self)
    }
}

use chull::ConvexHullWrapper;

#[derive(Debug, Clone)]
pub struct Lump {
    points: Vec<Point>,
    faces: Vec<([usize; 3], String)>, // the [usize; 3] contains indices into the points vector
}

impl Lump {
    pub fn try_from_points<'a>(
        points: impl IntoIterator<Item = &'a Point>,
        max_iter: Option<usize>,
    ) -> Result<Self, chull::convex::ErrorKind> {
        let points: Vec<Vec<f64>> = points
            .into_iter()
            .map(|point| vec![point.x, point.y, point.z])
            .collect();

        let hull = ConvexHullWrapper::try_new(&points, max_iter)?;

        Ok(hull.into())
    }

    pub fn to_faces(&self) -> impl Iterator<Item = Face> + use<'_> {
        self.faces.iter().map(|([idx0, idx1, idx2], tex)| Face {
            points: [self.points[*idx0], self.points[*idx1], self.points[*idx2]],
            texture: tex.clone(),
        })
    }

    pub fn points(&self) -> impl Iterator<Item = &Point> + use<'_> {
        self.points.iter()
    }
}

impl From<ConvexHullWrapper<f64>> for Lump {
    fn from(hull: ConvexHullWrapper<f64>) -> Self {
        let (points, face_indices) = hull.vertices_indices();

        let points = points
            .into_iter()
            .map(|vertex| Point {
                x: *vertex
                    .get(0)
                    .expect("vertices expected to have three components"),
                y: *vertex
                    .get(1)
                    .expect("vertices expected to have three components"),
                z: *vertex
                    .get(2)
                    .expect("vertices expected to have three components"),
            })
            .collect();

        // Just a check.
        assert_eq!(face_indices.len() % 3, 0);

        // When ArrayChunks is stabalized, switch to using that over tuples() and eliminate
        // the itertools dependancy.
        use itertools::Itertools;
        let faces: Vec<([usize; 3], String)> = face_indices
            .into_iter()
            .tuples()
            .map(|(idx1, idx2, idx3)| ([idx1, idx2, idx3], String::from(TEX_DEFAULT)))
            .collect();

        // TODO: Eliminate equivalent faces. It appears the library I use will split each
        // lump into triangles, producing excessive faces, though the geometry will
        // be the same.
        // This is an important step, as currently, lumps will have twice the number
        // of faces as required.
        // Test every face against every other face, and delete when two faces match.

        Self { points, faces }
    }
}

impl MapElement for Lump {
    fn bake(self) -> impl Display {
        struct LumpDisp(Lump);
        impl Display for LumpDisp {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                writeln!(f, "{{",)?;
                for face in self.0.to_faces() {
                    writeln!(f, "{}", face.bake())?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
        }
        LumpDisp(self)
    }
}

// -----------------------------------
//             Unit Tests
// -----------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.000000001;
    fn almost_equals(a: &Point, b: &Point) -> bool {
        (a.x - b.x).abs() < EPSILON && (a.y - b.y).abs() < EPSILON && (a.z - b.z).abs() < EPSILON
    }

    #[test]
    fn test_lump_cube() {
        let points = vec![
            Point::from([0.0, 0.0, 0.0]),
            Point::from([0.0, 0.0, 1.0]),
            Point::from([0.0, 1.0, 0.0]),
            Point::from([0.0, 1.0, 1.0]),
            Point::from([1.0, 0.0, 0.0]),
            Point::from([1.0, 0.0, 1.0]),
            Point::from([1.0, 1.0, 0.0]),
            Point::from([1.0, 1.0, 1.0]),
            Point::from([0.5, 0.5, 0.5]),
        ];

        let lump = Lump::try_from_points(&points, Some(1000)).unwrap();

        let extracted_points: Vec<&Point> = lump.points().collect();
        let extracted_faces: Vec<Face> = lump.to_faces().collect();

        assert_eq!(extracted_points.len(), 8);
        assert_eq!(extracted_faces.len(), 12); // TODO: Change to 6 when the above TODO is resolved.
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([0.0, 0.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([0.0, 0.0, 1.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([0.0, 1.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([0.0, 1.0, 1.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([1.0, 0.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([1.0, 0.0, 1.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([1.0, 1.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([1.0, 1.0, 1.0]))));
    }

    #[test]
    fn test_lump_pyramid() {
        let points = vec![
            Point::from([0.0, 0.0, 0.0]),
            Point::from([0.0, 0.0, 1.0]),
            Point::from([0.0, 1.0, 0.0]),
            Point::from([1.0, 0.0, 0.0]),
            Point::from([0.3, 0.3, 0.3]),
        ];

        let lump = Lump::try_from_points(&points, Some(1000)).unwrap();

        let extracted_points: Vec<&Point> = lump.points().collect();
        let extracted_faces: Vec<Face> = lump.to_faces().collect();

        assert_eq!(extracted_points.len(), 4);
        assert_eq!(extracted_faces.len(), 4);

        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([0.0, 0.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([0.0, 0.0, 1.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([0.0, 1.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &Point::from([1.0, 0.0, 0.0]))));
    }

    #[test]
    fn bake_point() {
        let testpoint = Point {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        };
        assert_eq!(
            format!("{}", testpoint.bake()),
            "10.000000 20.000000 30.000000"
        );
    }

    #[test]
    fn bake_face() {
        let testpoint1 = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let testpoint2 = Point {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        };

        let testpoint3 = Point {
            x: 100.0,
            y: 200.0,
            z: 300.0,
        };

        let face = Face {
            points: [testpoint1, testpoint2, testpoint3],
            texture: String::from("mtrl/invisible"),
        };

        assert_eq!(format!("{}", face.bake()), "( 1.000000 2.000000 3.000000 ) ( 10.000000 20.000000 30.000000 ) ( 100.000000 200.000000 300.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0");
    }
    #[test]
    fn bake_lump() {
        let points = vec![
            Point::from([0.0, 0.0, 0.0]),
            Point::from([0.0, 0.0, 1.0]),
            Point::from([0.0, 1.0, 0.0]),
            Point::from([1.0, 0.0, 0.0]),
            Point::from([0.3, 0.3, 0.3]),
        ];

        let lump = Lump::try_from_points(&points, Some(1000)).unwrap();

        let should_eq_str = r"{
( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 0.000000 0.000000 0.000000 ) ( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
}
";
        assert_eq!(format!("{}", lump.bake()), should_eq_str);
    }
}
