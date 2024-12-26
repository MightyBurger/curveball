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
                    writeln!(f, "{key}, {value}")?;
                }
                for brush in self.0.brushes.iter() {
                    writeln!(f, "{}", brush.bake())?;
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
}

impl QMap {
    // When you have a QMap, call this to get something you can write to a file.
    pub(crate) fn bake(&self) -> impl Display + use<'_> {
        struct QMapDisp<'a>(&'a QMap);
        impl Display for QMapDisp<'_> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                writeln!(f, "{{",)?;
                for entity in self.0.entities.iter() {
                    writeln!(f, "{}", entity.bake())?;
                }
                writeln!(f, "}}")?;
                Ok(())
            }
        }
        QMapDisp(self)
    }
}

impl From<QMap> for String {
    fn from(item: QMap) -> String {
        format!("{}", item.bake())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
}
