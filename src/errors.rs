use std::borrow::Cow;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

// Error that cannot be resolved by game logic, and is thus handled by restarting the game.
#[derive(Debug, Copy, Clone)]
pub enum QueensError {
    AreaNotFound { c: usize },
    OutOfBounds { c: usize },
    Invalid2DCoordinate { column: usize, row: usize, n: usize },
    RefreshRequested,
    
}

impl Display for QueensError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            QueensError::OutOfBounds { c } => {
                write!(f, "Coordinate {} out of bounds", c)
            }
            QueensError::Invalid2DCoordinate { column, row, n } => {
                write!(
                    f,
                    "2D coordinate ({}, {}) is invalid for {} by {} grid",
                    column, row, n, n
                )
            }
            QueensError::AreaNotFound { c } => {
                write!(f, "Area not found at coordinate {}", c)
            }
            QueensError::RefreshRequested => {
                write!(f, "Refresh request was requested")
            }
        }
    }
}

impl Into<Cow<'static, str>> for QueensError {
    fn into(self) -> Cow<'static, str> {
        Cow::from(format!("{}", self))
    }
}

pub type QueensResult<T> = Result<T, QueensError>;
