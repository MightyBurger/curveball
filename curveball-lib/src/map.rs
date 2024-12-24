use crate::lump::Lump;
use core::fmt;
use std::fmt::Formatter;

use std::fmt::Display;
pub trait MapElement {
    fn bake(self) -> impl Display;
}

pub struct Map {
    geometry: Vec<Lump>,
}

impl Map {
    pub fn from_lumps(lumps: impl IntoIterator<Item = Lump>) -> Self {
        Self {
            geometry: lumps.into_iter().collect(),
        }
    }
}

impl MapElement for Map {
    fn bake(self) -> impl Display {
        struct MapDisp(Map);
        impl Display for MapDisp {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                writeln!(f, "{{")?;
                for lump in self.0.geometry.iter() {
                    write!(f, "{}", lump.clone().bake())?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
        }
        MapDisp(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lump::{Lump, Point};

    #[test]
    fn bake_map() {
        let points = vec![
            Point::from([0.0, 0.0, 0.0]),
            Point::from([0.0, 0.0, 1.0]),
            Point::from([0.0, 1.0, 0.0]),
            Point::from([1.0, 0.0, 0.0]),
            Point::from([0.3, 0.3, 0.3]),
        ];

        let lump1 = Lump::try_from_points(&points, Some(1000)).unwrap();
        let lump2 = lump1.clone();

        let map = Map::from_lumps([lump1, lump2]);

        let should_eq_str = r"{
{
( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 0.000000 0.000000 0.000000 ) ( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
}
{
( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 0.000000 0.000000 0.000000 ) ( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
}
}
";
        assert_eq!(format!("{}", map.bake()), should_eq_str);
    }
}
