use crate::model::layout::Area;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum GameRuleBroken {
    Row { row: usize },
    Column { col: usize },
    Area { area: Area },
    Diagonal { c1: usize, c2: usize }, // add coordinate struct
}

impl Display for GameRuleBroken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameRuleBroken::Row { row } => {
                write!(f, "Row {} has duplicates!!", row)
            }
            GameRuleBroken::Column { col } => {
                write!(f, "Column {} has duplicates", col)
            }
            GameRuleBroken::Area { area } => {
                write!(f, "Duplicates in area {}", area.get_color())
            }
            GameRuleBroken::Diagonal { c1, c2 } => {
                write!(f, "Diagonal problem at {} and {}", c1, c2)
            }
        }
    }
}
