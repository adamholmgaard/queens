use crate::model::coordinate::Coordinate;
use crate::model::layout::Area;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum GameError {
    Row { row: usize },
    Column { col: usize },
    Area { area: Area },
    Diagonal { c1: Coordinate, c2: Coordinate }, // add coordinate struct
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::Row { row } => {
                write!(f, "Row {} has duplicates!!", row)
            }
            GameError::Column { col } => {
                write!(f, "Column {} has duplicates", col)
            }
            GameError::Area { area } => {
                write!(f, "Duplicates in area {}", area.get_color())
            }
            GameError::Diagonal { c1, c2 } => {
                write!(f, "Diagonal problem at {} and {}", c1, c2)
            }
        }
    }
}
