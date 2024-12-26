const TEX_DEFAULT: &str = "mtrl/invisible";
use core::fmt;
use glam::DVec3;
use std::fmt::{Display, Formatter};

// impl Point {
//     pub(crate) fn bake(&self) -> impl Display {
//         struct PointDisp(Point);
//         impl Display for PointDisp {
//             fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//                 write!(f, "{:.6} {:.6} {:.6}", self.0.x, self.0.y, self.0.z)
//             }
//         }
//         PointDisp(*self)
//     }
// }

// Not Copy to keep adding a texture in the realm of possibilities
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Face {
    pub points: [DVec3; 3],
    pub texture: String,
}

impl Face {
    pub(crate) fn bake(&self) -> impl Display + use<'_> {
        struct FaceDisplay<'a>(&'a Face);
        impl Display for FaceDisplay<'_> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(
                    f,
                    "( {:.6} {:.6} {:.6} ) ( {:.6} {:.6} {:.6} ) ( {:.6} {:.6} {:.6} ) {} 0 0 0 0.5 0.5 0",
                    self.0.points[0][0],self.0.points[0][1], self.0.points[0][2], 
                    self.0.points[1][0],self.0.points[1][1], self.0.points[1][2], 
                    self.0.points[2][0],self.0.points[2][1], self.0.points[2][2], 
                    self.0.texture
                )
            }
        }
        FaceDisplay(self)
    }
    pub fn normal(&self) -> DVec3 {
        todo!()
    }
}

// #[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
// pub struct FaceNormalOrig {
//     pub normal: [f64; 3], // normal vector of the plane
//     pub dist: [f64; 3],   // minimum distance to the origin
//     pub texture: String,
// }

use chull::ConvexHullWrapper;
#[derive(Debug, Clone)]
pub struct Brush {
    points: Vec<DVec3>,
    faces: Vec<([usize; 3], String)>, // the [usize; 3] contains indices into the points vector
}

impl Brush {
    pub fn try_from_points<'a>(
        points: impl IntoIterator<Item = &'a DVec3>,
        max_iter: Option<usize>,
    ) -> Result<Self, chull::convex::ErrorKind> {
        let points: Vec<Vec<f64>> = points
            .into_iter()
            .map(|point| vec![point.x, point.y, point.z])
            .collect();

        let hull = ConvexHullWrapper::try_new(&points, max_iter)?;

        Ok(hull.into())
    }

    pub fn face_point_indices(&self) -> (&Vec<DVec3>, &Vec<([usize; 3], String)>) {
        (&self.points, &self.faces)
    }

    pub fn to_faces_iter(&self) -> impl Iterator<Item = Face> + use<'_> {
        self.faces.iter().map(|([idx0, idx1, idx2], tex)| Face {
            points: [self.points[*idx0], self.points[*idx1], self.points[*idx2]],
            texture: tex.clone(),
        })
    }

    pub fn points(&self) -> &Vec<DVec3> {
        &self.points
    }
    pub fn points_iter(&self) -> impl Iterator<Item = &DVec3> + use<'_> {
        self.points.iter()
    }
}

fn faces_duplicate(face1: [DVec3; 3], face2: [DVec3; 3]) -> bool {
    let normal1 = (face1[1] - face1[0]).cross(face1[2] - face1[0]);
    let normal2 = (face2[1] - face2[0]).cross(face2[2] - face2[0]);
    if normal1.dot(normal2) == 1.0 {return true;}
    false
}

impl From<ConvexHullWrapper<f64>> for Brush {
    fn from(hull: ConvexHullWrapper<f64>) -> Self {
        let (points, face_indices) = hull.vertices_indices();

        let points: Vec<DVec3> = points
            .into_iter()
            .map(|vertex| DVec3 {
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
        #[cfg(debug_assertions)]
        assert_eq!(face_indices.len() % 3, 0);

        // TODO: Redo this really gross block of code.
        // Its purpose is to add face_indices, but only the ones that are
        // not duplicates.
        use itertools::Itertools;

        let face_indices2: Vec<(usize, usize, usize)> = face_indices
            .iter()
            .tuples()
            .fold(Vec::new(), |faces, (i1, i2, i3)|  {
                let mut unique = true;
                for (s1, s2, s3) in faces.iter() {
                    let face1 = [points[*i1], points[*i2], points[*i3]];
                    let face2 = [points[*s1], points[*s2], points[*s3]];
                    if faces_duplicate(face1, face2) {
                        unique = false;
                    }
                }
                if unique {
                    let mut next = faces.clone();
                    next.push((*i1, *i2, *i3));
                    next
                }
                else {
                    faces
                }
            });

        let faces = face_indices2.into_iter().map(|(i1, i2, i3)| ([i1, i2, i3], String::from(TEX_DEFAULT))).collect();


        Self { points, faces }
    }
}

impl Brush {
    pub(crate) fn bake(&self) -> impl Display + use<'_> {
        struct BrushDisp<'a>(&'a Brush);
        impl Display for BrushDisp<'_> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                writeln!(f, "{{",)?;
                for face in self.0.to_faces_iter() {
                    writeln!(f, "{}", face.bake())?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
        }
        BrushDisp(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.000000001;
    fn almost_equals(a: &DVec3, b: &DVec3) -> bool {
        (a.x - b.x).abs() < EPSILON && (a.y - b.y).abs() < EPSILON && (a.z - b.z).abs() < EPSILON
    }

    #[test]
    fn test_brush_cube() {
        let points = vec![
            DVec3::from([0.0, 0.0, 0.0]),
            DVec3::from([0.0, 0.0, 1.0]),
            DVec3::from([0.0, 1.0, 0.0]),
            DVec3::from([0.0, 1.0, 1.0]),
            DVec3::from([1.0, 0.0, 0.0]),
            DVec3::from([1.0, 0.0, 1.0]),
            DVec3::from([1.0, 1.0, 0.0]),
            DVec3::from([1.0, 1.0, 1.0]),
            DVec3::from([0.5, 0.5, 0.5]),
        ];

        let brush = Brush::try_from_points(&points, Some(1000)).unwrap();

        let extracted_points: &Vec<DVec3> = brush.points();
        let extracted_faces: Vec<Face> = brush.to_faces_iter().collect();

        assert_eq!(extracted_points.len(), 8);
        assert_eq!(extracted_faces.len(), 6);
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([0.0, 0.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([0.0, 0.0, 1.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([0.0, 1.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([0.0, 1.0, 1.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([1.0, 0.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([1.0, 0.0, 1.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([1.0, 1.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([1.0, 1.0, 1.0]))));
    }

    #[test]
    fn test_brush_pyramid() {
        let points = vec![
            DVec3::from([0.0, 0.0, 0.0]),
            DVec3::from([0.0, 0.0, 1.0]),
            DVec3::from([0.0, 1.0, 0.0]),
            DVec3::from([1.0, 0.0, 0.0]),
            DVec3::from([0.3, 0.3, 0.3]),
        ];

        let brush = Brush::try_from_points(&points, Some(1000)).unwrap();

        let extracted_points: &Vec<DVec3> = brush.points();
        let extracted_faces: Vec<Face> = brush.to_faces_iter().collect();

        assert_eq!(extracted_points.len(), 4);
        assert_eq!(extracted_faces.len(), 4);

        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([0.0, 0.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([0.0, 0.0, 1.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([0.0, 1.0, 0.0]))));
        assert!(extracted_points
            .iter()
            .any(|point| almost_equals(point, &DVec3::from([1.0, 0.0, 0.0]))));
    }

    #[test]
    fn bake_face() {
        let testpoint1 = DVec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let testpoint2 = DVec3 {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        };

        let testpoint3 = DVec3 {
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
    fn bake_brush() {
        let points = vec![
            DVec3::from([0.0, 0.0, 0.0]),
            DVec3::from([0.0, 0.0, 1.0]),
            DVec3::from([0.0, 1.0, 0.0]),
            DVec3::from([1.0, 0.0, 0.0]),
            DVec3::from([0.3, 0.3, 0.3]),
        ];

        let brush = Brush::try_from_points(&points, Some(1000)).unwrap();

        let should_eq_str = r"{
( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 0.000000 0.000000 0.000000 ) ( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
}
";
        assert_eq!(format!("{}", brush.bake()), should_eq_str);
    }
}
