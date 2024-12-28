use crate::brush::Brush;
use core::fmt;
use std::fmt::{Display, Formatter};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QEntity {
    pub parameters: HashMap<String, String>,
    pub brushes: Vec<Brush>,
}

impl QEntity {
    pub(crate) fn bake(&self) -> impl Display + use<'_> {
        struct QEntityDisp<'a>(&'a QEntity);
        impl Display for QEntityDisp<'_> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                writeln!(f, "{{",)?;
                for (key, value) in self.0.parameters.iter() {
                    writeln!(f, "\"{key}\" \"{value}\"")?;
                }
                for (i, brush) in self.0.brushes.iter().enumerate() {
                    writeln!(f, "// brush {i}")?;
                    write!(f, "{}", brush.bake())?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
        }
        QEntityDisp(self)
    }
}

#[derive(Debug, Clone)]
pub struct QMap {
    pub entities: Vec<QEntity>,
    pub metadata: Vec<String>,
}

impl QMap {
    pub fn new(entities: Vec<QEntity>) -> Self {
        Self {
            entities,
            metadata: Vec::new(),
        }
    }

    pub fn with_metadata(mut self, metadata: String) -> Self {
        self.metadata.push(metadata);
        self
    }

    pub fn with_tb_neverball_metadata(self) -> Self {
        self.with_metadata("Game: Neverball".to_string())
            .with_metadata("Format: Quake3".to_string())
    }

    // When you have a QMap, call this to get something you can write to a file.
    pub fn to_string(&self) -> String {
        String::from(self)
    }

    pub fn bake(&self) -> impl Display + use<'_> {
        struct QMapDisp<'a>(&'a QMap);
        impl Display for QMapDisp<'_> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                for metadata_line in self.0.metadata.iter().flat_map(|meta| meta.lines()) {
                    writeln!(f, "// {}", metadata_line)?;
                }
                for (i, entity) in self.0.entities.iter().enumerate() {
                    writeln!(f, "// entity {i}")?;
                    write!(f, "{}", entity.bake())?;
                }
                Ok(())
            }
        }
        QMapDisp(self)
    }
}

impl From<&QMap> for String {
    fn from(map: &QMap) -> String {
        format!("{}", map.bake())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::SimpleWorldspawn;
    use glam::DVec3;

    #[test]
    fn compile_map() {
        let vertices = vec![
            DVec3::from([0.0, 0.0, 0.0]),
            DVec3::from([0.0, 0.0, 1.0]),
            DVec3::from([0.0, 1.0, 0.0]),
            DVec3::from([1.0, 0.0, 0.0]),
            DVec3::from([0.3, 0.3, 0.3]),
        ];

        let brush1 = Brush::try_from_vertices(&vertices, Some(1000)).unwrap();
        let brush2 = Brush::try_from_vertices(&vertices, Some(1000)).unwrap();

        let worldspawn = SimpleWorldspawn::new(vec![brush1, brush2]);
        let entity: QEntity = worldspawn.into();
        let map: QMap = QMap::new(vec![entity]).with_tb_neverball_metadata();

        println!("{}", String::from(&map));

        let should_eq_str = r#"// Game: Neverball
// Format: Quake3
// entity 0
{
"classname" "worldspawn"
// brush 0
{
( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 0.000000 0.000000 0.000000 ) ( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
}
// brush 1
{
( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 0.000000 0.000000 0.000000 ) ( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 1.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
( 1.000000 0.000000 0.000000 ) ( 0.000000 0.000000 0.000000 ) ( 0.000000 1.000000 0.000000 ) mtrl/invisible 0 0 0 0.5 0.5 0
}
}
"#;
        println!("{}", should_eq_str);
        assert_eq!(format!("{}", String::from(&map)), should_eq_str);
    }
}
