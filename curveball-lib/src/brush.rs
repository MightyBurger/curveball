const TEX_DEFAULT: &str = "mtrl/invisible";
use core::fmt;
use glam::DVec3;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SideGeom(pub [DVec3; 3]);
impl SideGeom {
    pub fn normal(self) -> DVec3 {
        let Self([p0, p1, p2]) = self;
        ((p0 - p1).cross(p2 - p1)).normalize()
    }
    pub fn dist(self) -> f64 {
        let Self([_p0, p1, _p2]) = self;
        self.normal().dot(p1)
    }
    pub fn equivalent(self, other: SideGeom) -> bool {
        if self.normal().dot(other.normal()) < 1.0 {
            return false;
        }
        if self.dist() != other.dist() {
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
                    "( {:.6} {:.6} {:.6} ) ( {:.6} {:.6} {:.6} ) ( {:.6} {:.6} {:.6} ) {} 0 0 0 0.5 0.5 0",
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
    pub fn try_from_vertices<'a>(
        vertices: impl IntoIterator<Item = &'a DVec3>,
        max_iter: Option<usize>,
    ) -> Result<Self, chull::convex::ErrorKind> {
        let vertices: Vec<Vec<f64>> = vertices
            .into_iter()
            .map(|vertex| vec![vertex.x, vertex.y, vertex.z])
            .collect();

        let hull = ConvexHullWrapper::try_new(&vertices, max_iter)?;

        Ok(hull.into())
    }

    pub fn side_vertex_indices(&self) -> (&Vec<DVec3>, &Vec<([usize; 3], SideMtrl)>) {
        (&self.vertices, &self.sides)
    }

    pub fn to_sides_iter(&self) -> impl Iterator<Item = Side> + use<'_> {
        self.sides.iter().map(|([idx0, idx1, idx2], mtrl)| Side {
            geom: SideGeom([
                self.vertices[*idx0],
                self.vertices[*idx1],
                self.vertices[*idx2],
            ]),
            mtrl: mtrl.clone(),
        })
    }

    pub fn into_sides_iter(self) -> impl Iterator<Item = Side> {
        self.sides
            .into_iter()
            .map(move |([idx0, idx1, idx2], mtrl)| Side {
                geom: SideGeom([
                    self.vertices[idx0],
                    self.vertices[idx1],
                    self.vertices[idx2],
                ]),
                mtrl,
            })
    }

    pub fn vertices(&self) -> &Vec<DVec3> {
        &self.vertices
    }
    pub fn vertices_iter(&self) -> impl Iterator<Item = &DVec3> + use<'_> {
        self.vertices.iter()
    }
}

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

        // TODO: This block of code is kind of gross, consider rewriting.
        // Its purpose is: to group three side indices at a time from the result
        // of hull.vertices_indices();, to remove duplicate sides, and to attach
        // a texture of TEX_DEFAULT.

        use itertools::Itertools;

        let side_indices2: Vec<(usize, usize, usize)> =
            side_indices
                .iter()
                .tuples()
                .fold(Vec::new(), |sides, (i1, i2, i3)| {
                    let mut unique = true;
                    for (s1, s2, s3) in sides.iter() {
                        let side1 = SideGeom([vertices[*i1], vertices[*i2], vertices[*i3]]);
                        let side2 = SideGeom([vertices[*s1], vertices[*s2], vertices[*s3]]);
                        if SideGeom::equivalent(side1, side2) {
                            unique = false;
                        }
                    }
                    if unique {
                        let mut next = sides.clone();
                        next.push((*i1, *i2, *i3));
                        next
                    } else {
                        sides
                    }
                });

        let sides = side_indices2
            .into_iter()
            .map(|(i1, i2, i3)| ([i1, i2, i3], SideMtrl::default()))
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
                for side in self.0.to_sides_iter() {
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
        let extracted_sides: Vec<Side> = brush.to_sides_iter().collect();

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
        let extracted_sides: Vec<Side> = brush.to_sides_iter().collect();

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

        assert_eq!(format!("{}", side.bake()), "( 1.000000 2.000000 3.000000 ) ( 10.000000 20.000000 30.000000 ) ( 100.000000 200.000000 300.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0");
    }
    #[test]
    fn bake_brush() {
        let vertices = vec![
            DVec3::from([0.0, 0.0, 0.0]),
            DVec3::from([0.0, 0.0, 1.0]),
            DVec3::from([0.0, 1.0, 0.0]),
            DVec3::from([1.0, 0.0, 0.0]),
            DVec3::from([0.3, 0.3, 0.3]),
        ];

        let brush = Brush::try_from_vertices(&vertices, Some(1000)).unwrap();

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
