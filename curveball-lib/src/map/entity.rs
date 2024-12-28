use crate::map::geometry::Brush;
use crate::map::qmap::QEntity;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SimpleWorldspawn {
    pub brushes: Vec<Brush>,
}

impl SimpleWorldspawn {
    pub fn new(brushes: Vec<Brush>) -> Self {
        Self { brushes }
    }
}

impl From<SimpleWorldspawn> for QEntity {
    fn from(item: SimpleWorldspawn) -> Self {
        let mut parameters = HashMap::new();
        parameters.insert(String::from("classname"), String::from("worldspawn"));
        Self {
            parameters,
            brushes: item.brushes,
        }
    }
}

// TODO: Add other Neverball entities
// TODO: Add TrenchbroomGroup entity
//
#[cfg(test)]
mod tests {}
