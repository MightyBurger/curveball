// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

const TEX_DEFAULT: &str = "mtrl/invisible";
const ALMOST_EQUAL_DELTA: f64 = 0.000000001;
use core::fmt;
use glam::DVec3;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SideGeom(pub [DVec3; 3]);
impl SideGeom {
    pub fn normal(self) -> Option<DVec3> {
        let Self([p0, p1, p2]) = self;
        ((p0 - p1).cross(p2 - p1)).try_normalize()
    }
    pub fn dist(self) -> Option<f64> {
        let Self([_p0, p1, _p2]) = self;
        Some(self.normal()?.dot(p1))
    }
    pub fn equivalent(self, other: SideGeom) -> bool {
        let Some(normal) = self.normal() else {
            return false;
        };
        let Some(other_normal) = other.normal() else {
            return false;
        };
        if normal.dot(other_normal) < 1.0 - ALMOST_EQUAL_DELTA {
            return false;
        }

        let Some(dist) = self.dist() else {
            return false;
        };
        let Some(other_dist) = other.dist() else {
            return false;
        };
        if (dist - other_dist) > ALMOST_EQUAL_DELTA {
            return false;
        }
        true
    }
}

// TODO: Add texture offset, scale, rotation
#[derive(Debug, Clone, PartialEq)]
pub struct SideMtrl {
    pub texture: String,
}

impl Default for SideMtrl {
    fn default() -> Self {
        Self {
            texture: String::from(TEX_DEFAULT),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Side {
    pub geom: SideGeom,
    pub mtrl: SideMtrl,
}

impl Side {
    pub(crate) fn bake(&self) -> impl Display + use<'_> {
        struct SideDisplay<'a>(&'a Side);
        impl Display for SideDisplay<'_> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(
                    f,
                    "( {:.6} {:.6} {:.6} ) ( {:.6} {:.6} {:.6} ) ( {:.6} {:.6} {:.6} ) {} 0 0 0 0.5 0.5 0 0 0",
                    self.0.geom.0[0][0], self.0.geom.0[0][1], self.0.geom.0[0][2],
                    self.0.geom.0[1][0], self.0.geom.0[1][1], self.0.geom.0[1][2],
                    self.0.geom.0[2][0], self.0.geom.0[2][1], self.0.geom.0[2][2],
                    self.0.mtrl.texture
                )
            }
        }
        SideDisplay(self)
    }
}

use chull::ConvexHullWrapper;
#[derive(Debug, Clone)]
pub struct Brush {
    vertices: Vec<DVec3>,
    sides: Vec<([usize; 3], SideMtrl)>, // the [usize; 3] contains indices into the vertices vector
}

impl Brush {
    pub fn try_from_vertices(
        vertices: &[DVec3],
        max_iter: Option<usize>,
    ) -> Result<Self, chull::convex::ErrorKind> {
        let vertices: Vec<Vec<f64>> = vertices
            .iter()
            .map(|vertex| vec![vertex.x, vertex.y, vertex.z])
            .collect();

        let hull = ConvexHullWrapper::try_new(&vertices, max_iter)?;

        Ok(hull.into())
    }

    pub fn side_vertex_indices(&self) -> (&Vec<DVec3>, &Vec<([usize; 3], SideMtrl)>) {
        (&self.vertices, &self.sides)
    }

    pub fn triangles(&self) -> impl Iterator<Item = Side> + use<'_> {
        self.sides.iter().map(|([idx0, idx1, idx2], mtrl)| Side {
            geom: SideGeom([
                self.vertices[*idx0],
                self.vertices[*idx1],
                self.vertices[*idx2],
            ]),
            mtrl: mtrl.clone(),
        })
    }

    pub fn to_sides_unique(&self) -> Vec<Side> {
        let mut result: Vec<_> = self.triangles().collect();

        let keep: Vec<_> = result
            .iter()
            .enumerate()
            .map(|(i, candidate)| {
                !result[0..i]
                    .iter()
                    .any(|so_far| SideGeom::equivalent(so_far.geom, candidate.geom))
            })
            .collect();

        let mut keep_iter = keep.iter();

        // SideGeom::equivalent(*a, *b)
        result.retain(|_| *keep_iter.next().unwrap());
        result
    }

    pub fn vertices(&self) -> &Vec<DVec3> {
        &self.vertices
    }
}

#[allow(clippy::get_first)]
impl From<ConvexHullWrapper<f64>> for Brush {
    fn from(hull: ConvexHullWrapper<f64>) -> Self {
        let (vertices, side_indices) = hull.vertices_indices();

        let vertices: Vec<DVec3> = vertices
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
        assert_eq!(side_indices.len() % 3, 0);

        use itertools::Itertools;
        let sides = side_indices
            .into_iter()
            .tuples()
            .map(|(i0, i1, i2)| (i1, i0, i2)) // Reorder to (p1-p0; p2-p0) order
            .map(|(i0, i1, i2)| ([i0, i1, i2], SideMtrl::default()))
            .collect();

        Self { vertices, sides }
    }
}

impl Brush {
    pub(crate) fn bake(&self) -> impl Display + use<'_> {
        struct BrushDisp<'a>(&'a Brush);
        impl Display for BrushDisp<'_> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                writeln!(f, "{{",)?;
                for side in self.0.to_sides_unique().iter() {
                    writeln!(f, "{}", side.bake())?;
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

    fn almost_equals(a: &DVec3, b: &DVec3) -> bool {
        const EPSILON: f64 = 0.000000001;
        (a.x - b.x).abs() < EPSILON && (a.y - b.y).abs() < EPSILON && (a.z - b.z).abs() < EPSILON
    }

    #[test]
    fn test_brush_cube() {
        let vertices = vec![
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

        let brush = Brush::try_from_vertices(&vertices, Some(1000)).unwrap();

        let extracted_vertices: &Vec<DVec3> = brush.vertices();
        let extracted_sides: Vec<Side> = brush.to_sides_unique();

        assert_eq!(extracted_vertices.len(), 8);
        assert_eq!(extracted_sides.len(), 6);
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([0.0, 0.0, 0.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([0.0, 0.0, 1.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([0.0, 1.0, 0.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([0.0, 1.0, 1.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([1.0, 0.0, 0.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([1.0, 0.0, 1.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([1.0, 1.0, 0.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([1.0, 1.0, 1.0]))));
    }

    #[test]
    fn test_brush_pyramid() {
        let vertices = vec![
            DVec3::from([0.0, 0.0, 0.0]),
            DVec3::from([0.0, 0.0, 1.0]),
            DVec3::from([0.0, 1.0, 0.0]),
            DVec3::from([1.0, 0.0, 0.0]),
            DVec3::from([0.3, 0.3, 0.3]),
        ];

        let brush = Brush::try_from_vertices(&vertices, Some(1000)).unwrap();

        let extracted_vertices: &Vec<DVec3> = brush.vertices();
        let extracted_sides: Vec<Side> = brush.to_sides_unique();

        assert_eq!(extracted_vertices.len(), 4);
        assert_eq!(extracted_sides.len(), 4);

        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([0.0, 0.0, 0.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([0.0, 0.0, 1.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([0.0, 1.0, 0.0]))));
        assert!(extracted_vertices
            .iter()
            .any(|vertex| almost_equals(vertex, &DVec3::from([1.0, 0.0, 0.0]))));
    }

    #[test]
    fn bake_side() {
        let testvertex1 = DVec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let testvertex2 = DVec3 {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        };

        let testvertex3 = DVec3 {
            x: 100.0,
            y: 200.0,
            z: 300.0,
        };

        let side = Side {
            geom: SideGeom([testvertex1, testvertex2, testvertex3]),
            mtrl: SideMtrl::default(),
        };

        assert_eq!(format!("{}", side.bake()), "( 1.000000 2.000000 3.000000 ) ( 10.000000 20.000000 30.000000 ) ( 100.000000 200.000000 300.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0 0 0");
    }

    #[test]
    fn bake_brush_pyramid() {
        let vertices = vec![
            DVec3::from([0.0, 0.0, 0.0]),
            DVec3::from([0.0, 0.0, 1.0]),
            DVec3::from([0.0, 1.0, 0.0]),
            DVec3::from([1.0, 0.0, 0.0]),
            DVec3::from([0.3, 0.3, 0.3]),
        ];

        let brush = Brush::try_from_vertices(&vertices, Some(1000)).unwrap();

        let should_eq_str = r"{
( 0.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0 0 0
( 0.000000 1.000000 0.000000 ) ( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0 0 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0 0 0
( 0.000000 0.000000 0.000000 ) ( 1.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0 0 0
}
";
        println!("{}", brush.bake());
        println!("{}", should_eq_str);
        assert_eq!(format!("{}", brush.bake()), should_eq_str);
    }
}
